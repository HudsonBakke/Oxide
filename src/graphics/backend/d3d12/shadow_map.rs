use windows::{
    core::Result,
    Win32::Graphics::{
        Direct3D12::*,
        Dxgi::Common::{DXGI_FORMAT_D32_FLOAT, DXGI_FORMAT_R32_FLOAT, DXGI_FORMAT_R32_TYPELESS, DXGI_SAMPLE_DESC},
    },
};

use super::D3D12Device;

pub(crate) struct ShadowMap {
    pub resource: ID3D12Resource,
    _dsv_heap: ID3D12DescriptorHeap,
    pub dsv: D3D12_CPU_DESCRIPTOR_HANDLE,
    pub resolution: u32,
}

impl ShadowMap {
    // Also creates a valid null descriptor before any shadow pass is requested.
    pub fn write_srv(device: &D3D12Device, resource: Option<&ID3D12Resource>, handle: D3D12_CPU_DESCRIPTOR_HANDLE) {
        let desc = D3D12_SHADER_RESOURCE_VIEW_DESC {
            Format: DXGI_FORMAT_R32_FLOAT,
            ViewDimension: D3D12_SRV_DIMENSION_TEXTURE2D,
            Shader4ComponentMapping: D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING,
            Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
                Texture2D: D3D12_TEX2D_SRV { MipLevels: 1, ..Default::default() },
            },
        };
        unsafe { device.device().CreateShaderResourceView(resource, Some(&desc), handle); }
    }

    pub fn new(device: &D3D12Device, resolution: u32, srv: D3D12_CPU_DESCRIPTOR_HANDLE) -> Result<Self> {
        let desc = D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_TEXTURE2D,
            Width: resolution as u64,
            Height: resolution,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: DXGI_FORMAT_R32_TYPELESS,
            SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Flags: D3D12_RESOURCE_FLAG_ALLOW_DEPTH_STENCIL,
            ..Default::default()
        };
        let clear = D3D12_CLEAR_VALUE {
            Format: DXGI_FORMAT_D32_FLOAT,
            Anonymous: D3D12_CLEAR_VALUE_0 {
                DepthStencil: D3D12_DEPTH_STENCIL_VALUE { Depth: 1.0, Stencil: 0 },
            },
        };
        let mut resource = None;
        unsafe {
            device.device().CreateCommittedResource(
                &D3D12_HEAP_PROPERTIES { Type: D3D12_HEAP_TYPE_DEFAULT, ..Default::default() },
                D3D12_HEAP_FLAG_NONE, &desc, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE,
                Some(&clear), &mut resource,
            )?;
        }
        let resource: ID3D12Resource = resource.unwrap();
        let heap: ID3D12DescriptorHeap = unsafe {
            device.device().CreateDescriptorHeap(&D3D12_DESCRIPTOR_HEAP_DESC {
                Type: D3D12_DESCRIPTOR_HEAP_TYPE_DSV, NumDescriptors: 1, ..Default::default()
            })?
        };
        let dsv = unsafe { heap.GetCPUDescriptorHandleForHeapStart() };
        unsafe {
            device.device().CreateDepthStencilView(&resource, Some(&D3D12_DEPTH_STENCIL_VIEW_DESC {
                Format: DXGI_FORMAT_D32_FLOAT,
                ViewDimension: D3D12_DSV_DIMENSION_TEXTURE2D,
                ..Default::default()
            }), dsv);
        }
        Self::write_srv(device, Some(&resource), srv);
        Ok(Self { resource, _dsv_heap: heap, dsv, resolution })
    }
}
