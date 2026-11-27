use super::*;

#[test]
fn exhausted_and_fragmented_allocations_report_errors_without_consuming_slots() {
    let mut allocator = DescriptorAllocator::new(1633, 1643);
    let a = allocator.allocate(3).unwrap();
    let b = allocator.allocate(4).unwrap();
    let c = allocator.allocate(3).unwrap();
    assert_eq!((a, b, c), (1633, 1636, 1640));
    assert_eq!(allocator.used(), 10);
    assert_eq!(allocator.allocate(1).unwrap_err().code(), E_OUTOFMEMORY);
    assert!(allocator.release(a, 3));
    assert!(allocator.release(c, 3));
    let error = allocator.allocate(4).unwrap_err();
    assert!(error.to_string().contains("6 free, largest range 3"));
    assert_eq!(allocator.used(), 4);
    assert!(allocator.release(b, 4));
    assert_eq!(allocator.allocate(10).unwrap(), 1633);
}

#[test]
fn freed_ranges_are_reused_and_invalid_releases_cannot_corrupt_them() {
    let mut allocator = DescriptorAllocator::new(1633, 1643);
    for _ in 0..10_000 {
        let a = allocator.allocate(2).unwrap();
        let b = allocator.allocate(3).unwrap();
        assert!(!allocator.release(a, 1));
        assert!(!allocator.release(a + 1, 1));
        assert!(!allocator.release(0, 2));
        assert!(allocator.release(a, 2));
        assert!(!allocator.release(a, 2));
        assert!(allocator.release(b, 3));
        assert_eq!(allocator.used(), 0);
        assert_eq!(allocator.free, vec![1633..1643]);
    }
    assert_eq!(allocator.allocate(0).unwrap_err().code(), E_INVALIDARG);
}
