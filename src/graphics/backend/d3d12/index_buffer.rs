use windows::{
    core::Result,
    Win32::Graphics::{
        Direct3D12::{
            D3D12_HEAP_PROPERTIES,
            D3D12_HEAP_TYPE_UPLOAD,
            D3D12_INDEX_BUFFER_VIEW,
            D3D12_RESOURCE_DESC,
            D3D12_RESOURCE_DIMENSION_BUFFER,
            D3D12_RESOURCE_FLAG_NONE,
            D3D12_RESOURCE_STATE_GENERIC_READ,
            D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
            ID3D12Resource,
        },
        Dxgi::Common::{
            DXGI_FORMAT_R32_UINT,
            DXGI_FORMAT_UNKNOWN,
            DXGI_SAMPLE_DESC,
        },
    },
};

use super::D3D12Device;

pub(crate) struct D3D12IndexBuffer {
    // Although this field is not explicitly read after construction,
    // it owns the GPU resource referenced by `view`.
    _resource: ID3D12Resource,

    view: D3D12_INDEX_BUFFER_VIEW,
    index_count: u32,
}

impl D3D12IndexBuffer {
    pub(crate) fn new(
        device: &D3D12Device,
        indices: &[u32],
    ) -> Result<Self> {
        let size = std::mem::size_of_val(indices) as u64;

        let heap_properties = D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_UPLOAD,
            ..Default::default()
        };

        let resource_desc = D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
            Alignment: 0,
            Width: size,
            Height: 1,
            DepthOrArraySize: 1,
            MipLevels: 1,
            Format: DXGI_FORMAT_UNKNOWN,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
            Flags: D3D12_RESOURCE_FLAG_NONE,
        };

        let mut resource: Option<ID3D12Resource> = None;

        unsafe {
            device.device().CreateCommittedResource(
                &heap_properties,
                Default::default(),
                &resource_desc,
                D3D12_RESOURCE_STATE_GENERIC_READ,
                None,
                &mut resource,
            )?;
        }

        let resource = resource.expect(
            "CreateCommittedResource succeeded but returned no index buffer"
        );

        let mut mapped = std::ptr::null_mut();

        unsafe {
            resource.Map(
                0,
                None,
                Some(&mut mapped),
            )?;

            std::ptr::copy_nonoverlapping(
                indices.as_ptr() as *const u8,
                mapped as *mut u8,
                size as usize,
            );

            resource.Unmap(0, None);
        }

        let view = D3D12_INDEX_BUFFER_VIEW {
            BufferLocation: unsafe {
                resource.GetGPUVirtualAddress()
            },
            SizeInBytes: size as u32,
            Format: DXGI_FORMAT_R32_UINT,
        };

        Ok(Self {
            _resource: resource,
            view,
            index_count: indices.len() as u32,
        })
    }

    pub(crate) fn view(&self) -> &D3D12_INDEX_BUFFER_VIEW {
        &self.view
    }

    pub(crate) fn index_count(&self) -> u32 {
        self.index_count
    }
}