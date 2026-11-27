use windows::{
    core::{Error, Result},
    Win32::{Foundation::{E_INVALIDARG, E_OUTOFMEMORY}, Graphics::{
        Direct3D12::*, Dxgi::Common::{DXGI_FORMAT_UNKNOWN, DXGI_SAMPLE_DESC},
    }},
};

use super::D3D12Device;

const CONSTANT_BUFFER_ALIGNMENT: usize = 256;
const PAGE_SIZE: usize = 4 * 1024 * 1024;
const MAX_CONSTANT_SIZE: usize = 64 * 1024;

struct UploadPage {
    resource: ID3D12Resource,
    mapped: *mut u8,
}

impl UploadPage {
    fn new(device: &D3D12Device) -> Result<Self> {
        let mut resource = None;
        unsafe {
            device.device().CreateCommittedResource(
                &D3D12_HEAP_PROPERTIES { Type: D3D12_HEAP_TYPE_UPLOAD, ..Default::default() },
                D3D12_HEAP_FLAG_NONE,
                &D3D12_RESOURCE_DESC {
                    Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
                    Width: PAGE_SIZE as u64, Height: 1, DepthOrArraySize: 1, MipLevels: 1,
                    Format: DXGI_FORMAT_UNKNOWN,
                    SampleDesc: DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
                    Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
                    ..Default::default()
                }, D3D12_RESOURCE_STATE_GENERIC_READ, None, &mut resource,
            )?;
        }
        let resource: ID3D12Resource = resource.ok_or_else(|| Error::new(E_OUTOFMEMORY, "constant-buffer allocation returned no resource"))?;
        let mut mapped = std::ptr::null_mut();
        unsafe { resource.Map(0, None, Some(&mut mapped))?; }
        if mapped.is_null() { return Err(Error::new(E_OUTOFMEMORY, "constant-buffer mapping returned null")); }
        Ok(Self { resource, mapped: mapped as *mut u8 })
    }
}

impl Drop for UploadPage {
    fn drop(&mut self) { unsafe { self.resource.Unmap(0, None); } }
}

/// Stable GPU addresses for every allocation until the completed frame is reset.
pub(crate) struct D3D12ConstantBuffer {
    pages: Vec<UploadPage>,
    page: usize,
    next_offset: usize,
}

impl D3D12ConstantBuffer {
    pub(crate) fn new(device: &D3D12Device) -> Result<Self> {
        Ok(Self { pages: vec![UploadPage::new(device)?], page: 0, next_offset: 0 })
    }

    /// Called only after the previous frame has finished on the GPU.
    /// Retain pages used in that frame; retire older unused high-water pages.
    pub(crate) fn reset(&mut self) {
        self.pages.truncate(self.page + 1);
        self.page = 0;
        self.next_offset = 0;
    }

    pub(crate) fn write<T: Copy>(&mut self, device: &D3D12Device, data: &T) -> Result<u64> {
        let size = std::mem::size_of::<T>();
        if size == 0 || size > MAX_CONSTANT_SIZE {
            return Err(Error::new(E_INVALIDARG, "constant-buffer data must contain between one byte and 64 KiB"));
        }
        let allocation = size.div_ceil(CONSTANT_BUFFER_ALIGNMENT) * CONSTANT_BUFFER_ALIGNMENT;
        if self.next_offset + allocation > PAGE_SIZE {
            let next = self.page + 1;
            if next == self.pages.len() { self.pages.push(UploadPage::new(device)?); }
            self.page = next;
            self.next_offset = 0;
        }
        let page = &self.pages[self.page];
        let offset = self.next_offset;
        self.next_offset += allocation;
        unsafe {
            std::ptr::copy_nonoverlapping(data as *const T as *const u8, page.mapped.add(offset), size);
            Ok(page.resource.GetGPUVirtualAddress() + offset as u64)
        }
    }
}
