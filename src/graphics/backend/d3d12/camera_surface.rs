use windows::{core::Result, Win32::Graphics::{Direct3D12::*, Dxgi::Common::*}};
use super::{D3D12Device, render_target::HdrTarget};

/// Reusable HDR camera image and its independent depth buffer.
pub(crate) struct CameraSurfaceTarget {
    pub color: HdrTarget,
    pub _depth: ID3D12Resource,
    _dsv_heap: ID3D12DescriptorHeap,
    pub dsv: D3D12_CPU_DESCRIPTOR_HANDLE,
    pub descriptor: u32,
}
impl CameraSurfaceTarget {
    pub fn new(device: &D3D12Device, width: u32, height: u32, srv: D3D12_CPU_DESCRIPTOR_HANDLE, descriptor: u32) -> Result<Self> {
        // Create every fallible depth allocation before replacing the SRV slot.
        let mut depth = None;
        unsafe {
            device.device().CreateCommittedResource(
                &D3D12_HEAP_PROPERTIES { Type: D3D12_HEAP_TYPE_DEFAULT, ..Default::default() },
                D3D12_HEAP_FLAG_NONE,
                &D3D12_RESOURCE_DESC {
                    Dimension: D3D12_RESOURCE_DIMENSION_TEXTURE2D, Width: width as u64,
                    Height: height, DepthOrArraySize: 1, MipLevels: 1,
                    Format: DXGI_FORMAT_D32_FLOAT,
                    SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
                    Flags: D3D12_RESOURCE_FLAG_ALLOW_DEPTH_STENCIL, ..Default::default()
                }, D3D12_RESOURCE_STATE_DEPTH_WRITE,
                Some(&D3D12_CLEAR_VALUE { Format: DXGI_FORMAT_D32_FLOAT,
                    Anonymous: D3D12_CLEAR_VALUE_0 { DepthStencil: D3D12_DEPTH_STENCIL_VALUE { Depth: 1.0, Stencil: 0 } } }),
                &mut depth,
            )?;
        }
        let depth: ID3D12Resource = depth.unwrap();
        let heap: ID3D12DescriptorHeap = unsafe {
            device.device().CreateDescriptorHeap(&D3D12_DESCRIPTOR_HEAP_DESC {
                Type: D3D12_DESCRIPTOR_HEAP_TYPE_DSV, NumDescriptors: 1, ..Default::default()
            })?
        };
        let dsv = unsafe { heap.GetCPUDescriptorHandleForHeapStart() };
        unsafe { device.device().CreateDepthStencilView(&depth, None, dsv); }
        let color = HdrTarget::new(device, width, height, srv)?;
        Ok(Self { color, _depth: depth, _dsv_heap: heap, dsv, descriptor })
    }
}
