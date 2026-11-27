use windows::{
    core::Result,
    Win32::Graphics::{Direct3D12::*, Dxgi::Common::*},
};

use super::D3D12Device;

pub(crate) const TRANSMISSION_LAYERS: u32 = 8;
pub(crate) const TRANSMISSION_MAX_RESOLUTION: u32 = 1024;
// Heap slots: HDR=0, opaque depth=1, transmission depth=2, values=3,
// previous-depth views=4..12. Ordinary textures start at 12.
pub(crate) const MAX_SHADOW_LIGHTS: usize = 32;
pub(crate) const PREVIOUS_VIEWS_PER_LIGHT: u32 = 6 * TRANSMISSION_LAYERS;
pub(crate) const FIRST_TEXTURE_DESCRIPTOR: u32 = 1 + 3 * MAX_SHADOW_LIGHTS as u32 + MAX_SHADOW_LIGHTS as u32 * PREVIOUS_VIEWS_PER_LIGHT;

pub(crate) struct TransmissionMap {
    pub depth: ID3D12Resource,
    pub values: ID3D12Resource,
    _dsv_heap: ID3D12DescriptorHeap,
    _rtv_heap: ID3D12DescriptorHeap,
    pub dsvs: Vec<D3D12_CPU_DESCRIPTOR_HANDLE>,
    pub rtvs: Vec<D3D12_CPU_DESCRIPTOR_HANDLE>,
    pub resolution: u32,
}

pub(crate) fn array_srv(
    device: &D3D12Device,
    resource: Option<&ID3D12Resource>,
    format: DXGI_FORMAT,
    first_slice: u32,
    count: u32,
    handle: D3D12_CPU_DESCRIPTOR_HANDLE,
) {
    let desc = D3D12_SHADER_RESOURCE_VIEW_DESC {
        Format: format,
        ViewDimension: D3D12_SRV_DIMENSION_TEXTURE2DARRAY,
        Shader4ComponentMapping: D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING,
        Anonymous: D3D12_SHADER_RESOURCE_VIEW_DESC_0 {
            Texture2DArray: D3D12_TEX2D_ARRAY_SRV {
                MipLevels: 1, FirstArraySlice: first_slice, ArraySize: count,
                ..Default::default()
            },
        },
    };
    unsafe { device.device().CreateShaderResourceView(resource, Some(&desc), handle); }
}

impl TransmissionMap {
    pub fn new(device: &D3D12Device, resolution: u32, faces: u32, depth_srv: D3D12_CPU_DESCRIPTOR_HANDLE, values_srv: D3D12_CPU_DESCRIPTOR_HANDLE, mut previous_srv: D3D12_CPU_DESCRIPTOR_HANDLE, stride: usize) -> Result<Self> {
        let layers = faces * TRANSMISSION_LAYERS;
        let create = |format, flags, clear: &D3D12_CLEAR_VALUE| -> Result<ID3D12Resource> {
            let mut resource = None;
            unsafe {
                device.device().CreateCommittedResource(
                    &D3D12_HEAP_PROPERTIES { Type: D3D12_HEAP_TYPE_DEFAULT, ..Default::default() },
                    D3D12_HEAP_FLAG_NONE,
                    &D3D12_RESOURCE_DESC {
                        Dimension: D3D12_RESOURCE_DIMENSION_TEXTURE2D,
                        Width: resolution as u64, Height: resolution,
                        DepthOrArraySize: layers as u16, MipLevels: 1,
                        Format: format, Flags: flags,
                        SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
                        ..Default::default()
                    },
                    D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE, Some(clear), &mut resource,
                )?;
            }
            Ok(resource.unwrap())
        };
        let depth = create(DXGI_FORMAT_R32_TYPELESS, D3D12_RESOURCE_FLAG_ALLOW_DEPTH_STENCIL, &D3D12_CLEAR_VALUE {
            Format: DXGI_FORMAT_D32_FLOAT,
            Anonymous: D3D12_CLEAR_VALUE_0 {
                DepthStencil: D3D12_DEPTH_STENCIL_VALUE { Depth: 1.0, Stencil: 0 },
            },
        })?;
        let values = create(DXGI_FORMAT_R16_FLOAT, D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET, &D3D12_CLEAR_VALUE {
            Format: DXGI_FORMAT_R16_FLOAT,
            Anonymous: D3D12_CLEAR_VALUE_0 { Color: [1.0; 4] },
        })?;
        let dsv_heap: ID3D12DescriptorHeap = unsafe {
            device.device().CreateDescriptorHeap(&D3D12_DESCRIPTOR_HEAP_DESC {
                Type: D3D12_DESCRIPTOR_HEAP_TYPE_DSV, NumDescriptors: layers,
                ..Default::default()
            })?
        };
        let rtv_heap: ID3D12DescriptorHeap = unsafe {
            device.device().CreateDescriptorHeap(&D3D12_DESCRIPTOR_HEAP_DESC {
                Type: D3D12_DESCRIPTOR_HEAP_TYPE_RTV, NumDescriptors: layers,
                ..Default::default()
            })?
        };
        let mut dsv = unsafe { dsv_heap.GetCPUDescriptorHandleForHeapStart() };
        let mut rtv = unsafe { rtv_heap.GetCPUDescriptorHandleForHeapStart() };
        let dsv_stride = unsafe { device.device().GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_DSV) } as usize;
        let rtv_stride = unsafe { device.device().GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_RTV) } as usize;
        let mut dsvs = Vec::new();
        let mut rtvs = Vec::new();
        for layer in 0..layers {
            unsafe {
                device.device().CreateDepthStencilView(&depth, Some(&D3D12_DEPTH_STENCIL_VIEW_DESC {
                    Format: DXGI_FORMAT_D32_FLOAT, ViewDimension: D3D12_DSV_DIMENSION_TEXTURE2DARRAY,
                    Anonymous: D3D12_DEPTH_STENCIL_VIEW_DESC_0 {
                        Texture2DArray: D3D12_TEX2D_ARRAY_DSV {
                            MipSlice: 0, FirstArraySlice: layer, ArraySize: 1,
                        },
                    },
                    ..Default::default()
                }), dsv);
                device.device().CreateRenderTargetView(&values, Some(&D3D12_RENDER_TARGET_VIEW_DESC {
                    Format: DXGI_FORMAT_R16_FLOAT, ViewDimension: D3D12_RTV_DIMENSION_TEXTURE2DARRAY,
                    Anonymous: D3D12_RENDER_TARGET_VIEW_DESC_0 {
                        Texture2DArray: D3D12_TEX2D_ARRAY_RTV {
                            MipSlice: 0, FirstArraySlice: layer, ArraySize: 1, PlaneSlice: 0,
                        },
                    },
                }), rtv);
            }
            dsvs.push(dsv);
            rtvs.push(rtv);
            dsv.ptr += dsv_stride;
            rtv.ptr += rtv_stride;
        }
        array_srv(device, Some(&depth), DXGI_FORMAT_R32_FLOAT, 0, layers, depth_srv);
        array_srv(device, Some(&values), DXGI_FORMAT_R16_FLOAT, 0, layers, values_srv);
        for slice in 0..layers {
            if slice % TRANSMISSION_LAYERS == 0 {
                array_srv(device, None, DXGI_FORMAT_R32_FLOAT, 0, 1, previous_srv);
            } else {
                array_srv(device, Some(&depth), DXGI_FORMAT_R32_FLOAT, slice - 1, 1, previous_srv);
            }
            previous_srv.ptr += stride;
        }
        Ok(Self { depth, values, _dsv_heap: dsv_heap, _rtv_heap: rtv_heap, dsvs, rtvs, resolution })
    }
}
