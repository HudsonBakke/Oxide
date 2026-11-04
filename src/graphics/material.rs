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