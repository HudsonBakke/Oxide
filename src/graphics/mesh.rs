use std::sync::atomic::{
    AtomicU64,
    Ordering,
};

use super::Vertex;

use crate::math::Vec3;

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

    vertex_generation: u64,
    index_generation: u64,

    vertices: Vec<Vertex>,
    indices: Vec<u32>,

    pivot: Vec3,

    direction: Vec3,
    reference: Vec3,
}

fn calculate_vertex_average(vertices: &[Vertex]) -> Vec3 {
    if vertices.is_empty() {
        return Vec3::default();
    }

    let mut sum = Vec3::default();

    for vertex in vertices {
        sum = sum + vertex.position;
    }

    sum / vertices.len() as f32
}

const ORIENTATION_EPSILON: f32 = 1.0e-6;

fn orthogonalize_reference(
    direction: Vec3,
    reference: Vec3,
) -> Vec3 {
    let projected =
        reference
        - direction
            * reference.dot(&direction);

    assert!(
        projected.length()
            > ORIENTATION_EPSILON,
        "mesh reference cannot be parallel to mesh direction"
    );

    projected.normalized()
}

impl Mesh {
    /// Creates a mesh from vertex and index data.
    pub fn new(
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
    ) -> Self {
        let pivot = calculate_vertex_average(&vertices);

        Self {
            id: next_mesh_id(),

            vertex_generation: 0,
            index_generation: 0,

            vertices,
            indices,

            pivot,

            direction: Vec3::new(
                0.0,
                0.0,
                -1.0,
            ),

            reference: Vec3::new(
                0.0,
                1.0,
                0.0,
            ),
        }
    }

    /// Returns the mesh's model-space pivot point.
    pub fn pivot(&self) -> Vec3 {
        self.pivot
    }

    /// Sets the mesh's model-space pivot point.
    pub fn set_pivot(&mut self, pivot: Vec3) {
        self.pivot = pivot;
    }

    /// Recalculates the pivot using the average position of all vertices.
    pub fn recalculate_pivot(&mut self) {
        self.pivot =
            calculate_vertex_average(&self.vertices);
    }

    /// Returns the mesh's vertices.
    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    /// Returns the mesh's indices.
    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    /// Returns mutable access to the mesh's vertices.
    ///
    /// Requesting mutable access marks the mesh geometry as changed.
    pub fn vertices_mut(&mut self) -> &mut [Vertex] {
        self.vertex_generation =
            self.vertex_generation.wrapping_add(1);

        &mut self.vertices
    }

    /// Returns mutable access to the mesh's indices.
    ///
    /// Requesting mutable access marks the mesh geometry as changed.
    pub fn indices_mut(&mut self) -> &mut [u32] {
        self.index_generation =
            self.index_generation.wrapping_add(1);

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

    pub(crate) fn vertex_generation(&self) -> u64 {
        self.vertex_generation
    }

    pub(crate) fn index_generation(&self) -> u64 {
        self.index_generation
    }

    /// Returns the mesh's canonical model-space direction.
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    /// Returns the mesh's canonical model-space reference direction.
    ///
    /// The reference is always perpendicular to the canonical direction.
    pub fn reference(&self) -> Vec3 {
        self.reference
    }

    /// Sets the mesh's canonical reference direction.
    ///
    /// The supplied vector is automatically made perpendicular
    /// to the mesh's canonical direction and normalized.
    pub fn set_reference(
        &mut self,
        reference: Vec3,
    ) {
        assert!(
            reference.length()
                > ORIENTATION_EPSILON,
            "mesh reference cannot be zero"
        );

        self.reference =
            orthogonalize_reference(
                self.direction,
                reference,
            );
    }

    /// Sets the mesh's canonical direction while preserving its
    /// current reference orientation when possible.
    pub fn set_direction(
        &mut self,
        direction: Vec3,
    ) {
        assert!(
            direction.length()
                > ORIENTATION_EPSILON,
            "mesh direction cannot be zero"
        );

        let direction =
            direction.normalized();

        self.reference =
            orthogonalize_reference(
                direction,
                self.reference,
            );

        self.direction = direction;
    }

    /// Sets the mesh's canonical model-space orientation.
    ///
    /// `direction` defines the mesh's forward/longitudinal axis.
    /// `reference` defines its zero-twist orientation around that axis.
    ///
    /// Both vectors are normalized automatically, and `reference` is
    /// automatically made perpendicular to `direction`.
    pub fn set_orientation(
        &mut self,
        direction: Vec3,
        reference: Vec3,
    ) {
        assert!(
            direction.length()
                > ORIENTATION_EPSILON,
            "mesh direction cannot be zero"
        );

        assert!(
            reference.length()
                > ORIENTATION_EPSILON,
            "mesh reference cannot be zero"
        );

        let direction =
            direction.normalized();

        let reference =
            orthogonalize_reference(
                direction,
                reference,
            );

        self.direction = direction;
        self.reference = reference;
    }

    /// Returns the third axis of the mesh's canonical orientation frame.
    pub fn side(&self) -> Vec3 {
        self.direction
            .cross(&self.reference)
            .normalized()
    }
}

impl Clone for Mesh {
    fn clone(&self) -> Self {
        Self {
            id: next_mesh_id(),

            vertex_generation: 0,
            index_generation: 0,

            vertices: self.vertices.clone(),
            indices: self.indices.clone(),

            pivot: self.pivot,

            direction: self.direction,
            reference: self.reference, 
        }
    }
}