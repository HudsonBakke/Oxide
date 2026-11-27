include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/graphics/backend/d3d12/descriptor_allocator.rs"));

#[path = "descriptor_allocator_checks.rs"]
mod checks;

impl DescriptorAllocator {
    pub(crate) fn used(&self) -> u32 { self.allocated.values().sum() }
}
