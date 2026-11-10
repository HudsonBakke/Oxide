use std::{
    sync::atomic::{
        AtomicU64,
        Ordering,
    },
    io::{
        Read, 
        Write,
    },
    path::Path,
};

use super::Vertex;

use crate::{
    content::{
        AssetType,
        ContentAsset,
        ContentError,
        ContentResult,
    },
    math::{
        Vec3,
        Vec2,
    },
};

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

    fn from_compiled(
        vertices: Vec<Vertex>,
        indices: Vec<u32>,
        pivot: Vec3,
        direction: Vec3,
        reference: Vec3,
    ) -> Self {
        let mut mesh =
            Self::new(
                vertices,
                indices,
            );

        mesh.pivot = pivot;

        mesh.set_orientation(
            direction,
            reference,
        );

        mesh
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

impl ContentAsset for Mesh {
    fn asset_type_name()
        -> &'static str
    {
        "Mesh"
    }

    fn supports_extension(
        extension: &str,
    ) -> bool {
        matches!(
            extension,
            "gltf" | "glb"
        )
    }

    fn import(
        path: &Path,
        subobject: &str,
    ) -> ContentResult<Self> {
        crate::content::import_gltf_mesh(
            path,
            subobject,
        )
    }

    fn compiled_asset_type()
        -> AssetType
    {
        AssetType::Mesh
    }

    fn write_compiled(
        &self,
        writer: &mut impl Write,
    ) -> ContentResult<()> {
        write_mesh(
            self,
            writer,
        )
    }

    fn read_compiled(
        reader: &mut impl Read,
        payload_size: u64,
    ) -> ContentResult<Self> {
        read_mesh(
            reader,
            payload_size,
        )
    }
}

fn write_f32(
    writer: &mut impl Write,
    value: f32,
) -> ContentResult<()> {
    writer.write_all(
        &value.to_le_bytes()
    )?;

    Ok(())
}

fn write_vec2(
    writer: &mut impl Write,
    value: Vec2,
) -> ContentResult<()> {
    write_f32(
        writer,
        value.x,
    )?;

    write_f32(
        writer,
        value.y,
    )
}

fn write_vec3(
    writer: &mut impl Write,
    value: Vec3,
) -> ContentResult<()> {
    write_f32(
        writer,
        value.x,
    )?;

    write_f32(
        writer,
        value.y,
    )?;

    write_f32(
        writer,
        value.z,
    )
}

fn write_mesh(
    mesh: &Mesh,
    writer: &mut impl Write,
) -> ContentResult<()> {
    let vertex_count =
        u32::try_from(
            mesh.vertices.len()
        )
        .map_err(
            |_| {
                ContentError::InvalidCache(
                    "mesh contains too many vertices"
                        .into()
                )
            }
        )?;

    let index_count =
        u32::try_from(
            mesh.indices.len()
        )
        .map_err(
            |_| {
                ContentError::InvalidCache(
                    "mesh contains too many indices"
                        .into()
                )
            }
        )?;

    writer.write_all(
        &vertex_count.to_le_bytes()
    )?;

    writer.write_all(
        &index_count.to_le_bytes()
    )?;

    write_vec3(
        writer,
        mesh.pivot,
    )?;

    write_vec3(
        writer,
        mesh.direction,
    )?;

    write_vec3(
        writer,
        mesh.reference,
    )?;

    for vertex in &mesh.vertices {
        write_vec3(
            writer,
            vertex.position,
        )?;

        write_vec3(
            writer,
            vertex.normal,
        )?;

        write_vec2(
            writer,
            vertex.tex_coord,
        )?;
    }

    for index in &mesh.indices {
        writer.write_all(
            &index.to_le_bytes()
        )?;
    }

    Ok(())
}

fn read_f32(
    reader: &mut impl Read,
) -> ContentResult<f32> {
    let mut bytes =
        [0u8; 4];

    reader.read_exact(
        &mut bytes
    )?;

    Ok(
        f32::from_le_bytes(
            bytes
        )
    )
}

fn read_vec2(
    reader: &mut impl Read,
) -> ContentResult<Vec2> {
    Ok(
        Vec2::new(
            read_f32(reader)?,
            read_f32(reader)?,
        )
    )
}

fn read_vec3(
    reader: &mut impl Read,
) -> ContentResult<Vec3> {
    Ok(
        Vec3::new(
            read_f32(reader)?,
            read_f32(reader)?,
            read_f32(reader)?,
        )
    )
}

fn read_u32(
    reader: &mut impl Read,
) -> ContentResult<u32> {
    let mut bytes =
        [0u8; 4];

    reader.read_exact(
        &mut bytes
    )?;

    Ok(
        u32::from_le_bytes(
            bytes
        )
    )
}

fn read_mesh(
    reader: &mut impl Read,
    payload_size: u64,
) -> ContentResult<Mesh> {
    // vertex_count + index_count
    // + pivot + direction + reference
    const FIXED_SIZE: u64 =
        4
        + 4
        + 12
        + 12
        + 12;

    const VERTEX_SIZE: u64 =
        32;

    const INDEX_SIZE: u64 =
        4;

    if payload_size < FIXED_SIZE {
        return Err(
            ContentError::InvalidCache(
                "mesh payload is too small"
                    .into()
            )
        );
    }

    let vertex_count =
        read_u32(reader)?;

    let index_count =
        read_u32(reader)?;

    let expected_size =
        FIXED_SIZE
            .checked_add(
                u64::from(vertex_count)
                    .checked_mul(
                        VERTEX_SIZE
                    )
                    .ok_or_else(
                        || {
                            ContentError::InvalidCache(
                                "mesh vertex data size overflow"
                                    .into()
                            )
                        }
                    )?
            )
            .and_then(
                |size| {
                    u64::from(index_count)
                        .checked_mul(
                            INDEX_SIZE
                        )
                        .and_then(
                            |index_size| {
                                size.checked_add(
                                    index_size
                                )
                            }
                        )
                }
            )
            .ok_or_else(
                || {
                    ContentError::InvalidCache(
                        "mesh payload size overflow"
                            .into()
                    )
                }
            )?;

    if expected_size
        != payload_size
    {
        return Err(
            ContentError::InvalidCache(
                "mesh payload size does not match declared mesh data"
                    .into()
            )
        );
    }

    let pivot =
        read_vec3(reader)?;

    let direction =
        read_vec3(reader)?;

    let reference =
        read_vec3(reader)?;

    let vertex_count =
        usize::try_from(
            vertex_count
        )
        .map_err(
            |_| {
                ContentError::InvalidCache(
                    "mesh vertex count is too large for this platform"
                        .into()
                )
            }
        )?;

    let index_count =
        usize::try_from(
            index_count
        )
        .map_err(
            |_| {
                ContentError::InvalidCache(
                    "mesh index count is too large for this platform"
                        .into()
                )
            }
        )?;

    let mut vertices =
        Vec::with_capacity(
            vertex_count
        );

    for _ in 0..vertex_count {
        let position =
            read_vec3(reader)?;

        let normal =
            read_vec3(reader)?;

        let tex_coord =
            read_vec2(reader)?;

        vertices.push(
            Vertex::new(
                position,
                normal,
                tex_coord,
            )
        );
    }

    let mut indices =
        Vec::with_capacity(
            index_count
        );

    for _ in 0..index_count {
        indices.push(
            read_u32(reader)?
        );
    }

    // Don't trust a cache file merely because its byte count
    // is internally consistent.
    for &index in &indices {
        if index as usize
            >= vertices.len()
        {
            return Err(
                ContentError::InvalidCache(
                    "compiled mesh contains an out-of-range index"
                        .into()
                )
            );
        }
    }

    if direction.length()
        <= 1.0e-6
    {
        return Err(
            ContentError::InvalidCache(
                "compiled mesh contains a zero canonical direction"
                    .into()
            )
        );
    }

    if reference.length()
        <= 1.0e-6
    {
        return Err(
            ContentError::InvalidCache(
                "compiled mesh contains a zero canonical reference"
                    .into()
            )
        );
    }

    // set_orientation() will normalize and orthogonalize the
    // stored frame, while also rejecting a parallel frame.
    //
    // We don't want malformed cache data to panic, though, so
    // validate parallelism first.
    let normalized_direction =
        direction.normalized();

    let perpendicular_reference =
        reference
            - normalized_direction
                * reference.dot(
                    &normalized_direction
                );

    if perpendicular_reference.length()
        <= 1.0e-6
    {
        return Err(
            ContentError::InvalidCache(
                "compiled mesh contains parallel canonical orientation vectors"
                    .into()
            )
        );
    }

    Ok(
        Mesh::from_compiled(
            vertices,
            indices,
            pivot,
            direction,
            reference,
        )
    )
}