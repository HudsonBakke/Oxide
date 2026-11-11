use std::{
    io::{
        Read,
        Write,
    },
    path::Path,
};

use crate::content::{
    AssetType,
    ContentAsset,
    ContentResult,
    ContentAssetInternal,
};

use super::{
    Color,
    Texture,
};

/// Describes the surface appearance used to render a mesh.
#[derive(Debug)]
pub struct Material {
    base_texture: Texture,
}

impl Material {
    /// Creates a material using the given base texture.
    pub fn new(
        base_texture: Texture,
    ) -> Self {
        Self {
            base_texture,
        }
    }

    /// Returns the material's base texture.
    pub fn base_texture(&self) -> &Texture {
        &self.base_texture
    }

    /// Creates a solid-color material.
    pub fn solid(color: Color) -> Self {
        Self {
            base_texture:
                Texture::solid(color),
        }
    }
}

impl ContentAssetInternal for Material {
    fn asset_type_name()
        -> &'static str
    {
        "Material"
    }

    fn supports_extension(
        extension: &str,
    ) -> bool {
        matches!(
            extension,
            "gltf" | "glb"
        )
    }

    fn import(
        path: &Path,
        subobject: &str,
    ) -> ContentResult<Self> {
        crate::content::import_gltf_material(
            path,
            subobject,
        )
    }

    fn compiled_asset_type()
        -> AssetType
    {
        AssetType::Material
    }

    fn compiler_version() -> u32 {
        1
    }

    fn write_compiled(
        &self,
        writer: &mut impl Write,
    ) -> ContentResult<()> {
        <Texture as ContentAssetInternal>
            ::write_compiled(
                &self.base_texture,
                writer,
            )
    }

    fn read_compiled(
        reader: &mut impl Read,
        payload_size: u64,
    ) -> ContentResult<Self> {
        let texture =
            <Texture as ContentAssetInternal>
                ::read_compiled(
                    reader,
                    payload_size,
                )?;

        Ok(
            Self::new(texture)
        )
    }
}

impl ContentAsset for Material {}