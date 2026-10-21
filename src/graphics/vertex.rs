use crate::math::{
    Vec2,
    Vec3,
};

/// A vertex in a 3D mesh
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex {
    /// Position of the vertex in the mesh's model space
    pub position: Vec3,

    /// The vertex's surface normal
    pub normal: Vec3,

    /// The vertex's texture coordinate
    pub tex_coord: Vec2,
}

impl Vertex {
    pub const fn new(
        position: Vec3,
        normal: Vec3,
        tex_coord: Vec2,
    ) -> Self {
        Self {
            position,
            normal,
            tex_coord,
        }
    }
}