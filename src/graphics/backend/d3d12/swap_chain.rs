use windows::{
    core::{Interface, Result},
    Win32::{
        Foundation::HWND,
        Graphics::{
            Direct3D12::{
                ID3D12DescriptorHeap,
                ID3D12Resource,
                D3D12_CPU_DESCRIPTOR_HANDLE,
                D3D12_DESCRIPTOR_HEAP_DESC,
                D3D12_DESCRIPTOR_HEAP_FLAG_NONE,
                D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
                D3D12_DESCRIPTOR_HEAP_TYPE_DSV,
                D3D12_CLEAR_VALUE,
                D3D12_DEPTH_STENCIL_VIEW_DESC,
                D3D12_DSV_DIMENSION_TEXTURE2D,
                D3D12_DSV_FLAG_NONE,
                D3D12_HEAP_PROPERTIES,
                D3D12_HEAP_TYPE_DEFAULT,
                D3D12_RESOURCE_DESC,
                D3D12_RESOURCE_DIMENSION_TEXTURE2D,
                D3D12_RESOURCE_FLAG_ALLOW_DEPTH_STENCIL,
                D3D12_RESOURCE_STATE_DEPTH_WRITE,
                D3D12_TEXTURE_LAYOUT_UNKNOWN,
            },
            Dxgi::{
                Common::{
                    DXGI_FORMAT_R8G8B8A8_UNORM,
                    DXGI_FORMAT_D32_FLOAT,
                    DXGI_SAMPLE_DESC,
                },
                CreateDXGIFactory2,
                DXGI_CREATE_FACTORY_DEBUG,
                IDXGIFactory4,
                IDXGISwapChain1,
                IDXGISwapChain3,
                DXGI_CREATE_FACTORY_FLAGS,
                DXGI_SCALING_STRETCH,
                DXGI_SWAP_CHAIN_DESC1,
                DXGI_SWAP_EFFECT_FLIP_DISCARD,
                DXGI_USAGE_RENDER_TARGET_OUTPUT,
                DXGI_PRESENT,
                DXGI_SWAP_CHAIN_FLAG,
            },
        },
    },
};

use super::D3D12Device;

pub(crate) const FRAME_COUNT: usize = 2;

pub(crate) struct D3D12SwapChain {
    swap_chain: IDXGISwapChain3,
    render_targets: [Option<ID3D12Resource>; FRAME_COUNT],
    rtv_heap: ID3D12DescriptorHeap,
    rtv_descriptor_size: u32,
    depth_buffer: ID3D12Resource,
    dsv_heap: ID3D12DescriptorHeap,

    width: u32,
    height: u32,
}

impl D3D12SwapChain {
    fn create_depth_buffer(
        device: &D3D12Device,
        width: u32,
        height: u32,
    ) -> Result<(
        ID3D12Resource,
        ID3D12DescriptorHeap,
    )> {
        let heap_properties = D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_DEFAULT,
            ..Default::default()
        };

        let resource_desc = D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_TEXTURE2D,
            Alignment: 0,
            Width: width as u64,
            Height: height,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: DXGI_FORMAT_D32_FLOAT,

            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },

            Layout: D3D12_TEXTURE_LAYOUT_UNKNOWN,

            Flags:
                D3D12_RESOURCE_FLAG_ALLOW_DEPTH_STENCIL,
        };

        let clear_value = D3D12_CLEAR_VALUE {
            Format: DXGI_FORMAT_D32_FLOAT,

            Anonymous:
                windows::Win32::Graphics::Direct3D12::
                    D3D12_CLEAR_VALUE_0 {
                        DepthStencil:
                            windows::Win32::Graphics::Direct3D12::
                                D3D12_DEPTH_STENCIL_VALUE {
                                    Depth: 1.0,
                                    Stencil: 0,
                                },
                    },
        };

        let mut depth_buffer = None;

        unsafe {
            device.device().CreateCommittedResource(
                &heap_properties,
                Default::default(),
                &resource_desc,
                D3D12_RESOURCE_STATE_DEPTH_WRITE,
                Some(&clear_value),
                &mut depth_buffer,
            )?;
        }

        let depth_buffer = depth_buffer.expect(
            "CreateCommittedResource succeeded but returned no depth buffer"
        );

        let dsv_heap_desc = D3D12_DESCRIPTOR_HEAP_DESC {
            Type: D3D12_DESCRIPTOR_HEAP_TYPE_DSV,
            NumDescriptors: 1,
            Flags: D3D12_DESCRIPTOR_HEAP_FLAG_NONE,
            NodeMask: 0,
        };

        let dsv_heap: ID3D12DescriptorHeap = unsafe {
            device
                .device()
                .CreateDescriptorHeap(
                    &dsv_heap_desc
                )?
        };

        let dsv = unsafe {
            dsv_heap
                .GetCPUDescriptorHandleForHeapStart()
        };

        let dsv_desc = D3D12_DEPTH_STENCIL_VIEW_DESC {
            Format: DXGI_FORMAT_D32_FLOAT,
            ViewDimension:
                D3D12_DSV_DIMENSION_TEXTURE2D,
            Flags: D3D12_DSV_FLAG_NONE,
            ..Default::default()
        };

        unsafe {
            device.device().CreateDepthStencilView(
                &depth_buffer,
                Some(&dsv_desc),
                dsv,
            );
        }

        Ok((
            depth_buffer,
            dsv_heap,
        ))
    }

    pub(crate) fn new(
        device: &D3D12Device,
        hwnd: HWND,
        width: u32,
        height: u32,
    ) -> Result<Self> {
        let factory_flags = if cfg!(debug_assertions) {
            DXGI_CREATE_FACTORY_DEBUG
        } else {
            DXGI_CREATE_FACTORY_FLAGS(0)
        };

        let factory: IDXGIFactory4 = unsafe {
            CreateDXGIFactory2(factory_flags)?
        };

        let description = DXGI_SWAP_CHAIN_DESC1 {
            Width: width,
            Height: height,
            Format: DXGI_FORMAT_R8G8B8A8_UNORM,
            Stereo: false.into(),
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
            BufferCount: FRAME_COUNT as u32,
            Scaling: DXGI_SCALING_STRETCH,
            SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
            AlphaMode: Default::default(),
            Flags: 0,
        };

        let swap_chain: IDXGISwapChain1 = unsafe {
            factory.CreateSwapChainForHwnd(
                device.command_queue(),
                hwnd,
                &description,
                None,
                None,
            )?
        };

        let swap_chain: IDXGISwapChain3 = swap_chain.cast()?;

        let rtv_heap_desc = D3D12_DESCRIPTOR_HEAP_DESC {
            Type: D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
            NumDescriptors: FRAME_COUNT as u32,
            Flags: D3D12_DESCRIPTOR_HEAP_FLAG_NONE,
            NodeMask: 0,
        };

        let rtv_heap: ID3D12DescriptorHeap = unsafe {
            device.device().CreateDescriptorHeap(&rtv_heap_desc)?
        };

        let rtv_descriptor_size = unsafe {
            device
                .device()
                .GetDescriptorHandleIncrementSize(
                    D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
                )
        };

        let render_target_0: ID3D12Resource = unsafe {
            swap_chain.GetBuffer(0)?
        };

        let render_target_1: ID3D12Resource = unsafe {
            swap_chain.GetBuffer(1)?
        };

        let render_targets = [
            Some(render_target_0),
            Some(render_target_1),
        ];

        let (
            depth_buffer,
            dsv_heap,
        ) = Self::create_depth_buffer(
            device,
            width,
            height,
        )?;

        let rtv_start = unsafe {
            rtv_heap.GetCPUDescriptorHandleForHeapStart()
        };

        for i in 0..FRAME_COUNT {
            let handle = D3D12_CPU_DESCRIPTOR_HANDLE {
                ptr: rtv_start.ptr
                    + i * rtv_descriptor_size as usize,
            };

            unsafe {
                device.device().CreateRenderTargetView(
                    render_targets[i].as_ref(),
                    None,
                    handle,
                );
            }
        }

        Ok(Self {
            swap_chain,
            render_targets,
            rtv_heap,
            rtv_descriptor_size,
            depth_buffer,
            dsv_heap,
            width,
            height,
        })
    }

    pub(crate) fn current_frame_index(&self) -> usize {
        unsafe {
            self.swap_chain.GetCurrentBackBufferIndex() as usize
        }
    }

    pub(crate) fn current_render_target(&self) -> &ID3D12Resource {
        self.render_targets[self.current_frame_index()]
            .as_ref()
            .expect("swap chain back buffer is unavailable")
    }

    pub(crate) fn current_rtv(
        &self,
    ) -> D3D12_CPU_DESCRIPTOR_HANDLE {
        let index = self.current_frame_index();

        let start = unsafe {
            self.rtv_heap.GetCPUDescriptorHandleForHeapStart()
        };

        D3D12_CPU_DESCRIPTOR_HANDLE {
            ptr: start.ptr
                + index * self.rtv_descriptor_size as usize,
        }
    }

    pub(crate) fn present(&self) -> Result<()> {
        unsafe {
            self.swap_chain
                .Present(1, DXGI_PRESENT(0))
                .ok()?;
        }

        Ok(())
    }

    fn create_render_target_views(
        &mut self,
        device: &D3D12Device,
    ) -> Result<()> {
        let start = unsafe {
            self.rtv_heap.GetCPUDescriptorHandleForHeapStart()
        };

        for i in 0..FRAME_COUNT {
            let resource: ID3D12Resource = unsafe {
                self.swap_chain.GetBuffer(i as u32)?
            };

            let handle = D3D12_CPU_DESCRIPTOR_HANDLE {
                ptr: start.ptr
                    + i * self.rtv_descriptor_size as usize,
            };

            unsafe {
                device.device().CreateRenderTargetView(
                    &resource,
                    None,
                    handle,
                );
            }

            self.render_targets[i] = Some(resource);
        }

        Ok(())
    }

    pub(crate) fn resize(
        &mut self,
        device: &D3D12Device,
        width: u32,
        height: u32,
    ) -> Result<()> {
        // A minimized window can have a zero-sized client area.
        if width == 0 || height == 0 {
            return Ok(());
        }

        // Don't rebuild the swap chain if its dimensions have not changed.
        if width == self.width && height == self.height {
            return Ok(());
        }

        // Release every reference we own to the old swap-chain buffers.
        for render_target in &mut self.render_targets {
            *render_target = None;
        }

        unsafe {
            self.swap_chain.ResizeBuffers(
                FRAME_COUNT as u32,
                width,
                height,
                DXGI_FORMAT_R8G8B8A8_UNORM,
                DXGI_SWAP_CHAIN_FLAG(0),
            )?;
        }

        let (
            depth_buffer,
            dsv_heap,
        ) = Self::create_depth_buffer(
            device,
            width,
            height,
        )?;

        self.depth_buffer = depth_buffer;
        self.dsv_heap = dsv_heap;

        // Only update our stored dimensions AFTER ResizeBuffers succeeds.
        self.width = width;
        self.height = height;

        self.create_render_target_views(device)?;

        Ok(())
    }

    pub(crate) fn current_dsv(
        &self,
    ) -> D3D12_CPU_DESCRIPTOR_HANDLE {
        unsafe {
            self.dsv_heap
                .GetCPUDescriptorHandleForHeapStart()
        }
    }

    pub(crate) fn width(&self) -> u32 {
        self.width
    }

    pub(crate) fn height(&self) -> u32 {
        self.height
    }
}