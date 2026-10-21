use super::Vertex;

/// A collection of vertices and indices describing 3D geometry.
///
/// A mesh contains geometry only. It does not define where that geometry
/// appears in the world, how it is animated, or how it is rendered.
#[derive(Debug, Clone)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
}

impl Mesh {
    /// Creates a mesh from vertex and index data.
    pub fn new(
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
    ) -> Self {
        Self {
            vertices,
            indices,
        }
    }

    /// Returns the mesh's vertices.
    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    /// Returns mutable access to the mesh's vertices.
    pub fn vertices_mut(&mut self) -> &mut [Vertex] {
        &mut self.vertices
    }

    /// Returns the mesh's indices.
    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    /// Returns mutable access to the mesh's indices.
    pub fn indices_mut(&mut self) -> &mut [u32] {
        &mut self.indices
    }

    /// Returns the number of vertices in the mesh.
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Returns the number of indices in the mesh.
    pub fn index_count(&self) -> usize {
        self.indices.len()
    }
}