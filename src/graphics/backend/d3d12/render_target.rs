use windows::{
    core::Result,
    Win32::Graphics::{
        Direct3D12::*,
        Dxgi::Common::{DXGI_FORMAT_R16G16B16A16_FLOAT, DXGI_SAMPLE_DESC},
    },
};

use super::D3D12Device;

/// Linear scene-color buffer. The renderer transitions it between drawing and sampling.
pub(crate) struct HdrTarget {
    pub resource: ID3D12Resource,
    _rtv_heap: ID3D12DescriptorHeap,
    pub rtv: D3D12_CPU_DESCRIPTOR_HANDLE,
    pub width: u32,
    pub height: u32,
}

impl HdrTarget {
    pub fn new(
        device: &D3D12Device,
        width: u32,
        height: u32,
        srv: D3D12_CPU_DESCRIPTOR_HANDLE,
    ) -> Result<Self> {
        let desc = D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_TEXTURE2D,
            Width: u64::from(width),
            Height: height,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: DXGI_FORMAT_R16G16B16A16_FLOAT,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Flags: D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET,
            ..Default::default()
        };
        let mut resource = None;
        unsafe {
            device.device().CreateCommittedResource(
                &D3D12_HEAP_PROPERTIES { Type: D3D12_HEAP_TYPE_DEFAULT, ..Default::default() },
                D3D12_HEAP_FLAG_NONE,
                &desc,
                D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE,
                None,
                &mut resource,
            )?;
        }
        let resource: ID3D12Resource = resource.unwrap();
        let heap: ID3D12DescriptorHeap = unsafe {
            device.device().CreateDescriptorHeap(&D3D12_DESCRIPTOR_HEAP_DESC {
                Type: D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
                NumDescriptors: 1,
                ..Default::default()
            })?
        };
        let rtv = unsafe { heap.GetCPUDescriptorHandleForHeapStart() };
        unsafe {
            device.device().CreateRenderTargetView(&resource, None, rtv);
            device.device().CreateShaderResourceView(
                Some(&resource),
                Some(&D3D12_SHADER_RESOURCE_VIEW_DESC {
                    Format: DXGI_FORMAT_R16G16B16A16_FLOAT,
                    ViewDimension: D3D12_SRV_DIMENSION_TEXTURE2D,
                    Shader4ComponentMapping: D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING,
                    Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                        Texture2D: D3D12_TEX2D_SRV { MipLevels: 1, ..Default::default() },
                    },
                }),
                srv,
            );
        }
        Ok(Self { resource, _rtv_heap: heap, rtv, width, height })
    }
}
