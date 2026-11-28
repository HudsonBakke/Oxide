//! Mesh drawing, materials, cameras, textures, and lighting for Direct3D 12.

pub(crate) mod draw2d;
mod text;
pub(crate) mod backend;
pub(crate) mod mesh;
pub(crate) mod texture;

mod attachment;
mod camera;
mod color;
mod error;
mod graphics;
mod environment;
mod light;
mod material;
mod vertex;
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

pub use draw2d::{Draw2DOptions,TextOptions};
