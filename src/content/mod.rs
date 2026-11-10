mod asset;
mod cache;
mod error;
mod format;
mod manager;
mod metadata;
mod importers;

pub use asset::ContentAsset;

pub use error::{
    ContentError,
    ContentResult,
};

pub use manager::ContentManager;

#[doc(hidden)]
pub use format::AssetType;

pub(crate) fn import_gltf_mesh(
    path: &std::path::Path,
    subobject: &str,
) -> ContentResult<
    crate::graphics::Mesh
> {
    importers::gltf::import_mesh(
        path,
        subobject,
    )
}