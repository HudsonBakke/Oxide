use windows::{
    core::Result,
    Win32::Graphics::Direct3D12::{
        ID3D12Resource,
        D3D12_HEAP_PROPERTIES,
        D3D12_HEAP_TYPE_UPLOAD,
        D3D12_RESOURCE_DESC,
        D3D12_RESOURCE_DIMENSION_BUFFER,
        D3D12_RESOURCE_FLAG_NONE,
        D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
        D3D12_RESOURCE_STATE_GENERIC_READ,
        D3D12_VERTEX_BUFFER_VIEW,
    },
};

use windows::Win32::Graphics::Dxgi::Common::{
    DXGI_FORMAT_UNKNOWN,
    DXGI_SAMPLE_DESC,
};

use super::D3D12Device;

use crate::graphics::Vertex;

pub(crate) struct D3D12VertexBuffer {
    resource: ID3D12Resource,
    view: D3D12_VERTEX_BUFFER_VIEW,
    vertex_count: u32,
}

impl D3D12VertexBuffer {
    pub(crate) fn new(
        device: &D3D12Device,
        vertices: &[Vertex],
    ) -> Result<Self> {
        let size =
            std::mem::size_of_val(vertices) as u64;

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
            "CreateCommittedResource succeeded but returned no resource"
        );

        let mut mapped = std::ptr::null_mut();

        unsafe {
            resource.Map(
                0,
                None,
                Some(&mut mapped),
            )?;
        }

        unsafe {
            std::ptr::copy_nonoverlapping(
                vertices.as_ptr() as *const u8,
                mapped as *mut u8,
                size as usize,
            );
        }

        unsafe {
            resource.Unmap(0, None);
        }

        let view = D3D12_VERTEX_BUFFER_VIEW {
            BufferLocation: unsafe {
                resource.GetGPUVirtualAddress()
            },
            SizeInBytes: size as u32,
            StrideInBytes: std::mem::size_of::<Vertex>() as u32,
        };

        Ok(Self {
            resource,
            view,
            vertex_count: vertices.len() as u32,
        })
    }

    pub(crate) fn view(&self) -> &D3D12_VERTEX_BUFFER_VIEW {
        &self.view
    }

    pub(crate) fn vertex_count(&self) -> u32 {
        self.vertex_count
    }
}