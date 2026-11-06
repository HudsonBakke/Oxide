pub(crate) mod backend;
pub(crate) mod mesh;
pub(crate) mod texture;

mod error;
mod graphics;
mod vertex;
mod camera;
mod color;
mod light;
mod material;
mod attachment;

pub use graphics::Graphics;
pub use mesh::Mesh;
pub use vertex::Vertex;
pub use camera::Camera;
pub use color::Color;
pub use texture::Texture;
pub use material::Material;
pub use attachment::from_attachment;

pub use error::{
    GraphicsError,
    GraphicsResult,
};

pub use light::{
    DirectionalLight,
    LightSource,
    PointLight,
    SpotLight,
};