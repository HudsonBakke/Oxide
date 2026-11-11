mod asset;
mod cache;
mod error;
mod format;
mod manager;
mod metadata;
mod importers;

pub use asset::ContentAsset;

pub(crate) use asset::ContentAssetInternal;
pub(crate) use format::AssetType;

pub use error::{
    ContentError,
    ContentResult,
};

pub use manager::ContentManager;

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

pub(crate) fn import_gltf_texture(
    path: &std::path::Path,
    subobject: &str,
) -> ContentResult<
    crate::graphics::Texture
> {
    importers::gltf::import_texture(
        path,
        subobject,
    )
}

pub(crate) fn import_gltf_material(
    path: &std::path::Path,
    subobject: &str,
) -> ContentResult<
    crate::graphics::Material
> {
    importers::gltf::import_material(
        path,
        subobject,
    )
}