use std::{
    io::{Read, Write},
    path::Path,
};

use crate::{
    OxideError, OxideResult,
    content::{AssetType, ContentAsset, ContentAssetInternal, ContentResult},
};

use super::{Color, Texture};

/// Describes the surface appearance used to render a mesh.
#[derive(Debug)]
pub struct Material {
    base_texture: Texture,
    base_color: Color,
    normal_texture: Texture,
    normal_strength: f32,
    metallic: f32,
    roughness: f32,
    emissive_color: Color,
    emissive_strength: f32,
}

impl Material {
    /// Creates a material using the given base texture.
    pub fn new(base_texture: Texture) -> Self {
        Self {
            base_texture,
            base_color: Color::WHITE,
            normal_texture: Texture::solid(Color::rgba(0.5, 0.5, 1.0, 1.0)),
            normal_strength: 1.0,
            metallic: 0.0,
            roughness: 1.0,
            emissive_color: Color::BLACK,
            emissive_strength: 0.0,
        }
    }

    pub fn base_texture(&self) -> &Texture {
        &self.base_texture
    }

    pub fn normal_texture(&self) -> &Texture { &self.normal_texture }
    pub fn set_normal_texture(&mut self, texture: Texture) { self.normal_texture = texture; }
    pub fn normal_strength(&self) -> f32 { self.normal_strength }
    pub fn set_normal_strength(&mut self, strength: f32) -> OxideResult<()> {
        if !strength.is_finite() {
            return Err(OxideError::InvalidArgument {
                argument: "normal_strength",
                message: "normal strength must be finite",
            });
        }
        self.normal_strength = strength;
        Ok(())
    }

    pub fn base_color(&self) -> Color {
        self.base_color
    }

    pub fn set_base_color(&mut self, color: Color) {
        self.base_color = color;
    }

    pub fn metallic(&self) -> f32 {
        self.metallic
    }

    pub fn set_metallic(&mut self, metallic: f32) -> OxideResult<()> {
        validate_unit("metallic", metallic)?;
        self.metallic = metallic;
        Ok(())
    }

    pub fn roughness(&self) -> f32 {
        self.roughness
    }

    pub fn set_roughness(&mut self, roughness: f32) -> OxideResult<()> {
        validate_unit("roughness", roughness)?;
        self.roughness = roughness;
        Ok(())
    }

    pub fn emissive_color(&self) -> Color {
        self.emissive_color
    }

    pub fn set_emissive_color(&mut self, color: Color) {
        self.emissive_color = color;
    }

    pub fn emissive_strength(&self) -> f32 {
        self.emissive_strength
    }

    pub fn set_emissive_strength(&mut self, strength: f32) -> OxideResult<()> {
        if !strength.is_finite() || strength < 0.0 {
            return Err(OxideError::InvalidArgument {
                argument: "emissive_strength",
                message: "emissive strength must be finite and non-negative",
            });
        }

        self.emissive_strength = strength;
        Ok(())
    }

    /// Creates a solid-color material.
    pub fn solid(color: Color) -> Self {
        let mut material = Self::new(Texture::solid(Color::WHITE));
        material.base_color = color;
        material
    }
}

fn validate_unit(argument: &'static str, value: f32) -> OxideResult<()> {
    if !value.is_finite() || !(0.0..=1.0).contains(&value) {
        return Err(OxideError::InvalidArgument {
            argument,
            message: "material factor must be finite and between zero and one",
        });
    }

    Ok(())
}

impl ContentAssetInternal for Material {
    fn asset_type_name() -> &'static str { "Material" }
    fn supports_extension(extension: &str) -> bool { matches!(extension, "gltf" | "glb") }
    fn import(path: &Path, subobject: &str) -> ContentResult<Self> {
        crate::content::import_gltf_material(path, subobject)
    }
    fn compiled_asset_type() -> AssetType { AssetType::Material }
    fn compiler_version() -> u32 { 3 }

    fn write_compiled(&self, writer: &mut impl Write) -> ContentResult<()> {
        for value in [
            self.base_color.r, self.base_color.g, self.base_color.b, self.base_color.a,
            self.normal_strength,
            self.metallic, self.roughness,
            self.emissive_color.r, self.emissive_color.g,
            self.emissive_color.b, self.emissive_color.a,
            self.emissive_strength,
        ] {
            writer.write_all(&value.to_le_bytes())?;
        }

        write_embedded_texture(&self.base_texture, writer)?;
        write_embedded_texture(&self.normal_texture, writer)
    }

    fn read_compiled(reader: &mut impl Read, payload_size: u64) -> ContentResult<Self> {
        const PROPERTY_BYTES: u64 = 12 * 4;

        if payload_size < PROPERTY_BYTES {
            return Err(crate::content::ContentError::InvalidCache(
                "material payload is too small".into(),
            ));
        }

        fn read_f32(reader: &mut impl Read) -> ContentResult<f32> {
            let mut bytes = [0_u8; 4];
            reader.read_exact(&mut bytes)?;
            Ok(f32::from_le_bytes(bytes))
        }

        let base_color = Color::rgba(
            read_f32(reader)?, read_f32(reader)?,
            read_f32(reader)?, read_f32(reader)?,
        );
        let normal_strength = read_f32(reader)?;
        let metallic = read_f32(reader)?;
        let roughness = read_f32(reader)?;
        let emissive_color = Color::rgba(
            read_f32(reader)?, read_f32(reader)?,
            read_f32(reader)?, read_f32(reader)?,
        );
        let emissive_strength = read_f32(reader)?;

        let remaining = payload_size - PROPERTY_BYTES;
        let (base_texture, base_size) = read_embedded_texture(reader, remaining)?;
        let (normal_texture, normal_size) =
            read_embedded_texture(reader, remaining - base_size)?;

        if base_size + normal_size != remaining {
            return Err(crate::content::ContentError::InvalidCache(
                "material texture payload size mismatch".into(),
            ));
        }

        let mut material = Self::new(base_texture);
        material.normal_texture = normal_texture;
        material.base_color = base_color;
        material.set_normal_strength(normal_strength).map_err(|_| {
            crate::content::ContentError::InvalidCache("invalid normal strength".into())
        })?;
        material.set_metallic(metallic).map_err(|_| {
            crate::content::ContentError::InvalidCache("invalid metallic factor".into())
        })?;
        material.set_roughness(roughness).map_err(|_| {
            crate::content::ContentError::InvalidCache("invalid roughness factor".into())
        })?;
        material.emissive_color = emissive_color;
        material.set_emissive_strength(emissive_strength).map_err(|_| {
            crate::content::ContentError::InvalidCache("invalid emissive strength".into())
        })?;
        Ok(material)
    }
}


fn write_embedded_texture(
    texture: &Texture,
    writer: &mut impl Write,
) -> ContentResult<()> {
    writer.write_all(&texture.width().to_le_bytes())?;
    writer.write_all(&texture.height().to_le_bytes())?;
    let pixels = texture.pixels();
    let count = u64::try_from(pixels.len()).map_err(|_| {
        crate::content::ContentError::InvalidCache("embedded texture is too large".into())
    })?;
    writer.write_all(&count.to_le_bytes())?;
    writer.write_all(pixels)?;
    Ok(())
}

fn read_embedded_texture(
    reader: &mut impl Read,
    available: u64,
) -> ContentResult<(Texture, u64)> {
    const HEADER: u64 = 16;
    if available < HEADER {
        return Err(crate::content::ContentError::InvalidCache(
            "embedded texture payload is too small".into(),
        ));
    }
    let mut u32_bytes = [0_u8; 4];
    reader.read_exact(&mut u32_bytes)?;
    let width = u32::from_le_bytes(u32_bytes);
    reader.read_exact(&mut u32_bytes)?;
    let height = u32::from_le_bytes(u32_bytes);
    let mut u64_bytes = [0_u8; 8];
    reader.read_exact(&mut u64_bytes)?;
    let count = u64::from_le_bytes(u64_bytes);
    let expected = u64::from(width)
        .checked_mul(u64::from(height))
        .and_then(|v| v.checked_mul(4))
        .ok_or_else(|| crate::content::ContentError::InvalidCache(
            "embedded texture dimensions overflow".into(),
        ))?;
    if count != expected || HEADER + count > available {
        return Err(crate::content::ContentError::InvalidCache(
            "invalid embedded texture payload".into(),
        ));
    }
    let mut pixels = vec![0_u8; count as usize];
    reader.read_exact(&mut pixels)?;
    Ok((Texture::from_rgba(width, height, pixels), HEADER + count))
}

impl ContentAsset for Material {}
