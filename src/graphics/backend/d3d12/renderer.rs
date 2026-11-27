use windows::{
    Win32::Graphics::{
        Direct3D12::{
            D3D12_CLEAR_FLAG_DEPTH, D3D12_COMMAND_LIST_TYPE_DIRECT,
            D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING, D3D12_DESCRIPTOR_HEAP_DESC,
            D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE, D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV,
            D3D12_GPU_DESCRIPTOR_HANDLE, D3D12_RESOURCE_BARRIER, D3D12_RESOURCE_BARRIER_0,
            D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES, D3D12_RESOURCE_BARRIER_FLAG_NONE,
            D3D12_RESOURCE_BARRIER_TYPE_TRANSITION, D3D12_RESOURCE_STATE_PRESENT,
            D3D12_RESOURCE_STATE_RENDER_TARGET, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE,
            D3D12_RESOURCE_STATES,
            D3D12_RESOURCE_TRANSITION_BARRIER, D3D12_SHADER_RESOURCE_VIEW_DESC,
            D3D12_SRV_DIMENSION_TEXTURE2D, D3D12_TEX2D_SRV, ID3D12CommandAllocator,
            ID3D12CommandList, ID3D12DescriptorHeap, ID3D12GraphicsCommandList, ID3D12Resource,
        },
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

use std::collections::{HashMap, HashSet};
use super::{
    render_target::HdrTarget, shadow_map::ShadowMap,
    transmission_map::{TransmissionMap, TRANSMISSION_LAYERS, TRANSMISSION_MAX_RESOLUTION, FIRST_TEXTURE_DESCRIPTOR, MAX_SHADOW_LIGHTS, PREVIOUS_VIEWS_PER_LIGHT, array_srv},
};
use windows::Win32::Graphics::Direct3D12::{D3D12_RESOURCE_STATE_DEPTH_WRITE, D3D12_CPU_DESCRIPTOR_HANDLE};
use windows::Win32::Graphics::Dxgi::Common::{
    DXGI_FORMAT_R16G16B16A16_FLOAT, DXGI_FORMAT_R8G8B8A8_UNORM,
    DXGI_FORMAT_R8G8B8A8_UNORM_SRGB,
};

use super::{
    D3D12ConstantBuffer, D3D12Device, D3D12IndexBuffer, D3D12Pipeline, D3D12Shader, D3D12SwapChain,
    D3D12Texture, D3D12VertexBuffer, swap_chain::FRAME_COUNT,
};

use crate::{
    graphics::{
        Camera, DirectionalLight, ShadowSettings, Environment, GraphicsResult, Material, Mesh, PointLight,
        SpotLight, mesh::MeshId, texture::TextureId,
    },
    math::{Mat4, Rect, Transform, Vec3},
};

const TRIANGLE_VERTEX_SHADER: &[u8] =
    include_bytes!("../../../../shaders/compiled/triangle_vs.dxil");

const TRIANGLE_PIXEL_SHADER: &[u8] =
    include_bytes!("../../../../shaders/compiled/triangle_ps.dxil");
const BACKGROUND_VERTEX_SHADER: &[u8] =
    include_bytes!("../../../../shaders/compiled/background_vs.dxil");
const BACKGROUND_PIXEL_SHADER: &[u8] =
    include_bytes!("../../../../shaders/compiled/background_ps.dxil");

const TONEMAP_VERTEX_SHADER: &[u8] =
    include_bytes!("../../../../shaders/compiled/tonemap_vs.dxil");
const TONEMAP_PIXEL_SHADER: &[u8] =
    include_bytes!("../../../../shaders/compiled/tonemap_ps.dxil");

const SHADOW_VERTEX_SHADER: &[u8] = include_bytes!("../../../../shaders/compiled/shadow_vs.dxil");
const SHADOW_PIXEL_SHADER: &[u8] = include_bytes!("../../../../shaders/compiled/shadow_ps.dxil");

type MeshKey = (MeshId, u64, u64);

#[derive(Clone, Copy)]
struct QueuedMesh {
    mesh: MeshKey,
    model: Mat4,
    transparency: f32,
    sort_depth: f32,
    destination: Rect,
    view: u64,
    object: u64,
    lights: u64,
    material: D3D12_GPU_DESCRIPTOR_HANDLE,
    environment: D3D12_GPU_DESCRIPTOR_HANDLE,
}

#[derive(Clone, Copy)]
enum SceneCommand {
    Background { constants: u64, texture: D3D12_GPU_DESCRIPTOR_HANDLE, destination: Rect },
    Mesh(QueuedMesh),
}

fn unique_casters(commands: &[SceneCommand]) -> Vec<QueuedMesh> {
    let mut seen = HashSet::new();
    commands.iter().filter_map(|command| {
        let SceneCommand::Mesh(draw) = command else { return None; };
        let bits = draw.model.as_array().map(|row| row.map(f32::to_bits));
        let key = (draw.mesh, bits, draw.transparency.to_bits());
        (draw.transparency < 1.0 && seen.insert(key)).then_some(*draw)
    }).collect()
}

#[derive(Clone, Copy)]
struct ShadowRequest {
    light: ActiveLight,
    settings: ShadowSettings,
    matrices: [Mat4; 6],
    faces: u32,
    position: Vec3,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ShadowConstants {
    matrices: [[[f32; 4]; 4]; 6],
    parameters: [f32; 4],
    transmission: [f32; 4],
    position: [f32; 4],
}

impl Default for ShadowConstants {
    fn default() -> Self {
        Self { matrices: [[[0.0; 4]; 4]; 6], parameters: [0.0; 4], transmission: [0.0; 4], position: [0.0; 4] }
    }
}

struct ShadowResources {
    opaque: ShadowMap,
    transmission: Option<TransmissionMap>,
}

fn invalid_shadow_order(message: &str) -> windows::core::Error {
    windows::core::Error::new(windows::Win32::Foundation::E_INVALIDARG, message)
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ActiveLight {
    Point(PointLight),
    Spot(SpotLight),
    Directional(DirectionalLight),
}

impl ActiveLight {
    fn shadow_settings(&self) -> &ShadowSettings {
        match self { Self::Point(l) => l.shadow_settings(), Self::Spot(l) => l.shadow_settings(), Self::Directional(l) => l.shadow_settings() }
    }
    fn casts_shadows(&self) -> bool {
        match self { Self::Point(l) => l.casts_shadows(), Self::Spot(l) => l.casts_shadows(), Self::Directional(l) => l.casts_shadows() }
    }
    fn shadow_request(self) -> crate::OxideResult<ShadowRequest> {
        let settings = *self.shadow_settings();
        settings.validate()?;
        let mut matrices = [Mat4::identity(); 6];
        let (faces, position) = match self {
            Self::Directional(light) => { matrices[0] = settings.light_matrix(light.direction())?; (1, Vec3::default()) }
            Self::Spot(light) if light.angle().as_radians() < std::f32::consts::FRAC_PI_2 - 0.001 => {
                matrices[0] = settings.perspective_matrix(light.position(), light.direction(), 2.0 * light.angle().as_radians())?;
                (1, light.position())
            }
            _ => {
                let position = match self { Self::Point(light) => light.position(), Self::Spot(light) => light.position(), _ => unreachable!() };
                for (index, direction) in [Vec3::new(1.0,0.0,0.0), Vec3::new(-1.0,0.0,0.0), Vec3::new(0.0,1.0,0.0), Vec3::new(0.0,-1.0,0.0), Vec3::new(0.0,0.0,1.0), Vec3::new(0.0,0.0,-1.0)].into_iter().enumerate() {
                    matrices[index] = settings.perspective_matrix(position, direction, std::f32::consts::FRAC_PI_2)?;
                }
                (6, position)
            }
        };
        Ok(ShadowRequest { light: self, settings, matrices, faces, position })
    }
}

fn transition_resource(
    command_list: &ID3D12GraphicsCommandList,
    resource: &ID3D12Resource,
    before: D3D12_RESOURCE_STATES,
    after: D3D12_RESOURCE_STATES,
) {
    transition_slice(command_list, resource, before, after, D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES);
}

fn transition_slice(
    command_list: &ID3D12GraphicsCommandList,
    resource: &ID3D12Resource,
    before: D3D12_RESOURCE_STATES,
    after: D3D12_RESOURCE_STATES,
    subresource: u32,
) {
    let mut barrier = D3D12_RESOURCE_BARRIER {
        Type: D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
        Flags: D3D12_RESOURCE_BARRIER_FLAG_NONE,
        Anonymous: D3D12_RESOURCE_BARRIER_0 {
            Transition: std::mem::ManuallyDrop::new(D3D12_RESOURCE_TRANSITION_BARRIER {
                pResource: std::mem::ManuallyDrop::new(Some(resource.clone())),
                Subresource: subresource,
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
    transparency: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct BackgroundConstants {
    camera_right: [f32; 4],
    camera_up: [f32; 4],
    camera_forward: [f32; 4],
    projection_params: [f32; 4],
    fallback_color: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ViewConstants {
    view: [[f32; 4]; 4],
    projection: [[f32; 4]; 4],
    camera_position: [f32; 4],
}

struct CachedMesh {
    center: Vec3,
    vertex_buffer: D3D12VertexBuffer,
    index_buffer: D3D12IndexBuffer,
}

impl CachedMesh {
    fn new(device: &D3D12Device, mesh: &Mesh) -> Result<Self> {
        let (minimum, maximum) = mesh.vertices().iter().fold(
            (Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY), Vec3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY)),
            |(low, high), vertex| (
                Vec3::new(low.x.min(vertex.position.x), low.y.min(vertex.position.y), low.z.min(vertex.position.z)),
                Vec3::new(high.x.max(vertex.position.x), high.y.max(vertex.position.y), high.z.max(vertex.position.z)),
            ),
        );
        let center = if mesh.vertices().is_empty() { Vec3::default() } else { (minimum + maximum) * 0.5 };
        Ok(Self {
            center,
            vertex_buffer: D3D12VertexBuffer::new(device, mesh.vertices())?,
            index_buffer: D3D12IndexBuffer::new(device, mesh.indices())?,
        })
    }
}

const MAX_TEXTURES: u32 = 4096;

pub(crate) struct D3D12Renderer {
    command_allocators: [ID3D12CommandAllocator; FRAME_COUNT],
    command_list: ID3D12GraphicsCommandList,

    pipeline: D3D12Pipeline,
    transparent_pipeline: D3D12Pipeline,
    camera_view: Option<Mat4>,
    background_pipeline: D3D12Pipeline,
    display_pipeline: D3D12Pipeline,
    hdr_target: Option<HdrTarget>,
    shadow_pipeline: D3D12Pipeline,
    shadow_requests: Vec<ShadowRequest>,
    shadow_resources: Vec<Option<ShadowResources>>,
    transmission_pipeline: D3D12Pipeline,
    commands: Vec<SceneCommand>,
    destination: Option<Rect>,
    latest_mesh_versions: HashMap<MeshId, (u64, u64)>,
    exposure: f32,
    view_address: Option<u64>,

    mesh_cache: HashMap<MeshKey, CachedMesh>,

    fence: ID3D12Fence,
    fence_value: u64,
    fence_event: HANDLE,

    constant_buffer: D3D12ConstantBuffer,

    active_lights: Vec<ActiveLight>,
    environment: Option<Environment>,
    environment_descriptor_cache: HashMap<(Option<TextureId>, Option<TextureId>), u32>,
    irradiance_cache: HashMap<TextureId, D3D12Texture>,
    specular_cache: HashMap<TextureId, D3D12Texture>,
    normal_descriptor_cache: HashMap<TextureId, u32>,
    fallback_environment_texture: crate::graphics::Texture,
    texture_cache: HashMap<TextureId, D3D12Texture>,
    material_descriptor_cache: HashMap<(TextureId, TextureId), u32>,
    texture_heap: ID3D12DescriptorHeap,
    texture_staging_heap: ID3D12DescriptorHeap,
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
        let transparent_pipeline = D3D12Pipeline::new_transparent(device, &vertex_shader, &pixel_shader)?;
        let background_pipeline = D3D12Pipeline::new_fullscreen(
            device,
            &D3D12Shader::from_bytes(BACKGROUND_VERTEX_SHADER),
            &D3D12Shader::from_bytes(BACKGROUND_PIXEL_SHADER),
            DXGI_FORMAT_R16G16B16A16_FLOAT,
        )?;

        let display_pipeline = D3D12Pipeline::new_fullscreen(
            device,
            &D3D12Shader::from_bytes(TONEMAP_VERTEX_SHADER),
            &D3D12Shader::from_bytes(TONEMAP_PIXEL_SHADER),
            DXGI_FORMAT_R8G8B8A8_UNORM,
        )?;
        let shadow_pipeline = D3D12Pipeline::new_shadow(device, &D3D12Shader::from_bytes(SHADOW_VERTEX_SHADER))?;
        let transmission_pipeline = D3D12Pipeline::new_transmission(
            device, &D3D12Shader::from_bytes(SHADOW_VERTEX_SHADER), &D3D12Shader::from_bytes(SHADOW_PIXEL_SHADER),
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

        let texture_staging_heap: ID3D12DescriptorHeap = unsafe {
            device.device().CreateDescriptorHeap(&D3D12_DESCRIPTOR_HEAP_DESC {
                Type: D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV,
                NumDescriptors: MAX_TEXTURES,
                Flags: windows::Win32::Graphics::Direct3D12::D3D12_DESCRIPTOR_HEAP_FLAG_NONE,
                NodeMask: 0,
            })?
        };
        let texture_descriptor_size = unsafe {
            device
                .device()
                .GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV)
        };

        for index in 0..MAX_SHADOW_LIGHTS {
            for (base, format) in [(1, windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R32_FLOAT), (1 + MAX_SHADOW_LIGHTS, windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R32_FLOAT), (1 + 2 * MAX_SHADOW_LIGHTS, windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT_R16_FLOAT)] {
                let mut handle = unsafe { texture_heap.GetCPUDescriptorHandleForHeapStart() };
                handle.ptr += (base + index) * texture_descriptor_size as usize;
                array_srv(device, None, format, 0, 1, handle);
            }
        }

        Ok(Self {
            command_allocators,
            command_list,

            pipeline,
            transparent_pipeline,
            camera_view: None,
            background_pipeline,
            display_pipeline,
            hdr_target: None,
            shadow_pipeline,
            shadow_requests: Vec::new(),
            shadow_resources: Vec::new(),
            transmission_pipeline,
            commands: Vec::new(),
            destination: None,
            latest_mesh_versions: HashMap::new(),
            exposure: 1.0,
            view_address: None,

            mesh_cache: HashMap::new(),

            fence,
            fence_value: 0,
            fence_event,

            constant_buffer,

            active_lights: Vec::new(),
            environment: None,
            environment_descriptor_cache: HashMap::new(),
            irradiance_cache: HashMap::new(),
            specular_cache: HashMap::new(),
            normal_descriptor_cache: HashMap::new(),
            fallback_environment_texture: crate::graphics::Texture::solid(
                crate::graphics::Color::BLACK,
            ),
            texture_cache: HashMap::new(),
            material_descriptor_cache: HashMap::new(),
            texture_heap,
            texture_staging_heap,
            texture_descriptor_size,
            next_texture_descriptor: FIRST_TEXTURE_DESCRIPTOR,
            pending_background: None,
            background_drawn: false,
        })
    }

    fn reset_frame_state(&mut self) {
        self.commands.clear();
        self.shadow_requests.clear();
        self.destination = None;
        self.constant_buffer.reset();
        self.active_lights.clear();
        self.background_drawn = false;
        self.pending_background = None;
        self.view_address = None;
        self.camera_view = None;
    }

    pub(crate) fn begin_frame(
        &mut self,
        device: &D3D12Device,
        swap_chain: &D3D12SwapChain,
    ) -> Result<()> {
        self.reset_frame_state();
        let frame_index = swap_chain.current_frame_index();
        let command_allocator = &self.command_allocators[frame_index];


        unsafe {
            command_allocator.Reset()?;

            self.command_list.Reset(command_allocator, None)?;

            self.command_list
                .SetGraphicsRootSignature(self.pipeline.root_signature());

            self.command_list
                .SetPipelineState(self.pipeline.pipeline_state());
        }

        // End-frame waits for the GPU, so recreating this resource or its
        // reserved descriptor cannot race a previous frame.
        if self.hdr_target.as_ref().is_none_or(|target| {
            target.width != swap_chain.width() || target.height != swap_chain.height()
        }) {
            let srv = unsafe { self.texture_heap.GetCPUDescriptorHandleForHeapStart() };
            self.hdr_target = Some(HdrTarget::new(
                device, swap_chain.width(), swap_chain.height(), srv,
            )?);
        }
        let target = self.hdr_target.as_ref().unwrap();
        transition_resource(
            &self.command_list, &target.resource,
            D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE, D3D12_RESOURCE_STATE_RENDER_TARGET,
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

        let rtv = target.rtv;
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

    fn request_shadow(&mut self, light: ActiveLight) -> Result<f32> {
        if !light.casts_shadows() { return Ok(-1.0); }
        if let Some(index) = self.shadow_requests.iter().position(|request| request.light == light) { return Ok(index as f32); }
        if self.shadow_requests.len() == MAX_SHADOW_LIGHTS {
            return Err(invalid_shadow_order("frame exceeds 32 distinct shadow-enabled light snapshots"));
        }
        let request = light.shadow_request().map_err(|error| invalid_shadow_order(&error.to_string()))?;
        let index = self.shadow_requests.len();
        self.shadow_requests.push(request);
        Ok(index as f32)
    }

    fn shadow_cpu(&self, slot: usize) -> D3D12_CPU_DESCRIPTOR_HANDLE {
        let mut handle = unsafe { self.texture_heap.GetCPUDescriptorHandleForHeapStart() };
        handle.ptr += slot * self.texture_descriptor_size as usize;
        handle
    }

    fn set_viewport(&self, destination: Rect) {
        unsafe {
            self.command_list.RSSetViewports(&[D3D12_VIEWPORT {
                TopLeftX: destination.x, TopLeftY: destination.y,
                Width: destination.width, Height: destination.height,
                MinDepth: 0.0, MaxDepth: 1.0,
            }]);
            self.command_list.RSSetScissorRects(&[RECT {
                left: destination.x as i32, top: destination.y as i32,
                right: (destination.x + destination.width) as i32,
                bottom: (destination.y + destination.height) as i32,
            }]);
        }
    }

    fn record_caster(&mut self, draw: &QueuedMesh, matrix: Mat4, first_layer: bool) {
        #[repr(C)]
        #[derive(Clone, Copy)]
        struct Constants { matrix: [[f32; 4]; 4], parameters: [f32; 4] }
        let constants = Constants {
            matrix: (matrix * draw.model).transposed().as_array(),
            parameters: [draw.transparency, first_layer as u8 as f32, 0.0, 0.0],
        };
        let address = self.constant_buffer.write(&constants);
        let mesh = &self.mesh_cache[&draw.mesh];
        unsafe {
            self.command_list.SetGraphicsRootConstantBufferView(0, address);
            self.command_list.IASetVertexBuffers(0, Some(std::slice::from_ref(mesh.vertex_buffer.view())));
            self.command_list.IASetIndexBuffer(Some(mesh.index_buffer.view()));
            self.command_list.DrawIndexedInstanced(mesh.index_buffer.index_count(), 1, 0, 0, 0);
        }
    }

    fn render_shadows(&mut self, device: &D3D12Device) -> Result<u64> {
        let mut constants = [ShadowConstants::default(); MAX_SHADOW_LIGHTS];
        let casters = unique_casters(&self.commands);
        let transparent: Vec<_> = casters.iter().filter(|draw| draw.transparency > 0.0).collect();
        if self.shadow_resources.len() < self.shadow_requests.len() { self.shadow_resources.resize_with(self.shadow_requests.len(), || None); }
        for index in 0..self.shadow_requests.len() {
            let request = self.shadow_requests[index];
            let resolution = request.settings.resolution;
            if self.shadow_resources[index].as_ref().is_none_or(|resources| resources.opaque.resolution != resolution || resources.opaque.faces != request.faces) {
                let map = ShadowMap::new_array(device, resolution, request.faces, self.shadow_cpu(1 + index))?;
                self.shadow_resources[index] = Some(ShadowResources { opaque: map, transmission: None });
            }
            for face in 0..request.faces {
                let map = &self.shadow_resources[index].as_ref().unwrap().opaque;
                transition_slice(&self.command_list, &map.resource, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE, D3D12_RESOURCE_STATE_DEPTH_WRITE, face);
                let dsv = map.dsvs[face as usize];
                unsafe {
                    self.command_list.ClearDepthStencilView(dsv, D3D12_CLEAR_FLAG_DEPTH, 1.0, 0, None);
                    self.command_list.OMSetRenderTargets(0, None, false, Some(&dsv));
                    self.command_list.SetGraphicsRootSignature(self.shadow_pipeline.root_signature());
                    self.command_list.SetPipelineState(self.shadow_pipeline.pipeline_state());
                }
                self.set_viewport(Rect::new(0.0, 0.0, resolution as f32, resolution as f32));
                for draw in casters.iter().filter(|draw| draw.transparency == 0.0) { self.record_caster(draw, request.matrices[face as usize], true); }
                unsafe { self.command_list.OMSetRenderTargets(0, None, false, None); }
                transition_slice(&self.command_list, &self.shadow_resources[index].as_ref().unwrap().opaque.resource, D3D12_RESOURCE_STATE_DEPTH_WRITE, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE, face);
            }
            let mut transmission = [0.0; 4];
            if !transparent.is_empty() {
                let cap = if request.faces == 6 { 256 } else if matches!(request.light, ActiveLight::Spot(_)) { 512 } else { TRANSMISSION_MAX_RESOLUTION };
                let trans_resolution = resolution.min(cap);
                if self.shadow_resources[index].as_ref().unwrap().transmission.as_ref().is_none_or(|map| map.resolution != trans_resolution) {
                    let map = TransmissionMap::new(device, trans_resolution, request.faces,
                        self.shadow_cpu(1 + MAX_SHADOW_LIGHTS + index), self.shadow_cpu(1 + 2 * MAX_SHADOW_LIGHTS + index),
                        self.shadow_cpu(1 + 3 * MAX_SHADOW_LIGHTS + index * PREVIOUS_VIEWS_PER_LIGHT as usize), self.texture_descriptor_size as usize)?;
                    self.shadow_resources[index].as_mut().unwrap().transmission = Some(map);
                }
                self.set_viewport(Rect::new(0.0, 0.0, trans_resolution as f32, trans_resolution as f32));
                for face in 0..request.faces {
                    for layer in 0..TRANSMISSION_LAYERS {
                        let slice = face * TRANSMISSION_LAYERS + layer;
                        let map = self.shadow_resources[index].as_ref().unwrap().transmission.as_ref().unwrap();
                        transition_slice(&self.command_list, &map.depth, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE, D3D12_RESOURCE_STATE_DEPTH_WRITE, slice);
                        transition_slice(&self.command_list, &map.values, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE, D3D12_RESOURCE_STATE_RENDER_TARGET, slice);
                        let dsv = map.dsvs[slice as usize];
                        let rtv = map.rtvs[slice as usize];
                        let mut previous = unsafe { self.texture_heap.GetGPUDescriptorHandleForHeapStart() };
                        previous.ptr += (1 + 3 * MAX_SHADOW_LIGHTS + index * PREVIOUS_VIEWS_PER_LIGHT as usize + slice as usize) as u64 * self.texture_descriptor_size as u64;
                        unsafe {
                            self.command_list.ClearDepthStencilView(dsv, D3D12_CLEAR_FLAG_DEPTH, 1.0, 0, None);
                            self.command_list.ClearRenderTargetView(rtv, &[1.0; 4], None);
                            self.command_list.OMSetRenderTargets(1, Some(&rtv), false, Some(&dsv));
                            self.command_list.SetGraphicsRootSignature(self.transmission_pipeline.root_signature());
                            self.command_list.SetPipelineState(self.transmission_pipeline.pipeline_state());
                            self.command_list.SetGraphicsRootDescriptorTable(1, previous);
                        }
                        for draw in &transparent { self.record_caster(draw, request.matrices[face as usize], layer == 0); }
                        unsafe { self.command_list.OMSetRenderTargets(0, None, false, None); }
                        let map = self.shadow_resources[index].as_ref().unwrap().transmission.as_ref().unwrap();
                        transition_slice(&self.command_list, &map.depth, D3D12_RESOURCE_STATE_DEPTH_WRITE, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE, slice);
                        transition_slice(&self.command_list, &map.values, D3D12_RESOURCE_STATE_RENDER_TARGET, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE, slice);
                    }
                }
                transmission = [1.0 / trans_resolution as f32, TRANSMISSION_LAYERS as f32, 1.0, 0.0];
            }
            constants[index] = ShadowConstants {
                matrices: request.matrices.map(|matrix| matrix.transposed().as_array()),
                parameters: [request.settings.depth_bias, request.settings.normal_bias, 1.0 / resolution as f32, request.faces as f32],
                transmission,
                position: [request.position.x, request.position.y, request.position.z, 0.0],
            };
        }
        Ok(self.constant_buffer.write(&constants))
    }

    fn render_scene_mesh(&self, draw: &QueuedMesh, shadow_address: u64) {
        let pipeline = if draw.transparency > 0.0 { &self.transparent_pipeline } else { &self.pipeline };
        let mut shadow = unsafe { self.texture_heap.GetGPUDescriptorHandleForHeapStart() };
        shadow.ptr += self.texture_descriptor_size as u64;
        self.set_viewport(draw.destination);
        let mesh = &self.mesh_cache[&draw.mesh];
        unsafe {
            self.command_list.SetGraphicsRootSignature(pipeline.root_signature());
            self.command_list.SetPipelineState(pipeline.pipeline_state());
            self.command_list.SetGraphicsRootConstantBufferView(0, draw.object);
            self.command_list.SetGraphicsRootConstantBufferView(1, draw.view);
            self.command_list.SetGraphicsRootConstantBufferView(2, draw.lights);
            self.command_list.SetGraphicsRootConstantBufferView(5, shadow_address);
            self.command_list.SetGraphicsRootDescriptorTable(3, draw.material);
            self.command_list.SetGraphicsRootDescriptorTable(4, draw.environment);
            self.command_list.SetGraphicsRootDescriptorTable(6, shadow);
            self.command_list.IASetVertexBuffers(0, Some(std::slice::from_ref(mesh.vertex_buffer.view())));
            self.command_list.IASetIndexBuffer(Some(mesh.index_buffer.view()));
            self.command_list.DrawIndexedInstanced(mesh.index_buffer.index_count(), 1, 0, 0, 0);
        }
    }

    fn render_camera_meshes(&self, commands: &[SceneCommand], shadow_address: u64) {
        // Opaque depth is established before blending; transparent geometry
        // tests against it but never hides later transparent layers with a write.
        let mut transparent = Vec::new();
        for command in commands {
            if let SceneCommand::Mesh(draw) = command {
                if draw.transparency == 0.0 {
                    self.render_scene_mesh(draw, shadow_address);
                } else if draw.transparency < 1.0 {
                    transparent.push(draw);
                }
            }
        }
        // Stable ties preserve submission order. Sort using each draw's captured
        // camera-space bounds center, rather than the last camera bound.
        transparent.sort_by(|a, b| b.sort_depth.total_cmp(&a.sort_depth));
        for draw in transparent {
            self.render_scene_mesh(draw, shadow_address);
        }
    }

    fn render_scene(&self, shadow_address: u64) {
        let mut start = 0;
        for (index, command) in self.commands.iter().enumerate() {
            if let SceneCommand::Background { constants, texture, destination } = command {
                // A background marks the next camera's draw group. Never sort
                // geometry across cameras or overwrite the previous viewport early.
                self.render_camera_meshes(&self.commands[start..index], shadow_address);
                self.set_viewport(*destination);
                unsafe {
                    self.command_list.SetGraphicsRootSignature(self.background_pipeline.root_signature());
                    self.command_list.SetPipelineState(self.background_pipeline.pipeline_state());
                    self.command_list.SetGraphicsRootConstantBufferView(0, *constants);
                    self.command_list.SetGraphicsRootDescriptorTable(1, *texture);
                    self.command_list.DrawInstanced(3, 1, 0, 0);
                }
                start = index + 1;
            }
        }
        self.render_camera_meshes(&self.commands[start..], shadow_address);
    }

    pub(crate) fn draw_mesh(
        &mut self,
        device: &D3D12Device,
        mesh: &Mesh,
        material: &Material,
        transform: &Transform,
        transparency: f32,
    ) -> Result<()> {
        crate::graphics::shadow::validate_transparency(transparency)
            .map_err(|error| invalid_shadow_order(&error.to_string()))?;
        let view_address = self.view_address.ok_or_else(|| invalid_shadow_order("bind a camera before drawing a mesh"))?;
        let destination = self.destination.unwrap();
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

        let normal_descriptor_index = if let Some(&index) =
            self.normal_descriptor_cache.get(&normal_texture_id)
        {
            index
        } else {
            let index = self.next_texture_descriptor;
            assert!(index < MAX_TEXTURES);
            self.next_texture_descriptor += 1;
            let texture = &self.texture_cache[&normal_texture_id];
            let format = if texture.format() == DXGI_FORMAT_R8G8B8A8_UNORM_SRGB {
                DXGI_FORMAT_R8G8B8A8_UNORM
            } else {
                texture.format()
            };
            self.create_texture_view(device, texture, index, format);
            self.normal_descriptor_cache.insert(normal_texture_id, index);
            index
        };

        // t0 and t1 must be consecutive in the descriptor table. Cache one
        // stable descriptor pair for each base/normal texture combination.
        let pair_key = (texture_id, normal_texture_id);
        let descriptor_index = if let Some(&index) = self.material_descriptor_cache.get(&pair_key) {
            index
        } else {
            let index = self.next_texture_descriptor;
            assert!(
                index + 1 < MAX_TEXTURES,
                "Oxide exceeded the temporary GPU texture limit"
            );
            self.next_texture_descriptor += 2;
            self.copy_texture_descriptor(device, base_descriptor_index, index);
            self.copy_texture_descriptor(device, normal_descriptor_index, index + 1);
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
            base_color: [base_color.r, base_color.g, base_color.b, base_color.a],
            material: [
                material.metallic(),
                material.roughness(),
                material.emissive_strength(),
                material.normal_strength(),
            ],
            emissive: [emissive.r, emissive.g, emissive.b, emissive.a],
            transparency: [transparency, 0.0, 0.0, 0.0],
        };

        let model_address = self.constant_buffer.write(&object_constants);

        let mesh_key = (mesh.id(), mesh.vertex_generation(), mesh.index_generation());
        if !self.mesh_cache.contains_key(&mesh_key) {
            self.mesh_cache.insert(mesh_key, CachedMesh::new(device, mesh)?);
        }
        self.latest_mesh_versions.insert(mesh.id(), (mesh.vertex_generation(), mesh.index_generation()));

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

        let active_lights = self.active_lights.clone();
        for light in &active_lights {
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

                        color: [color.r, color.g, color.b, self.request_shadow(ActiveLight::Point(*light))?],
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

                        color: [color.r, color.g, color.b, self.request_shadow(ActiveLight::Spot(*light))?],
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
                        direction: [direction.x, direction.y, direction.z, self.request_shadow(ActiveLight::Directional(*light))?],

                        color: [color.r, color.g, color.b, light.intensity()],
                    };

                    light_constants.directional_light_count += 1;
                }
            }
        }

        let light_address = self.constant_buffer.write(&light_constants);

        let center = model_matrix.transform_point(self.mesh_cache[&mesh_key].center);
        let sort_depth = -self.camera_view.unwrap().transform_point(center).z;
        self.commands.push(SceneCommand::Mesh(QueuedMesh {
            mesh: mesh_key, model: model_matrix, transparency, sort_depth, destination,
            view: view_address, object: model_address, lights: light_address,
            material: gpu_handle, environment: environment_handle,
        }));

        Ok(())
    }

    pub(crate) fn end_frame(
        &mut self,
        device: &D3D12Device,
        swap_chain: &D3D12SwapChain,
    ) -> Result<()> {
        if !self.background_drawn {
            self.draw_background(device)?;
        }
        let shadow_address = self.render_shadows(device)?;
        let scene_rtv = self.hdr_target.as_ref().unwrap().rtv;
        let scene_dsv = swap_chain.current_dsv();
        unsafe { self.command_list.OMSetRenderTargets(1, Some(&scene_rtv), false, Some(&scene_dsv)); }
        self.render_scene(shadow_address);
        let target = self.hdr_target.as_ref().unwrap();
        let render_target = swap_chain.current_render_target();
        let rtv = swap_chain.current_rtv();
        // Unbind HDR before sampling it. The depth buffer is not used by the display pass.
        unsafe {
            self.command_list.OMSetRenderTargets(0, None, false, None);
        }
        transition_resource(&self.command_list, &target.resource,
            D3D12_RESOURCE_STATE_RENDER_TARGET, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE);
        transition_resource(&self.command_list, render_target,
            D3D12_RESOURCE_STATE_PRESENT, D3D12_RESOURCE_STATE_RENDER_TARGET);
        unsafe {
            self.command_list.OMSetRenderTargets(1, Some(&rtv), false, None);
        }
        let display_address = self.constant_buffer.write(&[self.exposure, 0.0_f32, 0.0, 0.0]);
        unsafe {
            self.command_list.SetGraphicsRootSignature(self.display_pipeline.root_signature());
            self.command_list.SetPipelineState(self.display_pipeline.pipeline_state());
            self.command_list.SetGraphicsRootConstantBufferView(0, display_address);
            self.command_list.SetGraphicsRootDescriptorTable(1, self.texture_heap.GetGPUDescriptorHandleForHeapStart());
            self.command_list.RSSetViewports(&[D3D12_VIEWPORT {
                Width: target.width as f32, Height: target.height as f32, MinDepth: 0.0, MaxDepth: 1.0,
                ..Default::default()
            }]);
            self.command_list.RSSetScissorRects(&[RECT { left: 0, top: 0, right: target.width as i32, bottom: target.height as i32 }]);
            self.command_list.DrawInstanced(3, 1, 0, 0);
        }


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
        self.commands.clear();
        self.mesh_cache.retain(|(id, vertices, indices), _| {
            self.latest_mesh_versions.get(id) == Some(&(*vertices, *indices))
        });


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

    pub(crate) fn bind_camera(
        &mut self,
        device: &D3D12Device,
        camera: &Camera,
        destination: Rect,
    ) -> Result<()> {
        // Flush a background-only viewport before replacing its camera and scissor.
        if !self.background_drawn {
            self.draw_background(device)?;
        }
        self.background_drawn = false;
        assert!(
            destination.width > 0.0 && destination.height > 0.0,
            "camera destination must have positive dimensions"
        );

        let view = camera.view_matrix();
        self.camera_view = Some(view);

        let projection = camera.projection_matrix(destination.aspect_ratio());

        let position = camera.position();
        let rotation = camera.rotation();
        let right = rotation.rotate_vector(crate::math::Vec3::new(1.0, 0.0, 0.0));
        let up = rotation.rotate_vector(crate::math::Vec3::new(0.0, 1.0, 0.0));
        let forward = rotation.rotate_vector(crate::math::Vec3::new(0.0, 0.0, -1.0));
        self.pending_background = Some(BackgroundConstants {
            fallback_color: [0.0; 4],
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
        self.view_address = Some(view_address);

        self.destination = Some(destination);

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
        let Some(mut constants) = self.pending_background else {
            return Ok(());
        };
        let texture = self.environment.as_ref()
            .and_then(|environment| environment.background_texture.as_ref());
        constants.projection_params[2] = texture.is_some() as u8 as f32;
        let color = self.environment.as_ref().map(|environment| environment.background_color)
            .unwrap_or(crate::graphics::Color::rgba(0.10, 0.15, 0.25, 1.0));
        constants.fallback_color = [color.r, color.g, color.b, color.a];
        let texture = texture.unwrap_or(&self.fallback_environment_texture);

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

        self.commands.push(SceneCommand::Background {
            constants: address, texture: handle, destination: self.destination.unwrap(),
        });
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
            if !self.specular_cache.contains_key(&id) {
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
                self.specular_cache.insert(id, specular);

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
                let gpu = D3D12Texture::new(device, &self.command_list, texture, index)?;
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
        let descriptor_index = if let Some(&index) = self.environment_descriptor_cache.get(&key) {
            index
        } else {
            let index = self.next_texture_descriptor;
            assert!(index + 2 < MAX_TEXTURES);
            self.next_texture_descriptor += 3;

            let fallback_descriptor = self.texture_cache[&fallback_id].descriptor_index();
            let specular_descriptor = environment_id
                .map(|id| self.specular_cache[&id].descriptor_index())
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

        let mut handle = unsafe { self.texture_heap.GetGPUDescriptorHandleForHeapStart() };
        handle.ptr += descriptor_index as u64 * self.texture_descriptor_size as u64;
        Ok(handle)
    }

    fn copy_texture_descriptor(
        &self,
        device: &D3D12Device,
        source_index: u32,
        destination_index: u32,
    ) {
        use windows::Win32::Graphics::Direct3D12::D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV;

        // Shader-visible heaps are CPU write-only and cannot be copy sources.
        // Keep readable originals and stable descriptor pairs in a CPU-only heap.
        let mut source = unsafe { self.texture_staging_heap.GetCPUDescriptorHandleForHeapStart() };
        source.ptr += source_index as usize * self.texture_descriptor_size as usize;

        let mut destination = unsafe { self.texture_heap.GetCPUDescriptorHandleForHeapStart() };
        destination.ptr += destination_index as usize * self.texture_descriptor_size as usize;

        unsafe {
            if source_index != destination_index {
                let mut cpu_destination = self.texture_staging_heap.GetCPUDescriptorHandleForHeapStart();
                cpu_destination.ptr += destination_index as usize * self.texture_descriptor_size as usize;
                device.device().CopyDescriptorsSimple(
                    1, cpu_destination, source, D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV,
                );
            }
            device.device().CopyDescriptorsSimple(
                1,
                destination,
                source,
                D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV,
            );
        }
    }

    pub(crate) fn exposure(&self) -> f32 {
        self.exposure
    }

    pub(crate) fn set_exposure(&mut self, value: f32) {
        self.exposure = value;
    }

    fn create_texture_srv(&self, device: &D3D12Device, texture: &D3D12Texture) {
        self.create_texture_view(device, texture, texture.descriptor_index(), texture.format());
    }

    fn create_texture_view(
        &self,
        device: &D3D12Device,
        texture: &D3D12Texture,
        index: u32,
        format: windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT,
    ) {
        let mut handle = unsafe { self.texture_staging_heap.GetCPUDescriptorHandleForHeapStart() };
        handle.ptr += index as usize * self.texture_descriptor_size as usize;

        let desc = D3D12_SHADER_RESOURCE_VIEW_DESC {
            Format: format,

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
        self.copy_texture_descriptor(device, index, index);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directional_shadow_pipelines_and_depth_views_are_valid_on_d3d12() {
        let device = D3D12Device::warp_for_test().unwrap();
        // Creating the real PSOs checks shader/root-signature compatibility,
        // including comparison sampling and the depth-only output layout.
        let mut renderer = D3D12Renderer::new(&device).unwrap();
        let mut srv = unsafe { renderer.texture_heap.GetCPUDescriptorHandleForHeapStart() };
        srv.ptr += renderer.texture_descriptor_size as usize;
        for resolution in [128, 1024] {
            let map = ShadowMap::new(&device, resolution, srv).unwrap();
            assert_eq!(map.resolution, resolution);
            assert_eq!(unsafe { map.resource.GetDesc() }.Width, resolution as u64);
        }
        let light = DirectionalLight::new(crate::math::Vec3::new(0.0, -1.0, 0.0), crate::graphics::Color::WHITE, 1.0).unwrap();
        let _ = light;
        let layers = TransmissionMap::new(&device, 128, 1, renderer.shadow_cpu(1 + MAX_SHADOW_LIGHTS), renderer.shadow_cpu(1 + 2 * MAX_SHADOW_LIGHTS), renderer.shadow_cpu(1 + 3 * MAX_SHADOW_LIGHTS), renderer.texture_descriptor_size as usize).unwrap();
        assert_eq!(layers.dsvs.len(), TRANSMISSION_LAYERS as usize);
    }
}

#[cfg(test)]
#[path = "renderer_tests.rs"]
mod render_tests;
