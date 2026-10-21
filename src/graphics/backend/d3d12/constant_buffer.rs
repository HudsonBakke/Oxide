use windows::{
    core::Result,
    Win32::Graphics::{
        Direct3D12::{
            D3D12_HEAP_PROPERTIES,
            D3D12_HEAP_TYPE_UPLOAD,
            D3D12_RESOURCE_DESC,
            D3D12_RESOURCE_DIMENSION_BUFFER,
            D3D12_RESOURCE_FLAG_NONE,
            D3D12_RESOURCE_STATE_GENERIC_READ,
            D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
            ID3D12Resource,
        },
        Dxgi::Common::{
            DXGI_FORMAT_UNKNOWN,
            DXGI_SAMPLE_DESC,
        },
    },
};

use super::D3D12Device;

use crate::math::Mat4;

const CONSTANT_BUFFER_ALIGNMENT: usize = 256;

pub(crate) struct D3D12ConstantBuffer {
    resource: ID3D12Resource,
    mapped: *mut u8,
}

impl D3D12ConstantBuffer {
    pub(crate) fn new(
        device: &D3D12Device,
    ) -> Result<Self> {
        let resource_desc = D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
            Alignment: 0,
            Width: CONSTANT_BUFFER_ALIGNMENT as u64,
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

        let heap_properties = D3D12_HEAP_PROPERTIES {
            Type: D3D12_HEAP_TYPE_UPLOAD,
            ..Default::default()
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
            "CreateCommittedResource succeeded but returned no constant buffer"
        );

        let mut mapped = std::ptr::null_mut();

        unsafe {
            resource.Map(
                0,
                None,
                Some(&mut mapped),
            )?;
        }

        Ok(Self {
            resource,
            mapped: mapped as *mut u8,
        })
    }

    pub(crate) fn write_matrix(
        &mut self,
        matrix: &Mat4,
    ) {
        // HLSL matrices are column-major by default, while Oxide's
        // Mat4 storage is interpreted row-major by our CPU math.
        let matrix = matrix.transposed();
        let data = matrix.as_array();

        unsafe {
            std::ptr::copy_nonoverlapping(
                data.as_ptr() as *const u8,
                self.mapped,
                std::mem::size_of_val(&data),
            );
        }
    }

    pub(crate) fn gpu_address(&self) -> u64 {
        unsafe {
            self.resource.GetGPUVirtualAddress()
        }
    }
}

impl Drop for D3D12ConstantBuffer {
    fn drop(&mut self) {
        unsafe {
            self.resource.Unmap(0, None);
        }
    }
}