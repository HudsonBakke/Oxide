use windows::{
    Win32::Graphics::{
        Direct3D12::{
            D3D12_CLEAR_FLAG_DEPTH, D3D12_COMMAND_LIST_TYPE_DIRECT,
            D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING, D3D12_DESCRIPTOR_HEAP_DESC,
            D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE, D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV,
            D3D12_GPU_DESCRIPTOR_HANDLE, D3D12_RESOURCE_BARRIER, D3D12_RESOURCE_BARRIER_0,
            D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES, D3D12_RESOURCE_BARRIER_FLAG_NONE,
            D3D12_RESOURCE_BARRIER_TYPE_TRANSITION, D3D12_RESOURCE_STATE_PRESENT,
            D3D12_RESOURCE_STATE_RENDER_TARGET, D3D12_RESOURCE_STATES,
            D3D12_RESOURCE_TRANSITION_BARRIER, D3D12_SHADER_RESOURCE_VIEW_DESC,
            D3D12_SRV_DIMENSION_TEXTURE2D, D3D12_TEX2D_SRV, ID3D12CommandAllocator,
            ID3D12CommandList, ID3D12DescriptorHeap, ID3D12GraphicsCommandList, ID3D12Resource,
        },
        Dxgi::Common::DXGI_FORMAT_R8G8B8A8_UNORM,
    },
    Win32::{
        Foundation::{CloseHandle, HANDLE, RECT},
        Graphics::{
            Direct3D::D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST,
            Direct3D12::{D3D12_FENCE_FLAG_NONE, D3D12_VIEWPORT, ID3D12Fence},
        },
        System::Threading::{CreateEventW, INFINITE, WaitForSingleObject},
    },
    core::{Interface, Result},
};

use std::collections::HashMap;

use super::{
    D3D12ConstantBuffer, D3D12Device, D3D12IndexBuffer, D3D12Pipeline, D3D12Shader, D3D12SwapChain,
    D3D12Texture, D3D12VertexBuffer, swap_chain::FRAME_COUNT,
};

use crate::{
    graphics::{
        Camera, DirectionalLight, Environment, GraphicsResult, Material, Mesh, PointLight, SpotLight,
        mesh::MeshId, texture::TextureId,
    },
    math::{Rect, Transform},
};

const TRIANGLE_VERTEX_SHADER: &[u8] =
    include_bytes!("../../../../shaders/compiled/triangle_vs.dxil");

const TRIANGLE_PIXEL_SHADER: &[u8] =
    include_bytes!("../../../../shaders/compiled/triangle_ps.dxil");
const BACKGROUND_VERTEX_SHADER: &[u8] =
    include_bytes!("../../../../shaders/compiled/background_vs.dxil");
const BACKGROUND_PIXEL_SHADER: &[u8] =
    include_bytes!("../../../../shaders/compiled/background_ps.dxil");

#[derive(Debug, Clone, Copy)]
enum ActiveLight {
    Point(PointLight),
    Spot(SpotLight),
    Directional(DirectionalLight),
}

fn transition_resource(
    command_list: &ID3D12GraphicsCommandList,
    resource: &ID3D12Resource,
    before: D3D12_RESOURCE_STATES,
    after: D3D12_RESOURCE_STATES,
) {
    let mut barrier = D3D12_RESOURCE_BARRIER {
        Type: D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
        Flags: D3D12_RESOURCE_BARRIER_FLAG_NONE,
        Anonymous: D3D12_RESOURCE_BARRIER_0 {
            Transition: std::mem::ManuallyDrop::new(D3D12_RESOURCE_TRANSITION_BARRIER {
                pResource: std::mem::ManuallyDrop::new(Some(resource.clone())),
                Subresource: D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES,
                StateBefore: before,
                StateAfter: after,
            }),
        },
    };

    unsafe {
        command_list.ResourceBarrier(std::slice::from_ref(&barrier));

        std::mem::ManuallyDrop::drop(&mut (*barrier.Anonymous.Transition).pResource);
    }
}

const MAX_POINT_LIGHTS: usize = 8;
const MAX_SPOT_LIGHTS: usize = 8;
const MAX_DIRECTIONAL_LIGHTS: usize = 8;

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct GpuSpotLight {
    // xyz = position
    // w   = intensity
    position: [f32; 4],

    // xyz = normalized direction
    // w   = cosine of cone half-angle
    direction: [f32; 4],

    // rgb = color
    color: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct GpuDirectionalLight {
    // xyz = normalized direction
    direction: [f32; 4],

    // rgb = color
    // a   = intensity
    color: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
struct GpuPointLight {
    position: [f32; 4],
    color: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct LightConstants {
    point_lights: [GpuPointLight; MAX_POINT_LIGHTS],

    spot_lights: [GpuSpotLight; MAX_SPOT_LIGHTS],

    directional_lights: [GpuDirectionalLight; MAX_DIRECTIONAL_LIGHTS],

    point_light_count: u32,
    spot_light_count: u32,
    directional_light_count: u32,

    _padding: u32,

    environment_ambient: [f32; 4],
    environment_params: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ObjectConstants {
    model: [[f32; 4]; 4],
    normal: [[f32; 4]; 4],
    base_color: [f32; 4],
    material: [f32; 4],
    emissive: [f32; 4],
}


#[repr(C)]
#[derive(Clone, Copy)]
struct BackgroundConstants {
    camera_right: [f32; 4],
    camera_up: [f32; 4],
    camera_forward: [f32; 4],
    projection_params: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ViewConstants {
    view: [[f32; 4]; 4],
    projection: [[f32; 4]; 4],
    camera_position: [f32; 4],
}

struct CachedMesh {
    vertex_generation: u64,
    index_generation: u64,

    vertex_buffer: D3D12VertexBuffer,
    index_buffer: D3D12IndexBuffer,
}

impl CachedMesh {
    fn new(device: &D3D12Device, mesh: &Mesh) -> Result<Self> {
        Ok(Self {
            vertex_generation: mesh.vertex_generation(),
            index_generation: mesh.index_generation(),

            vertex_buffer: D3D12VertexBuffer::new(device, mesh.vertices())?,

            index_buffer: D3D12IndexBuffer::new(device, mesh.indices())?,
        })
    }

    fn synchronize(&mut self, device: &D3D12Device, mesh: &Mesh) -> Result<()> {
        if self.vertex_generation != mesh.vertex_generation() {
            self.vertex_buffer = D3D12VertexBuffer::new(device, mesh.vertices())?;

            self.vertex_generation = mesh.vertex_generation();
        }

        if self.index_generation != mesh.index_generation() {
            self.index_buffer = D3D12IndexBuffer::new(device, mesh.indices())?;

            self.index_generation = mesh.index_generation();
        }

        Ok(())
    }
}

const MAX_TEXTURES: u32 = 1024;

pub(crate) struct D3D12Renderer {
    command_allocators: [ID3D12CommandAllocator; FRAME_COUNT],
    command_list: ID3D12GraphicsCommandList,

    pipeline: D3D12Pipeline,
    background_pipeline: D3D12Pipeline,

    mesh_cache: HashMap<MeshId, CachedMesh>,

    fence: ID3D12Fence,
    fence_value: u64,
    fence_event: HANDLE,

    constant_buffer: D3D12ConstantBuffer,

    active_lights: Vec<ActiveLight>,
    environment: Option<Environment>,
    environment_descriptor_cache: HashMap<(Option<TextureId>, Option<TextureId>), u32>,
    irradiance_cache: HashMap<TextureId, D3D12Texture>,
    fallback_environment_texture: crate::graphics::Texture,
    texture_cache: HashMap<TextureId, D3D12Texture>,
    material_descriptor_cache: HashMap<(TextureId, TextureId), u32>,
    texture_heap: ID3D12DescriptorHeap,
    texture_descriptor_size: u32,
    next_texture_descriptor: u32,
    pending_background: Option<BackgroundConstants>,
    background_drawn: bool,
}

impl D3D12Renderer {
    pub(crate) fn new(device: &D3D12Device) -> GraphicsResult<Self> {
        let allocator_0: ID3D12CommandAllocator = unsafe {
            device
                .device()
                .CreateCommandAllocator(D3D12_COMMAND_LIST_TYPE_DIRECT)?
        };

        let allocator_1: ID3D12CommandAllocator = unsafe {
            device
                .device()
                .CreateCommandAllocator(D3D12_COMMAND_LIST_TYPE_DIRECT)?
        };

        let command_allocators = [allocator_0, allocator_1];

        let command_list: ID3D12GraphicsCommandList = unsafe {
            device.device().CreateCommandList(
                0,
                D3D12_COMMAND_LIST_TYPE_DIRECT,
                &command_allocators[0],
                None,
            )?
        };

        unsafe {
            command_list.Close()?;
        }

        let fence: ID3D12Fence = unsafe { device.device().CreateFence(0, D3D12_FENCE_FLAG_NONE)? };

        let fence_event = unsafe { CreateEventW(None, false, false, None)? };

        let vertex_shader = D3D12Shader::from_bytes(TRIANGLE_VERTEX_SHADER);

        let pixel_shader = D3D12Shader::from_bytes(TRIANGLE_PIXEL_SHADER);

        let pipeline = D3D12Pipeline::new(device, &vertex_shader, &pixel_shader)?;
        let background_pipeline = D3D12Pipeline::new_background(
            device,
            &D3D12Shader::from_bytes(BACKGROUND_VERTEX_SHADER),
            &D3D12Shader::from_bytes(BACKGROUND_PIXEL_SHADER),
        )?;

        let constant_buffer = D3D12ConstantBuffer::new(device)?;

        let texture_heap_desc = D3D12_DESCRIPTOR_HEAP_DESC {
            Type: D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV,
            NumDescriptors: MAX_TEXTURES,
            Flags: D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE,
            NodeMask: 0,
        };

        let texture_heap: ID3D12DescriptorHeap =
            unsafe { device.device().CreateDescriptorHeap(&texture_heap_desc)? };

        let texture_descriptor_size = unsafe {
            device
                .device()
                .GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV)
        };

        Ok(Self {
            command_allocators,
            command_list,

            pipeline,
            background_pipeline,

            mesh_cache: HashMap::new(),

            fence,
            fence_value: 0,
            fence_event,

            constant_buffer,

            active_lights: Vec::new(),
            environment: None,
            environment_descriptor_cache: HashMap::new(),
            irradiance_cache: HashMap::new(),
            fallback_environment_texture: crate::graphics::Texture::solid(crate::graphics::Color::BLACK),
            texture_cache: HashMap::new(),
            material_descriptor_cache: HashMap::new(),
            texture_heap,
            texture_descriptor_size,
            next_texture_descriptor: 0,
            pending_background: None,
            background_drawn: false,
        })
    }

    pub(crate) fn begin_frame(
        &mut self,
        _device: &D3D12Device,
        swap_chain: &D3D12SwapChain,
    ) -> Result<()> {
        let frame_index = swap_chain.current_frame_index();
        let command_allocator = &self.command_allocators[frame_index];

        self.constant_buffer.reset();
        self.active_lights.clear();
        self.background_drawn = false;
        self.background_drawn = false;

        unsafe {
            command_allocator.Reset()?;

            self.command_list.Reset(command_allocator, None)?;

            self.command_list
                .SetGraphicsRootSignature(self.pipeline.root_signature());

            self.command_list
                .SetPipelineState(self.pipeline.pipeline_state());
        }

        let render_target = swap_chain.current_render_target();

        transition_resource(
            &self.command_list,
            render_target,
            D3D12_RESOURCE_STATE_PRESENT,
            D3D12_RESOURCE_STATE_RENDER_TARGET,
        );

        let clear_color = if let Some(environment) = &self.environment {
            [
                environment.background_color.r,
                environment.background_color.g,
                environment.background_color.b,
                environment.background_color.a,
            ]
        } else {
            [0.10, 0.15, 0.25, 1.0]
        };

        let rtv = swap_chain.current_rtv();
        let dsv = swap_chain.current_dsv();

        unsafe {
            self.command_list
                .ClearDepthStencilView(dsv, D3D12_CLEAR_FLAG_DEPTH, 1.0, 0, None);

            self.command_list
                .ClearRenderTargetView(rtv, &clear_color, None);

            self.command_list
                .OMSetRenderTargets(1, Some(&rtv), false, Some(&dsv));

            self.command_list
                .IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);

            self.command_list
                .SetDescriptorHeaps(&[Some(self.texture_heap.clone())]);
        }

        Ok(())
    }

    pub(crate) fn draw_mesh(
        &mut self,
        device: &D3D12Device,
        mesh: &Mesh,
        material: &Material,
        transform: &Transform,
    ) -> Result<()> {
        if !self.background_drawn {
            self.draw_background(device)?;
            self.background_drawn = true;
        }

        let model_matrix = transform.matrix_around(mesh.pivot());

        let normal_matrix = transform.normal_matrix();

        let texture = material.base_texture();

        let texture_id = texture.id();

        if !self.texture_cache.contains_key(&texture_id) {
            assert!(
                self.next_texture_descriptor < MAX_TEXTURES,
                "Oxide exceeded the temporary GPU texture limit"
            );

            let descriptor_index = self.next_texture_descriptor;

            self.next_texture_descriptor += 1;

            let gpu_texture =
                D3D12Texture::new(device, &self.command_list, texture, descriptor_index)?;

            self.create_texture_srv(device, &gpu_texture);

            self.texture_cache.insert(texture_id, gpu_texture);
        }

        let base_descriptor_index = self
            .texture_cache
            .get(&texture_id)
            .expect("texture should be cached after upload")
            .descriptor_index();

        let normal_texture = material.normal_texture();
        let normal_texture_id = normal_texture.id();

        if !self.texture_cache.contains_key(&normal_texture_id) {
            assert!(
                self.next_texture_descriptor < MAX_TEXTURES,
                "Oxide exceeded the temporary GPU texture limit"
            );
            let descriptor_index = self.next_texture_descriptor;
            self.next_texture_descriptor += 1;
            let gpu_normal =
                D3D12Texture::new(device, &self.command_list, normal_texture, descriptor_index)?;
            self.create_texture_srv(device, &gpu_normal);
            self.texture_cache.insert(normal_texture_id, gpu_normal);
        }

        let normal_descriptor_index = self
            .texture_cache
            .get(&normal_texture_id)
            .expect("normal texture should be cached after upload")
            .descriptor_index();

        // t0 and t1 must be consecutive in the descriptor table. Cache one
        // stable descriptor pair for each base/normal texture combination.
        let pair_key = (texture_id, normal_texture_id);
        let descriptor_index = if let Some(&index) =
            self.material_descriptor_cache.get(&pair_key)
        {
            index
        } else {
            let index = self.next_texture_descriptor;
            assert!(
                index + 1 < MAX_TEXTURES,
                "Oxide exceeded the temporary GPU texture limit"
            );
            self.next_texture_descriptor += 2;
            self.copy_texture_descriptor(
                device,
                base_descriptor_index,
                index,
            );
            self.copy_texture_descriptor(
                device,
                normal_descriptor_index,
                index + 1,
            );
            self.material_descriptor_cache.insert(pair_key, index);
            index
        };

        let mut gpu_handle: D3D12_GPU_DESCRIPTOR_HANDLE =
            unsafe { self.texture_heap.GetGPUDescriptorHandleForHeapStart() };
        gpu_handle.ptr += descriptor_index as u64 * self.texture_descriptor_size as u64;

        let environment_handle = self.prepare_environment_descriptors(device)?;

        let base_color = material.base_color();
        let emissive = material.emissive_color();

        let object_constants = ObjectConstants {
            model: model_matrix.transposed().as_array(),
            normal: normal_matrix.transposed().as_array(),
            base_color: [
                base_color.r, base_color.g,
                base_color.b, base_color.a,
            ],
            material: [
                material.metallic(),
                material.roughness(),
                material.emissive_strength(),
                material.normal_strength(),
            ],
            emissive: [
                emissive.r, emissive.g,
                emissive.b, emissive.a,
            ],
        };

        let model_address = self.constant_buffer.write(&object_constants);

        let mesh_id = mesh.id();

        if !self.mesh_cache.contains_key(&mesh_id) {
            let cached = CachedMesh::new(device, mesh)?;

            self.mesh_cache.insert(mesh_id, cached);
        }

        let cached = self
            .mesh_cache
            .get_mut(&mesh_id)
            .expect("mesh should be cached after insertion");

        cached.synchronize(device, mesh)?;

        let mut light_constants = LightConstants {
            point_lights: [GpuPointLight::default(); MAX_POINT_LIGHTS],

            spot_lights: [GpuSpotLight::default(); MAX_SPOT_LIGHTS],

            directional_lights: [GpuDirectionalLight::default(); MAX_DIRECTIONAL_LIGHTS],

            point_light_count: 0,
            spot_light_count: 0,
            directional_light_count: 0,

            _padding: 0,
            environment_ambient: [0.0; 4],
            environment_params: [0.0; 4],
        };

        if let Some(environment) = &self.environment {
            light_constants.environment_ambient = [
                environment.ambient_color.r,
                environment.ambient_color.g,
                environment.ambient_color.b,
                environment.ambient_intensity,
            ];
            light_constants.environment_params = [
                environment.environment_intensity,
                environment.environment_texture.is_some() as u8 as f32,
                environment.background_texture.is_some() as u8 as f32,
                0.0,
            ];
        }

        for light in &self.active_lights {
            match light {
                ActiveLight::Point(light) => {
                    let index = light_constants.point_light_count as usize;

                    if index >= MAX_POINT_LIGHTS {
                        continue;
                    }

                    let position = light.position();
                    let color = light.color();

                    light_constants.point_lights[index] = GpuPointLight {
                        position: [position.x, position.y, position.z, light.intensity()],

                        color: [color.r, color.g, color.b, color.a],
                    };

                    light_constants.point_light_count += 1;
                }

                ActiveLight::Spot(light) => {
                    let index = light_constants.spot_light_count as usize;

                    if index >= MAX_SPOT_LIGHTS {
                        continue;
                    }

                    let position = light.position();
                    let direction = light.direction();
                    let color = light.color();

                    light_constants.spot_lights[index] = GpuSpotLight {
                        position: [position.x, position.y, position.z, light.intensity()],

                        direction: [
                            direction.x,
                            direction.y,
                            direction.z,
                            light.angle().as_radians().cos(),
                        ],

                        color: [color.r, color.g, color.b, color.a],
                    };

                    light_constants.spot_light_count += 1;
                }

                ActiveLight::Directional(light) => {
                    let index = light_constants.directional_light_count as usize;

                    if index >= MAX_DIRECTIONAL_LIGHTS {
                        continue;
                    }

                    let direction = light.direction();
                    let color = light.color();

                    light_constants.directional_lights[index] = GpuDirectionalLight {
                        direction: [direction.x, direction.y, direction.z, 0.0],

                        color: [color.r, color.g, color.b, light.intensity()],
                    };

                    light_constants.directional_light_count += 1;
                }
            }
        }

        let light_address = self.constant_buffer.write(&light_constants);

        unsafe {
            self.command_list
                .SetGraphicsRootConstantBufferView(2, light_address);

            self.command_list
                .IASetVertexBuffers(0, Some(std::slice::from_ref(cached.vertex_buffer.view())));

            self.command_list
                .IASetIndexBuffer(Some(cached.index_buffer.view()));

            self.command_list
                .SetGraphicsRootConstantBufferView(0, model_address);

            self.command_list
                .SetGraphicsRootDescriptorTable(3, gpu_handle);

            self.command_list
                .SetGraphicsRootDescriptorTable(4, environment_handle);

            self.command_list
                .DrawIndexedInstanced(cached.index_buffer.index_count(), 1, 0, 0, 0);
        }

        Ok(())
    }

    pub(crate) fn end_frame(
        &mut self,
        device: &D3D12Device,
        swap_chain: &D3D12SwapChain,
    ) -> Result<()> {
        let render_target = swap_chain.current_render_target();

        transition_resource(
            &self.command_list,
            render_target,
            D3D12_RESOURCE_STATE_RENDER_TARGET,
            D3D12_RESOURCE_STATE_PRESENT,
        );

        unsafe {
            self.command_list.Close()?;
        }

        let command_list: ID3D12CommandList = self.command_list.cast()?;

        unsafe {
            device
                .command_queue()
                .ExecuteCommandLists(&[Some(command_list)]);
        }

        swap_chain.present()?;

        self.wait_for_gpu(device)?;

        Ok(())
    }

    pub(crate) fn wait_for_gpu(&mut self, device: &D3D12Device) -> Result<()> {
        self.fence_value += 1;

        let value = self.fence_value;

        unsafe {
            device.command_queue().Signal(&self.fence, value)?;
        }

        if unsafe { self.fence.GetCompletedValue() } < value {
            unsafe {
                self.fence.SetEventOnCompletion(value, self.fence_event)?;

                WaitForSingleObject(self.fence_event, INFINITE);
            }
        }

        Ok(())
    }

    pub(crate) fn prepare_for_resize(&mut self, device: &D3D12Device) -> Result<()> {
        // Ensure the GPU has completely finished all submitted work.
        self.wait_for_gpu(device)?;

        // Reset every allocator now that the GPU is finished with them.
        for allocator in &self.command_allocators {
            unsafe {
                allocator.Reset()?;
            }
        }

        // Reset and immediately close the command list so its previous
        // recorded command stream is discarded.
        unsafe {
            self.command_list.Reset(&self.command_allocators[0], None)?;

            self.command_list.Close()?;
        }

        Ok(())
    }

    pub(crate) fn bind_camera(&mut self, camera: &Camera, destination: Rect) -> Result<()> {
        assert!(
            destination.width > 0.0 && destination.height > 0.0,
            "camera destination must have positive dimensions"
        );

        let view = camera.view_matrix();

        let projection = camera.projection_matrix(destination.aspect_ratio());

        let position = camera.position();
        let rotation = camera.rotation();
        let right = rotation.rotate_vector(crate::math::Vec3::new(1.0, 0.0, 0.0));
        let up = rotation.rotate_vector(crate::math::Vec3::new(0.0, 1.0, 0.0));
        let forward = rotation.rotate_vector(crate::math::Vec3::new(0.0, 0.0, -1.0));
        self.pending_background = Some(BackgroundConstants {
            camera_right: [right.x, right.y, right.z, 0.0],
            camera_up: [up.x, up.y, up.z, 0.0],
            camera_forward: [forward.x, forward.y, forward.z, 0.0],
            projection_params: [
                (camera.field_of_view().as_radians() * 0.5).tan(),
                destination.aspect_ratio(),
                0.0,
                0.0,
            ],
        });

        let view_constants = ViewConstants {
            view: view.transposed().as_array(),
            projection: projection.transposed().as_array(),
            camera_position: [position.x, position.y, position.z, 1.0],
        };

        let view_address = self.constant_buffer.write(&view_constants);

        let viewport = D3D12_VIEWPORT {
            TopLeftX: destination.x,
            TopLeftY: destination.y,
            Width: destination.width,
            Height: destination.height,
            MinDepth: 0.0,
            MaxDepth: 1.0,
        };

        let scissor = RECT {
            left: destination.x as i32,
            top: destination.y as i32,
            right: (destination.x + destination.width) as i32,
            bottom: (destination.y + destination.height) as i32,
        };

        unsafe {
            self.command_list
                .SetGraphicsRootConstantBufferView(1, view_address);

            self.command_list.RSSetViewports(&[viewport]);

            self.command_list.RSSetScissorRects(&[scissor]);
        }

        Ok(())
    }


    pub(crate) fn set_environment(&mut self, environment: Environment) {
        self.environment = Some(environment);
    }

    pub(crate) fn clear_environment(&mut self) {
        self.environment = None;
    }

    pub(crate) fn add_point_light(&mut self, light: &PointLight) {
        self.active_lights.push(ActiveLight::Point(*light));
    }

    pub(crate) fn add_spot_light(&mut self, light: &SpotLight) {
        self.active_lights.push(ActiveLight::Spot(*light));
    }

    pub(crate) fn add_directional_light(&mut self, light: &DirectionalLight) {
        self.active_lights.push(ActiveLight::Directional(*light));
    }

    pub(crate) fn clear_lights(&mut self) {
        self.active_lights.clear();
    }




    fn draw_background(&mut self, device: &D3D12Device) -> Result<()> {
        let Some(constants) = self.pending_background else {
            return Ok(());
        };
        let Some(texture) = self
            .environment
            .as_ref()
            .and_then(|environment| environment.background_texture.as_ref())
        else {
            return Ok(());
        };

        let texture_id = texture.id();
        if !self.texture_cache.contains_key(&texture_id) {
            let descriptor_index = self.next_texture_descriptor;
            assert!(descriptor_index < MAX_TEXTURES);
            self.next_texture_descriptor += 1;
            let gpu = D3D12Texture::new(device, &self.command_list, texture, descriptor_index)?;
            self.create_texture_srv(device, &gpu);
            self.texture_cache.insert(texture_id, gpu);
        }
        let descriptor_index = self.texture_cache[&texture_id].descriptor_index();
        let mut handle = unsafe { self.texture_heap.GetGPUDescriptorHandleForHeapStart() };
        handle.ptr += descriptor_index as u64 * self.texture_descriptor_size as u64;
        let address = self.constant_buffer.write(&constants);

        unsafe {
            self.command_list.SetGraphicsRootSignature(self.background_pipeline.root_signature());
            self.command_list.SetPipelineState(self.background_pipeline.pipeline_state());
            self.command_list.SetGraphicsRootConstantBufferView(0, address);
            self.command_list.SetGraphicsRootDescriptorTable(1, handle);
            self.command_list.DrawInstanced(3, 1, 0, 0);

            self.command_list.SetGraphicsRootSignature(self.pipeline.root_signature());
            self.command_list.SetPipelineState(self.pipeline.pipeline_state());
        }
        Ok(())
    }


    fn prepare_environment_descriptors(
        &mut self,
        device: &D3D12Device,
    ) -> Result<D3D12_GPU_DESCRIPTOR_HANDLE> {
        let environment_texture = self
            .environment
            .as_ref()
            .and_then(|environment| environment.environment_texture.as_ref());
        let background_texture = self
            .environment
            .as_ref()
            .and_then(|environment| environment.background_texture.as_ref());

        let environment_id = environment_texture.map(|texture| texture.id());
        let background_id = background_texture.map(|texture| texture.id());

        if let Some(texture) = environment_texture {
            let id = texture.id();
            if !self.texture_cache.contains_key(&id) {
                let prefilter = texture.environment_prefilter();

                let specular_index = self.next_texture_descriptor;
                assert!(specular_index < MAX_TEXTURES);
                self.next_texture_descriptor += 1;
                let specular = D3D12Texture::new_float_levels(
                    device,
                    &self.command_list,
                    &prefilter.specular,
                    specular_index,
                )?;
                self.create_texture_srv(device, &specular);
                self.texture_cache.insert(id, specular);

                let irradiance_index = self.next_texture_descriptor;
                assert!(irradiance_index < MAX_TEXTURES);
                self.next_texture_descriptor += 1;
                let irradiance_level = vec![prefilter.irradiance];
                let irradiance = D3D12Texture::new_float_levels(
                    device,
                    &self.command_list,
                    &irradiance_level,
                    irradiance_index,
                )?;
                self.create_texture_srv(device, &irradiance);
                self.irradiance_cache.insert(id, irradiance);
            }
        }

        if let Some(texture) = background_texture {
            let id = texture.id();
            if !self.texture_cache.contains_key(&id) {
                let index = self.next_texture_descriptor;
                assert!(index < MAX_TEXTURES);
                self.next_texture_descriptor += 1;
                let gpu = D3D12Texture::new(
                    device,
                    &self.command_list,
                    texture,
                    index,
                )?;
                self.create_texture_srv(device, &gpu);
                self.texture_cache.insert(id, gpu);
            }
        }

        let fallback_id = self.fallback_environment_texture.id();
        if !self.texture_cache.contains_key(&fallback_id) {
            let index = self.next_texture_descriptor;
            assert!(index < MAX_TEXTURES);
            self.next_texture_descriptor += 1;
            let gpu = D3D12Texture::new(
                device,
                &self.command_list,
                &self.fallback_environment_texture,
                index,
            )?;
            self.create_texture_srv(device, &gpu);
            self.texture_cache.insert(fallback_id, gpu);
        }

        let key = (environment_id, background_id);
        let descriptor_index = if let Some(&index) =
            self.environment_descriptor_cache.get(&key)
        {
            index
        } else {
            let index = self.next_texture_descriptor;
            assert!(index + 2 < MAX_TEXTURES);
            self.next_texture_descriptor += 3;

            let fallback_descriptor =
                self.texture_cache[&fallback_id].descriptor_index();
            let specular_descriptor = environment_id
                .map(|id| self.texture_cache[&id].descriptor_index())
                .unwrap_or(fallback_descriptor);
            let irradiance_descriptor = environment_id
                .and_then(|id| self.irradiance_cache.get(&id))
                .map(|texture| texture.descriptor_index())
                .unwrap_or(fallback_descriptor);
            let background_descriptor = background_id
                .map(|id| self.texture_cache[&id].descriptor_index())
                .unwrap_or(fallback_descriptor);

            self.copy_texture_descriptor(device, specular_descriptor, index);
            self.copy_texture_descriptor(device, irradiance_descriptor, index + 1);
            self.copy_texture_descriptor(device, background_descriptor, index + 2);
            self.environment_descriptor_cache.insert(key, index);
            index
        };

        let mut handle =
            unsafe { self.texture_heap.GetGPUDescriptorHandleForHeapStart() };
        handle.ptr +=
            descriptor_index as u64 * self.texture_descriptor_size as u64;
        Ok(handle)
    }

    fn copy_texture_descriptor(
        &self,
        device: &D3D12Device,
        source_index: u32,
        destination_index: u32,
    ) {
        use windows::Win32::Graphics::Direct3D12::D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV;

        let mut source = unsafe { self.texture_heap.GetCPUDescriptorHandleForHeapStart() };
        source.ptr += source_index as usize * self.texture_descriptor_size as usize;

        let mut destination = unsafe { self.texture_heap.GetCPUDescriptorHandleForHeapStart() };
        destination.ptr += destination_index as usize * self.texture_descriptor_size as usize;

        unsafe {
            device.device().CopyDescriptorsSimple(
                1,
                destination,
                source,
                D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV,
            );
        }
    }

    fn create_texture_srv(&self, device: &D3D12Device, texture: &D3D12Texture) {
        let mut handle = unsafe { self.texture_heap.GetCPUDescriptorHandleForHeapStart() };

        handle.ptr += texture.descriptor_index() as usize * self.texture_descriptor_size as usize;

        let desc = D3D12_SHADER_RESOURCE_VIEW_DESC {
            Format: texture.format(),

            ViewDimension: D3D12_SRV_DIMENSION_TEXTURE2D,

            Shader4ComponentMapping: D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING,

            Anonymous: windows::Win32::Graphics::Direct3D12::D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                Texture2D: D3D12_TEX2D_SRV {
                    MostDetailedMip: 0,
                    MipLevels: u32::MAX,
                    PlaneSlice: 0,
                    ResourceMinLODClamp: 0.0,
                },
            },
        };

        unsafe {
            device
                .device()
                .CreateShaderResourceView(Some(texture.resource()), Some(&desc), handle);
        }
    }
}

impl Drop for D3D12Renderer {
    fn drop(&mut self) {
        if !self.fence_event.is_invalid() {
            unsafe {
                let _ = CloseHandle(self.fence_event);
            }
        }
    }
}
