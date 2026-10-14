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
        Foundation::{CloseHandle, HANDLE},
        Graphics::Direct3D12::{
            ID3D12Fence,
            D3D12_FENCE_FLAG_NONE,
        },
        System::Threading::{
            CreateEventW,
            WaitForSingleObject,
            INFINITE,
        },
    },
};

use super::{
    D3D12Device,
    D3D12Shader,
    D3D12SwapChain,
    swap_chain::FRAME_COUNT,
};

use crate::graphics::GraphicsResult;

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

pub(crate) struct D3D12Renderer {
    command_allocators: [ID3D12CommandAllocator; FRAME_COUNT],
    command_list: ID3D12GraphicsCommandList,

    vertex_shader: D3D12Shader,
    pixel_shader: D3D12Shader,

    fence: ID3D12Fence,
    fence_value: u64,
    fence_event: HANDLE,
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

        Ok(Self {
            command_allocators,
            command_list,

            vertex_shader,
            pixel_shader,

            fence,
            fence_value: 0,
            fence_event,
        })
    }

    pub(crate) fn render(
        &mut self,
        device: &D3D12Device,
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
        }

        let render_target = swap_chain.current_render_target();

        transition_resource(
            &self.command_list,
            render_target,
            D3D12_RESOURCE_STATE_PRESENT,
            D3D12_RESOURCE_STATE_RENDER_TARGET,
        );

        let clear_color = [
            0.10,
            0.15,
            0.25,
            1.0,
        ];

        unsafe {
            self.command_list.ClearRenderTargetView(
                swap_chain.current_rtv(),
                &clear_color,
                None,
            );
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