include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/graphics/backend/d3d12/texture.rs"));

impl D3D12Texture {
    pub(crate) fn upload_pending(&self) -> bool { self.upload_resource.is_some() }
}
