//! Mesh drawing, materials, cameras, textures, and lighting for Direct3D 12.

#[path = "backend.rs"]
pub(crate) mod backend;
#[path = "../../src/graphics/mesh.rs"]
pub(crate) mod mesh;
#[path = "texture.rs"]
pub(crate) mod texture;

#[path = "../../src/graphics/attachment.rs"]
mod attachment;
#[path = "../../src/graphics/camera.rs"]
mod camera;
#[path = "../../src/graphics/color.rs"]
mod color;
#[path = "../../src/graphics/error.rs"]
mod error;
#[path = "graphics_api.rs"]
mod graphics;
#[path = "../../src/graphics/environment.rs"]
mod environment;
#[path = "light.rs"]
mod light;
#[path = "../../src/graphics/material.rs"]
mod material;
#[path = "../../src/graphics/vertex.rs"]
mod vertex;
#[path = "shadow.rs"]
pub(crate) mod shadow;

pub use attachment::from_attachment;
pub use camera::Camera;
pub use color::Color;
pub use graphics::Graphics;
pub use environment::Environment;
pub use material::Material;
pub use mesh::Mesh;
pub use texture::Texture;
pub use vertex::Vertex;
pub use shadow::{DirectionalShadowSettings, ShadowSettings};

pub use error::{GraphicsError, GraphicsResult};

pub use light::{DirectionalLight, LightSource, PointLight, SpotLight};
