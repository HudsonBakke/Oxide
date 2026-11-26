use crate::math::{Vec2, Vec3, Vec4};

/// A vertex in a 3D mesh.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Vertex data consumed by Oxide's mesh renderer.
pub struct Vertex {
    /// World-space position.
    pub position: Vec3,
    /// Public Oxide normal API.
    pub normal: Vec3,
    /// Public Oxide tex coord API.
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
            tangent: Vec4 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
        }
    }

    /// Public Oxide fn API.
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
