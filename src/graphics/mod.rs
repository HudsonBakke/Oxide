pub(crate) mod backend;

mod error;
mod graphics;
pub(crate) mod mesh;
pub(crate) mod texture;
mod vertex;
mod camera;
mod color;
mod light;
mod material;

pub use error::{
    GraphicsError,
    GraphicsResult,
};
pub use graphics::Graphics;

pub use mesh::Mesh;
pub use vertex::Vertex;
pub use camera::Camera;
pub use color::Color;
pub use texture::Texture;
pub use material::Material;

pub use light::{
    DirectionalLight,
    LightSource,
    PointLight,
    SpotLight,
};