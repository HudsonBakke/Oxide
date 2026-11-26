use crate::math::{Vec2, Vec3, Vec4};

/// A vertex in a 3D mesh.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub tex_coord: Vec2,

    /// Tangent-space X axis. xyz is the tangent direction and w is the
    /// handedness used to reconstruct the bitangent.
    pub tangent: Vec4,
}

impl Vertex {
    /// Creates a vertex. Mesh construction automatically generates a tangent
    /// basis from positions/UVs when no explicit tangent was supplied.
    pub const fn new(position: Vec3, normal: Vec3, tex_coord: Vec2) -> Self {
        Self {
            position,
            normal,
            tex_coord,
            tangent: Vec4::new(0.0, 0.0, 0.0, 1.0),
        }
    }

    pub const fn with_tangent(
        position: Vec3,
        normal: Vec3,
        tex_coord: Vec2,
        tangent: Vec4,
    ) -> Self {
        Self {
            position,
            normal,
            tex_coord,
            tangent,
        }
    }
}
