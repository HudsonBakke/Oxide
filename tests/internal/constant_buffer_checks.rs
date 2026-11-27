use super::*;

#[test]
fn upload_pages_grow_without_overwriting_existing_constants_and_retire_when_idle() {
    let device = D3D12Device::warp_for_test().unwrap();
    let mut buffer = D3D12ConstantBuffer::new(&device).unwrap();
    let first = buffer.write(&device, &[7_u8; 1024]).unwrap();
    for _ in 1..5000 { buffer.write(&device, &[9_u8; 1024]).unwrap(); }
    assert_eq!(buffer.pages.len(), 2);
    assert_eq!(first, unsafe { buffer.pages[0].resource.GetGPUVirtualAddress() });
    assert_eq!(unsafe { *buffer.pages[0].mapped }, 7);
    assert_eq!(unsafe { *buffer.pages[1].mapped }, 9);
    let second_page = unsafe { buffer.pages[1].resource.GetGPUVirtualAddress() };
    buffer.reset();
    for _ in 0..5000 { buffer.write(&device, &[3_u8; 1024]).unwrap(); }
    assert_eq!(buffer.pages.len(), 2);
    assert_eq!(unsafe { buffer.pages[1].resource.GetGPUVirtualAddress() }, second_page);
    buffer.reset();
    assert_eq!(buffer.write(&device, &[1_u8; 16]).unwrap(), first);
    buffer.reset();
    assert_eq!(buffer.pages.len(), 1);
}

#[test]
fn invalid_constant_sizes_return_errors_and_small_constants_are_aligned() {
    let device = D3D12Device::warp_for_test().unwrap();
    let mut buffer = D3D12ConstantBuffer::new(&device).unwrap();
    assert_eq!(buffer.write(&device, &()).unwrap_err().code(), E_INVALIDARG);
    assert_eq!(buffer.write(&device, &[0_u8; 65537]).unwrap_err().code(), E_INVALIDARG);
    let a = buffer.write(&device, &1_u8).unwrap();
    let b = buffer.write(&device, &[2_u8; 256]).unwrap();
    assert_eq!(b - a, 256);
    assert_eq!(buffer.next_offset, 512);
}
