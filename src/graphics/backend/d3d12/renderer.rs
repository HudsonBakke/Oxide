use windows::{
    core::{Interface, Result},
    Win32::Graphics::Direct3D12::{
        ID3D12CommandAllocator,
        ID3D12CommandList,
        ID3D12GraphicsCommandList,
        D3D12_COMMAND_LIST_TYPE_DIRECT,
        D3D12_RESOURCE_BARRIER,
        D3D12_RESOURCE_BARRIER_0,
        D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES,
        D3D12_RESOURCE_BARRIER_FLAG_NONE,
        D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
        D3D12_RESOURCE_STATE_PRESENT,
        D3D12_RESOURCE_STATE_RENDER_TARGET,
        D3D12_RESOURCE_TRANSITION_BARRIER,
        ID3D12Resource,
        D3D12_RESOURCE_STATES,
    },
    Win32::{
        Foundation::{CloseHandle, HANDLE, RECT},
        Graphics::{
            Direct3D::D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST,
            Direct3D12::{
                D3D12_VIEWPORT,
                ID3D12Fence,
                D3D12_FENCE_FLAG_NONE,
            },
        },
        System::Threading::{
            CreateEventW,
            WaitForSingleObject,
            INFINITE,
        },
    },
};

use std::collections::HashMap;

use super::{
    D3D12Device,
    D3D12Pipeline,
    D3D12Shader,
    D3D12SwapChain,
    D3D12VertexBuffer,
    D3D12IndexBuffer,
    D3D12ConstantBuffer,
    swap_chain::FRAME_COUNT,
};

use crate::{
    graphics::{
        GraphicsResult,
        Mesh,
        mesh::MeshId,
    },
    math::Transform,
};

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
            Transition: std::mem::ManuallyDrop::new(
                D3D12_RESOURCE_TRANSITION_BARRIER {
                    pResource: std::mem::ManuallyDrop::new(
                        Some(resource.clone())
                    ),
                    Subresource: D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES,
                    StateBefore: before,
                    StateAfter: after,
                },
            ),
        },
    };

    unsafe {
        command_list.ResourceBarrier(
            std::slice::from_ref(&barrier)
        );

        std::mem::ManuallyDrop::drop(
            &mut (*barrier.Anonymous.Transition).pResource
        );
    }
}

struct CachedMesh {
    vertex_generation: u64,
    index_generation: u64,

    vertex_buffer: D3D12VertexBuffer,
    index_buffer: D3D12IndexBuffer,
}

impl CachedMesh {
    fn new(
        device: &D3D12Device,
        mesh: &Mesh,
    ) -> Result<Self> {
        Ok(Self {
            vertex_generation: mesh.vertex_generation(),
            index_generation: mesh.index_generation(),

            vertex_buffer: D3D12VertexBuffer::new(
                device,
                mesh.vertices(),
            )?,

            index_buffer: D3D12IndexBuffer::new(
                device,
                mesh.indices(),
            )?,
        })
    }

    fn synchronize(
        &mut self,
        device: &D3D12Device,
        mesh: &Mesh,
    ) -> Result<()> {
        if self.vertex_generation != mesh.vertex_generation() {
            self.vertex_buffer = D3D12VertexBuffer::new(
                device,
                mesh.vertices(),
            )?;

            self.vertex_generation = mesh.vertex_generation();
        }

        if self.index_generation != mesh.index_generation() {
            self.index_buffer = D3D12IndexBuffer::new(
                device,
                mesh.indices(),
            )?;

            self.index_generation = mesh.index_generation();
        }

        Ok(())
    }
}

pub(crate) struct D3D12Renderer {
    command_allocators: [ID3D12CommandAllocator; FRAME_COUNT],
    command_list: ID3D12GraphicsCommandList,

    pipeline: D3D12Pipeline,

    mesh_cache: HashMap<MeshId, CachedMesh>,

    fence: ID3D12Fence,
    fence_value: u64,
    fence_event: HANDLE,

    model_constant_buffer: D3D12ConstantBuffer,
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

        let command_allocators = [
            allocator_0,
            allocator_1,
        ];

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

        let fence: ID3D12Fence = unsafe {
            device.device().CreateFence(
                0,
                D3D12_FENCE_FLAG_NONE,
            )?
        };

        let fence_event = unsafe {
            CreateEventW(
                None,
                false,
                false,
                None,
            )?
        };

        let vertex_shader = D3D12Shader::load(
            "shaders/compiled/triangle_vs.dxil"
        )?;

        let pixel_shader = D3D12Shader::load(
            "shaders/compiled/triangle_ps.dxil"
        )?;

        let pipeline = D3D12Pipeline::new(
            device,
            &vertex_shader,
            &pixel_shader,
        )?;

        let model_constant_buffer =
            D3D12ConstantBuffer::new(device)?;

        Ok(Self {
            command_allocators,
            command_list,

            pipeline,

            mesh_cache: HashMap::new(),

            fence,
            fence_value: 0,
            fence_event,

            model_constant_buffer,
        })
    }

    pub(crate) fn begin_frame(
        &mut self,
        _device: &D3D12Device,
        swap_chain: &D3D12SwapChain,
    ) -> Result<()> {
        let frame_index = swap_chain.current_frame_index();
        let command_allocator = &self.command_allocators[frame_index];

        unsafe {
            command_allocator.Reset()?;

            self.command_list.Reset(
                command_allocator,
                None,
            )?;

            self.command_list.SetGraphicsRootSignature(
                self.pipeline.root_signature()
            );

            self.command_list.SetPipelineState(
                self.pipeline.pipeline_state()
            );
        }

        let render_target = swap_chain.current_render_target();

        transition_resource(
            &self.command_list,
            render_target,
            D3D12_RESOURCE_STATE_PRESENT,
            D3D12_RESOURCE_STATE_RENDER_TARGET,
        );

        let viewport = D3D12_VIEWPORT {
            TopLeftX: 0.0,
            TopLeftY: 0.0,
            Width: swap_chain.width() as f32,
            Height: swap_chain.height() as f32,
            MinDepth: 0.0,
            MaxDepth: 1.0,
        };

        let scissor = RECT {
            left: 0,
            top: 0,
            right: swap_chain.width() as i32,
            bottom: swap_chain.height() as i32,
        };

        let clear_color = [
            0.10,
            0.15,
            0.25,
            1.0,
        ];

        let rtv = swap_chain.current_rtv();

        unsafe {
            self.command_list.RSSetViewports(&[
                viewport
            ]);

            self.command_list.RSSetScissorRects(&[
                scissor
            ]);

            self.command_list.ClearRenderTargetView(
                rtv,
                &clear_color,
                None,
            );

            self.command_list.OMSetRenderTargets(
                1,
                Some(&rtv),
                false,
                None,
            );

            self.command_list.IASetPrimitiveTopology(
                D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST
            );
        }

        Ok(())
    }

    pub(crate) fn draw_mesh(
        &mut self,
        device: &D3D12Device,
        mesh: &Mesh,
        transform: &Transform,
    ) -> Result<()> {
        let model_matrix =
            transform.matrix_around(mesh.pivot());

        self.model_constant_buffer
            .write_matrix(&model_matrix);

        let mesh_id = mesh.id();

        if !self.mesh_cache.contains_key(&mesh_id) {
            let cached = CachedMesh::new(
                device,
                mesh,
            )?;

            self.mesh_cache.insert(
                mesh_id,
                cached,
            );
        }

        let cached = self
            .mesh_cache
            .get_mut(&mesh_id)
            .expect("mesh should be cached after insertion");

        cached.synchronize(
            device,
            mesh,
        )?;

        unsafe {
            self.command_list.IASetVertexBuffers(
                0,
                Some(std::slice::from_ref(
                    cached.vertex_buffer.view()
                )),
            );

            self.command_list.IASetIndexBuffer(
                Some(cached.index_buffer.view())
            );

            self.command_list.SetGraphicsRootConstantBufferView(
                0,
                self.model_constant_buffer.gpu_address(),
            );

            self.command_list.DrawIndexedInstanced(
                cached.index_buffer.index_count(),
                1,
                0,
                0,
                0,
            );
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

        let command_list: ID3D12CommandList =
            self.command_list.cast()?;

        unsafe {
            device
                .command_queue()
                .ExecuteCommandLists(&[
                    Some(command_list)
                ]);
        }

        swap_chain.present()?;

        self.wait_for_gpu(device)?;

        Ok(())
    }

    pub(crate) fn wait_for_gpu(
        &mut self,
        device: &D3D12Device,
    ) -> Result<()> {
        self.fence_value += 1;

        let value = self.fence_value;

        unsafe {
            device
                .command_queue()
                .Signal(&self.fence, value)?;
        }

        if unsafe {
            self.fence.GetCompletedValue()
        } < value
        {
            unsafe {
                self.fence.SetEventOnCompletion(
                    value,
                    self.fence_event,
                )?;

                WaitForSingleObject(
                    self.fence_event,
                    INFINITE,
                );
            }
        }

        Ok(())
    }

    pub(crate) fn prepare_for_resize(
        &mut self,
        device: &D3D12Device,
    ) -> Result<()> {
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
            self.command_list.Reset(
                &self.command_allocators[0],
                None,
            )?;

            self.command_list.Close()?;
        }

        Ok(())
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