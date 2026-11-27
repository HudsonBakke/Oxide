//! Right-handed 3D math with +Y up, explicit angles, and quaternion orientations.

pub mod angle;
pub mod quaternion;
pub mod rect;
pub mod rect3d;
pub mod transform;
pub mod vec2;
pub mod vec3;
pub mod vec4;

pub(crate) mod mat4;

pub use angle::Angle;
pub use quaternion::Quaternion;
pub use rect::Rect;
pub use rect3d::Rect3D;
pub use transform::Transform;
pub use vec2::Vec2;
pub use vec3::Vec3;
pub use vec4::Vec4;

pub(crate) use mat4::Mat4;
