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

const CONSTANT_BUFFER_ALIGNMENT: usize = 256;

// Temporary capacity. Later we'll make this grow dynamically.
const MAX_CONSTANTS_PER_FRAME: usize = 1024;

pub(crate) struct D3D12ConstantBuffer {
    resource: ID3D12Resource,
    mapped: *mut u8,
    next_offset: usize,
}

impl D3D12ConstantBuffer {
    pub(crate) fn new(
        device: &D3D12Device,
    ) -> Result<Self> {
        let size =
            CONSTANT_BUFFER_ALIGNMENT * MAX_CONSTANTS_PER_FRAME;

        let resource_desc = D3D12_RESOURCE_DESC {
            Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
            Alignment: 0,
            Width: size as u64,
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
            next_offset: 0,
        })
    }

    /// Resets allocation to the beginning of the buffer.
    ///
    /// This is safe because the current renderer waits for the GPU
    /// to finish each frame before beginning another.
    pub(crate) fn reset(&mut self) {
        self.next_offset = 0;
    }

    pub(crate) fn write<T: Copy>(
        &mut self,
        data: &T,
    ) -> u64 {
        let data_size = std::mem::size_of::<T>();

        let allocation_size =
            (data_size + CONSTANT_BUFFER_ALIGNMENT - 1)
                / CONSTANT_BUFFER_ALIGNMENT
                * CONSTANT_BUFFER_ALIGNMENT;

        assert!(
            self.next_offset + allocation_size
                <= CONSTANT_BUFFER_ALIGNMENT
                    * MAX_CONSTANTS_PER_FRAME,
            "Oxide exceeded the temporary per-frame constant-buffer capacity"
        );

        let offset = self.next_offset;

        self.next_offset += allocation_size;

        unsafe {
            std::ptr::copy_nonoverlapping(
                data as *const T as *const u8,
                self.mapped.add(offset),
                data_size,
            );
        }

        let base_address = unsafe {
            self.resource.GetGPUVirtualAddress()
        };

        base_address + offset as u64
    }
}

impl Drop for D3D12ConstantBuffer {
    fn drop(&mut self) {
        unsafe {
            self.resource.Unmap(0, None);
        }
    }
}