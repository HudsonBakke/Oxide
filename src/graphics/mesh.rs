use std::{
    io::{Read, Write},
    path::Path,
    sync::{atomic::{AtomicU64, Ordering}, Arc, Weak},
};

use super::Vertex;

use crate::{
    content::{AssetType, ContentAsset, ContentAssetInternal, ContentError, ContentResult},
    math::{Vec2, Vec3, Vec4},
};

static NEXT_MESH_ID: AtomicU64 = AtomicU64::new(1);

/// Internal unique identifier for a mesh.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct MeshId(u64);

fn next_mesh_id() -> MeshId {
    MeshId(NEXT_MESH_ID.fetch_add(1, Ordering::Relaxed))
}

/// A collection of vertices and indices describing 3D geometry.
///
/// A mesh contains geometry only. It does not define where that geometry
/// appears in the world, how it is animated, or how it is rendered.
#[derive(Debug)]
pub struct Mesh {
    id: MeshId,
    lifetime: Arc<()>,

    vertex_generation: u64,
    index_generation: u64,

    vertices: Vec<Vertex>,
    indices: Vec<u32>,

    pivot: Vec3,

    direction: Vec3,
    reference: Vec3,
}

fn generate_tangents(vertices: &mut [Vertex], indices: &[u32]) {
    if vertices.is_empty() || indices.len() < 3 {
        return;
    }

    let mut tangents = vec![Vec3::default(); vertices.len()];
    let mut bitangents = vec![Vec3::default(); vertices.len()];

    for triangle in indices.chunks_exact(3) {
        let [i0, i1, i2] = [
            triangle[0] as usize,
            triangle[1] as usize,
            triangle[2] as usize,
        ];
        if i0 >= vertices.len() || i1 >= vertices.len() || i2 >= vertices.len() {
            continue;
        }

        let v0 = vertices[i0];
        let v1 = vertices[i1];
        let v2 = vertices[i2];
        let edge1 = v1.position - v0.position;
        let edge2 = v2.position - v0.position;
        let duv1 = v1.tex_coord - v0.tex_coord;
        let duv2 = v2.tex_coord - v0.tex_coord;
        let determinant = duv1.x * duv2.y - duv1.y * duv2.x;

        if determinant.abs() <= 1.0e-8 {
            continue;
        }

        let inverse = 1.0 / determinant;
        let tangent = (edge1 * duv2.y - edge2 * duv1.y) * inverse;
        let bitangent = (edge2 * duv1.x - edge1 * duv2.x) * inverse;

        for index in [i0, i1, i2] {
            tangents[index] = tangents[index] + tangent;
            bitangents[index] = bitangents[index] + bitangent;
        }
    }

    for (index, vertex) in vertices.iter_mut().enumerate() {
        // Preserve explicitly supplied tangents, such as glTF tangents.
        if Vec3::new(vertex.tangent.x, vertex.tangent.y, vertex.tangent.z).length() > 1.0e-6 {
            continue;
        }

        let normal = vertex.normal.normalized();
        let projected = tangents[index] - normal * normal.dot(&tangents[index]);
        let tangent = if projected.length() > 1.0e-6 {
            projected.normalized()
        } else {
            let axis = if normal.y.abs() < 0.999 {
                Vec3::new(0.0, 1.0, 0.0)
            } else {
                Vec3::new(1.0, 0.0, 0.0)
            };
            axis.cross(&normal).normalized()
        };

        let handedness = if normal.cross(&tangent).dot(&bitangents[index]) < 0.0 {
            -1.0
        } else {
            1.0
        };

        vertex.tangent = Vec4::new(tangent.x, tangent.y, tangent.z, handedness);
    }
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

fn orthogonalize_reference(direction: Vec3, reference: Vec3) -> crate::OxideResult<Vec3> {
    let projected = reference - direction * reference.dot(&direction);

    if projected.length() <= ORIENTATION_EPSILON {
        return Err(crate::OxideError::InvalidArgument {
            argument: "reference",

            message: "mesh reference cannot be parallel to mesh direction",
        });
    }

    Ok(projected.normalized())
}

impl Mesh {
    /// Creates a mesh from owned vertex/index arrays and calculates tangents and the average-vertex pivot.
    pub fn new(mut vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        generate_tangents(&mut vertices, &indices);
        let pivot = calculate_vertex_average(&vertices);

        Self {
            id: next_mesh_id(),
            lifetime: Arc::new(()),

            vertex_generation: 0,
            index_generation: 0,

            vertices,
            indices,

            pivot,

            direction: Vec3::new(0.0, 0.0, -1.0),

            reference: Vec3::new(0.0, 1.0, 0.0),
        }
    }

    fn from_compiled(
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
        pivot: Vec3,
        direction: Vec3,
        reference: Vec3,
    ) -> Self {
        let mut mesh = Self::new(vertices, indices);

        mesh.pivot = pivot;

        mesh.set_orientation(direction, reference)
            .expect("validated compiled mesh orientation should be valid");

        mesh
    }

    /// Model-space point placed at the transform's position when drawing or attaching this mesh.
    pub fn pivot(&self) -> Vec3 {
        self.pivot
    }

    /// Sets the model-space point used as the mesh's transformation origin.
    pub fn set_pivot(&mut self, pivot: Vec3) {
        self.pivot = pivot;
    }

    /// Sets the pivot to the arithmetic mean of all model-space vertex positions.
    pub fn recalculate_pivot(&mut self) {
        self.pivot = calculate_vertex_average(&self.vertices);
    }

    /// Regenerates tangents from positions, normals, and UVs after geometry changes.
    pub fn recalculate_tangents(&mut self) {
        for vertex in &mut self.vertices {
            vertex.tangent = Vec4::new(0.0, 0.0, 0.0, 1.0);
        }
        generate_tangents(&mut self.vertices, &self.indices);
        self.vertex_generation = self.vertex_generation.wrapping_add(1);
    }

    /// Borrowed vertex data in model space.
    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    /// Borrowed triangle indices into the vertex array.
    pub fn indices(&self) -> &[u32] {
        &self.indices
    }

    /// Returns mutable access to the mesh's vertices.
    ///
    /// Requesting mutable access marks the mesh geometry as changed.
    pub fn vertices_mut(&mut self) -> &mut [Vertex] {
        self.vertex_generation = self.vertex_generation.wrapping_add(1);

        &mut self.vertices
    }

    /// Returns mutable access to the mesh's indices.
    ///
    /// Requesting mutable access marks the mesh geometry as changed.
    pub fn indices_mut(&mut self) -> &mut [u32] {
        self.index_generation = self.index_generation.wrapping_add(1);

        &mut self.indices
    }

    /// Number of vertices in the mesh.
    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    /// Number of vertex indices; each group of three describes a triangle.
    pub fn index_count(&self) -> usize {
        self.indices.len()
    }

    pub(crate) fn lifetime(&self) -> Weak<()> {
        Arc::downgrade(&self.lifetime)
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

    /// Normalized forward direction of the mesh's model-space attachment frame.
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
    pub fn set_reference(&mut self, reference: Vec3) -> crate::OxideResult<()> {
        if reference.length() <= ORIENTATION_EPSILON {
            return Err(crate::OxideError::InvalidArgument {
                argument: "reference",

                message: "mesh reference cannot be zero",
            });
        }

        self.reference = orthogonalize_reference(self.direction, reference)?;

        Ok(())
    }

    /// Normalizes the nonzero model-space direction and rebuilds the attachment frame.
    pub fn set_direction(&mut self, direction: Vec3) -> crate::OxideResult<()> {
        if direction.length() <= ORIENTATION_EPSILON {
            return Err(crate::OxideError::InvalidArgument {
                argument: "direction",

                message: "mesh direction cannot be zero",
            });
        }

        let direction = direction.normalized();

        self.reference = orthogonalize_reference(direction, self.reference)?;

        self.direction = direction;

        Ok(())
    }

    /// Sets the mesh's canonical model-space orientation.
    ///
    /// `direction` defines the mesh's forward/longitudinal axis.
    /// `reference` defines its zero-twist orientation around that axis.
    ///
    /// Both vectors are normalized automatically, and `reference` is
    /// automatically made perpendicular to `direction`.
    pub fn set_orientation(&mut self, direction: Vec3, reference: Vec3) -> crate::OxideResult<()> {
        if direction.length() <= ORIENTATION_EPSILON {
            return Err(crate::OxideError::InvalidArgument {
                argument: "direction",

                message: "mesh direction cannot be zero",
            });
        }

        if reference.length() <= ORIENTATION_EPSILON {
            return Err(crate::OxideError::InvalidArgument {
                argument: "reference",

                message: "mesh reference cannot be zero",
            });
        }

        let direction = direction.normalized();

        let projected = reference - direction * reference.dot(&direction);

        if projected.length() <= ORIENTATION_EPSILON {
            return Err(crate::OxideError::InvalidArgument {
                argument: "reference",

                message: "mesh reference cannot be parallel to mesh direction",
            });
        }

        self.direction = direction;

        self.reference = projected.normalized();

        Ok(())
    }

    /// Normalized side axis computed from direction cross reference.
    pub fn side(&self) -> Vec3 {
        self.direction.cross(&self.reference).normalized()
    }
}

impl Clone for Mesh {
    fn clone(&self) -> Self {
        Self {
            id: next_mesh_id(),
            lifetime: Arc::new(()),

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

impl ContentAssetInternal for Mesh {
    fn compiler_version() -> u32 {
        4
    }

    fn asset_type_name() -> &'static str {
        "Mesh"
    }

    fn supports_extension(extension: &str) -> bool {
        matches!(extension, "gltf" | "glb")
    }

    fn import(path: &Path, subobject: &str) -> ContentResult<Self> {
        crate::content::import_gltf_mesh(path, subobject)
    }

    fn compiled_asset_type() -> AssetType {
        AssetType::Mesh
    }

    fn write_compiled(&self, writer: &mut impl Write) -> ContentResult<()> {
        write_mesh(self, writer)
    }

    fn read_compiled(reader: &mut impl Read, payload_size: u64) -> ContentResult<Self> {
        read_mesh(reader, payload_size)
    }
}

impl ContentAsset for Mesh {}

fn write_f32(writer: &mut impl Write, value: f32) -> ContentResult<()> {
    writer.write_all(&value.to_le_bytes())?;

    Ok(())
}

fn write_vec2(writer: &mut impl Write, value: Vec2) -> ContentResult<()> {
    write_f32(writer, value.x)?;

    write_f32(writer, value.y)
}

fn write_vec3(writer: &mut impl Write, value: Vec3) -> ContentResult<()> {
    write_f32(writer, value.x)?;

    write_f32(writer, value.y)?;

    write_f32(writer, value.z)
}

fn write_mesh(mesh: &Mesh, writer: &mut impl Write) -> ContentResult<()> {
    let vertex_count = u32::try_from(mesh.vertices.len())
        .map_err(|_| ContentError::InvalidCache("mesh contains too many vertices".into()))?;

    let index_count = u32::try_from(mesh.indices.len())
        .map_err(|_| ContentError::InvalidCache("mesh contains too many indices".into()))?;

    writer.write_all(&vertex_count.to_le_bytes())?;

    writer.write_all(&index_count.to_le_bytes())?;

    write_vec3(writer, mesh.pivot)?;

    write_vec3(writer, mesh.direction)?;

    write_vec3(writer, mesh.reference)?;

    for vertex in &mesh.vertices {
        write_vec3(writer, vertex.position)?;

        write_vec3(writer, vertex.normal)?;

        write_vec2(writer, vertex.tex_coord)?;
        write_f32(writer, vertex.tangent.x)?;
        write_f32(writer, vertex.tangent.y)?;
        write_f32(writer, vertex.tangent.z)?;
        write_f32(writer, vertex.tangent.w)?;
    }

    for index in &mesh.indices {
        writer.write_all(&index.to_le_bytes())?;
    }

    Ok(())
}

fn read_f32(reader: &mut impl Read) -> ContentResult<f32> {
    let mut bytes = [0u8; 4];

    reader.read_exact(&mut bytes)?;

    Ok(f32::from_le_bytes(bytes))
}

fn read_vec2(reader: &mut impl Read) -> ContentResult<Vec2> {
    Ok(Vec2::new(read_f32(reader)?, read_f32(reader)?))
}

fn read_vec3(reader: &mut impl Read) -> ContentResult<Vec3> {
    Ok(Vec3::new(
        read_f32(reader)?,
        read_f32(reader)?,
        read_f32(reader)?,
    ))
}

fn read_u32(reader: &mut impl Read) -> ContentResult<u32> {
    let mut bytes = [0u8; 4];

    reader.read_exact(&mut bytes)?;

    Ok(u32::from_le_bytes(bytes))
}

fn read_mesh(reader: &mut impl Read, payload_size: u64) -> ContentResult<Mesh> {
    // vertex_count + index_count
    // + pivot + direction + reference
    const FIXED_SIZE: u64 = 4 + 4 + 12 + 12 + 12;

    const VERTEX_SIZE: u64 = 48;

    const INDEX_SIZE: u64 = 4;

    if payload_size < FIXED_SIZE {
        return Err(ContentError::InvalidCache(
            "mesh payload is too small".into(),
        ));
    }

    let vertex_count = read_u32(reader)?;

    let index_count = read_u32(reader)?;

    let expected_size = FIXED_SIZE
        .checked_add(
            u64::from(vertex_count)
                .checked_mul(VERTEX_SIZE)
                .ok_or_else(|| {
                    ContentError::InvalidCache("mesh vertex data size overflow".into())
                })?,
        )
        .and_then(|size| {
            u64::from(index_count)
                .checked_mul(INDEX_SIZE)
                .and_then(|index_size| size.checked_add(index_size))
        })
        .ok_or_else(|| ContentError::InvalidCache("mesh payload size overflow".into()))?;

    if expected_size != payload_size {
        return Err(ContentError::InvalidCache(
            "mesh payload size does not match declared mesh data".into(),
        ));
    }

    let pivot = read_vec3(reader)?;

    let direction = read_vec3(reader)?;

    let reference = read_vec3(reader)?;

    let vertex_count = usize::try_from(vertex_count).map_err(|_| {
        ContentError::InvalidCache("mesh vertex count is too large for this platform".into())
    })?;

    let index_count = usize::try_from(index_count).map_err(|_| {
        ContentError::InvalidCache("mesh index count is too large for this platform".into())
    })?;

    let mut vertices = Vec::with_capacity(vertex_count);

    for _ in 0..vertex_count {
        let position = read_vec3(reader)?;

        let normal = read_vec3(reader)?;

        let tex_coord = read_vec2(reader)?;
        let tangent = Vec4::new(
            read_f32(reader)?,
            read_f32(reader)?,
            read_f32(reader)?,
            read_f32(reader)?,
        );

        vertices.push(Vertex::with_tangent(position, normal, tex_coord, tangent));
    }

    let mut indices = Vec::with_capacity(index_count);

    for _ in 0..index_count {
        indices.push(read_u32(reader)?);
    }

    // Don't trust a cache file merely because its byte count
    // is internally consistent.
    for &index in &indices {
        if index as usize >= vertices.len() {
            return Err(ContentError::InvalidCache(
                "compiled mesh contains an out-of-range index".into(),
            ));
        }
    }

    if direction.length() <= 1.0e-6 {
        return Err(ContentError::InvalidCache(
            "compiled mesh contains a zero canonical direction".into(),
        ));
    }

    if reference.length() <= 1.0e-6 {
        return Err(ContentError::InvalidCache(
            "compiled mesh contains a zero canonical reference".into(),
        ));
    }

    // set_orientation() will normalize and orthogonalize the
    // stored frame, while also rejecting a parallel frame.
    //
    // We don't want malformed cache data to panic, though, so
    // validate parallelism first.
    let normalized_direction = direction.normalized();

    let perpendicular_reference =
        reference - normalized_direction * reference.dot(&normalized_direction);

    if perpendicular_reference.length() <= 1.0e-6 {
        return Err(ContentError::InvalidCache(
            "compiled mesh contains parallel canonical orientation vectors".into(),
        ));
    }

    Ok(Mesh::from_compiled(
        vertices, indices, pivot, direction, reference,
    ))
}
