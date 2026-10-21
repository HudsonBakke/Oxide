use std::sync::atomic::{
    AtomicU64,
    Ordering,
};

use super::Vertex;

static NEXT_MESH_ID: AtomicU64 = AtomicU64::new(1);

/// Internal unique identifier for a mesh.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct MeshId(u64);

fn next_mesh_id() -> MeshId {
    MeshId(
        NEXT_MESH_ID.fetch_add(
            1,
            Ordering::Relaxed,
        )
    )
}

/// A collection of vertices and indices describing 3D geometry.
///
/// A mesh contains geometry only. It does not define where that geometry
/// appears in the world, how it is animated, or how it is rendered.
#[derive(Debug)]
pub struct Mesh {
    id: MeshId,
    generation: u64,

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
            id: next_mesh_id(),
            generation: 0,
            vertices,
            indices,
        }
    }

    /// Returns the mesh's vertices.
    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    /// Returns mutable access to the mesh's vertices.
    ///
    /// Requesting mutable access marks the mesh geometry as changed.
    pub fn vertices_mut(&mut self) -> &mut [Vertex] {
        self.mark_changed();
        &mut self.vertices
    }

    /// Returns the mesh's indices.
    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    /// Returns mutable access to the mesh's indices.
    ///
    /// Requesting mutable access marks the mesh geometry as changed.
    pub fn indices_mut(&mut self) -> &mut [u32] {
        self.mark_changed();
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

    pub(crate) fn id(&self) -> MeshId {
        self.id
    }

    pub(crate) fn generation(&self) -> u64 {
        self.generation
    }

    fn mark_changed(&mut self) {
        self.generation =
            self.generation.wrapping_add(1);
    }
}

impl Clone for Mesh {
    fn clone(&self) -> Self {
        Self {
            id: next_mesh_id(),
            generation: 0,
            vertices: self.vertices.clone(),
            indices: self.indices.clone(),
        }
    }
}