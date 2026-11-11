use std::path::Path;

use crate::{
    content::{
        ContentError,
        ContentResult,
    },
    graphics::{
        Mesh,
        Vertex,
        Texture,
        Color,
        Material,
    },
    math::{
        Vec2,
        Vec3,
    },
};

type Matrix4 = [[f32; 4]; 4];
type Matrix3 = [[f32; 3]; 3];

const IDENTITY_MATRIX: Matrix4 = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];

fn linear_matrix(
    matrix: &Matrix4,
) -> Matrix3 {
    [
        [
            matrix[0][0],
            matrix[0][1],
            matrix[0][2],
        ],
        [
            matrix[1][0],
            matrix[1][1],
            matrix[1][2],
        ],
        [
            matrix[2][0],
            matrix[2][1],
            matrix[2][2],
        ],
    ]
}

fn inverse_matrix3(
    matrix: &Matrix3,
) -> Option<Matrix3> {
    let a = matrix[0][0];
    let b = matrix[1][0];
    let c = matrix[2][0];

    let d = matrix[0][1];
    let e = matrix[1][1];
    let f = matrix[2][1];

    let g = matrix[0][2];
    let h = matrix[1][2];
    let i = matrix[2][2];

    let determinant =
        a * (e * i - f * h)
        - b * (d * i - f * g)
        + c * (d * h - e * g);

    if determinant.abs()
        <= 1.0e-8
    {
        return None;
    }

    let inverse_determinant =
        1.0 / determinant;

    Some([
        [
            (e * i - f * h)
                * inverse_determinant,

            (f * g - d * i)
                * inverse_determinant,

            (d * h - e * g)
                * inverse_determinant,
        ],
        [
            (c * h - b * i)
                * inverse_determinant,

            (a * i - c * g)
                * inverse_determinant,

            (b * g - a * h)
                * inverse_determinant,
        ],
        [
            (b * f - c * e)
                * inverse_determinant,

            (c * d - a * f)
                * inverse_determinant,

            (a * e - b * d)
                * inverse_determinant,
        ],
    ])
}

fn transpose_matrix3(
    matrix: &Matrix3,
) -> Matrix3 {
    [
        [
            matrix[0][0],
            matrix[1][0],
            matrix[2][0],
        ],
        [
            matrix[0][1],
            matrix[1][1],
            matrix[2][1],
        ],
        [
            matrix[0][2],
            matrix[1][2],
            matrix[2][2],
        ],
    ]
}

fn multiply_matrices(
    a: &Matrix4,
    b: &Matrix4,
) -> Matrix4 {
    let mut result =
        [[0.0; 4]; 4];

    for column in 0..4 {
        for row in 0..4 {
            result[column][row] =
                a[0][row] * b[column][0]
                + a[1][row] * b[column][1]
                + a[2][row] * b[column][2]
                + a[3][row] * b[column][3];
        }
    }

    result
}

fn transform_position(
    matrix: &[[f32; 4]; 4],
    position: [f32; 3],
) -> Vec3 {
    Vec3::new(
        matrix[0][0] * position[0]
            + matrix[1][0] * position[1]
            + matrix[2][0] * position[2]
            + matrix[3][0],

        matrix[0][1] * position[0]
            + matrix[1][1] * position[1]
            + matrix[2][1] * position[2]
            + matrix[3][1],

        matrix[0][2] * position[0]
            + matrix[1][2] * position[1]
            + matrix[2][2] * position[2]
            + matrix[3][2],
    )
}

fn transform_normal(
    matrix: &Matrix3,
    normal: [f32; 3],
) -> Vec3 {
    Vec3::new(
        matrix[0][0] * normal[0]
            + matrix[1][0] * normal[1]
            + matrix[2][0] * normal[2],

        matrix[0][1] * normal[0]
            + matrix[1][1] * normal[1]
            + matrix[2][1] * normal[2],

        matrix[0][2] * normal[0]
            + matrix[1][2] * normal[1]
            + matrix[2][2] * normal[2],
    )
    .normalized()
}

pub(crate) fn import_mesh(
    path: &Path,
    subobject: &str,
) -> ContentResult<Mesh> {
    let (
        document,
        buffers,
        _images,
    ) = gltf::import(path)
        .map_err(
            |error| {
                ContentError::ImportFailed {
                    path:
                        path.to_path_buf(),

                    message:
                        error.to_string(),
                }
            }
        )?;

    let selected =
        select_mesh_node(
            &document,
            path,
            subobject,
        )?;

    let selected_mesh =
        selected
            .node
            .mesh()
            .expect(
                "selected mesh node must contain a mesh"
            );

    let world_transform =
        selected.world_transform;

    let linear =
        linear_matrix(
            &world_transform
        );

    let inverse =
        inverse_matrix3(
            &linear
        )
        .ok_or_else(
            || {
                ContentError::ImportFailed {
                    path:
                        path.to_path_buf(),

                    message:
                        "mesh object has a non-invertible transform"
                            .into(),
                }
            }
        )?;

    let normal_transform =
        transpose_matrix3(
            &inverse
        );

    let mut vertices =
        Vec::new();

    let mut indices =
        Vec::new();

    for primitive
        in selected_mesh.primitives()
    {
        import_primitive(
            primitive,
            &buffers,
            path,

            &world_transform,
            &normal_transform,

            &mut vertices,
            &mut indices,
        )?;
    }

    if vertices.is_empty() {
        return Err(
            ContentError::ImportFailed {
                path:
                    path.to_path_buf(),

                message:
                    "selected glTF mesh contains no vertices"
                        .into(),
            }
        );
    }

    if indices.is_empty() {
        return Err(
            ContentError::ImportFailed {
                path:
                    path.to_path_buf(),

                message:
                    "selected glTF mesh contains no triangles"
                        .into(),
            }
        );
    }

    Ok(
        Mesh::new(
            vertices,
            indices,
        )
    )
}

pub(crate) fn import_texture(
    path: &Path,
    subobject: &str,
) -> ContentResult<Texture> {
    let (
        document,
        _buffers,
        images,
    ) = gltf::import(path)
        .map_err(
            |error| {
                ContentError::ImportFailed {
                    path:
                        path.to_path_buf(),

                    message:
                        error.to_string(),
                }
            }
        )?;

    let texture =
        select_texture(
            &document,
            path,
            subobject,
        )?;

    let source_index =
        texture.source().index();

    let image =
        images.get(
            source_index
        )
        .ok_or_else(
            || {
                ContentError::ImportFailed {
                    path:
                        path.to_path_buf(),

                    message:
                        "glTF texture references missing image data"
                            .into(),
                }
            }
        )?;

    let pixels =
        convert_image_to_rgba(
            image,
            path,
        )?;

    Ok(
        Texture::from_rgba(
            image.width,
            image.height,
            pixels,
        )
    )
}

pub(crate) fn import_material(
    path: &Path,
    subobject: &str,
) -> ContentResult<Material> {
    let (
        document,
        _buffers,
        images,
    ) = gltf::import(path)
        .map_err(
            |error| {
                ContentError::ImportFailed {
                    path:
                        path.to_path_buf(),

                    message:
                        error.to_string(),
                }
            }
        )?;

    let material =
        select_material(
            &document,
            path,
            subobject,
        )?;

    let pbr =
        material
            .pbr_metallic_roughness();

    let factor =
        pbr.base_color_factor();

    let texture =
        if let Some(info) =
            pbr.base_color_texture()
        {
            let source_index =
                info
                    .texture()
                    .source()
                    .index();

            let image =
                images.get(
                    source_index
                )
                .ok_or_else(
                    || {
                        ContentError::ImportFailed {
                            path:
                                path.to_path_buf(),

                            message:
                                "glTF material references missing image data"
                                    .into(),
                        }
                    }
                )?;

            let mut pixels =
                convert_image_to_rgba(
                    image,
                    path,
                )?;

            // glTF's base-color factor multiplies the
            // base-color texture.
            apply_base_color_factor(
                &mut pixels,
                factor,
            );

            Texture::from_rgba(
                image.width,
                image.height,
                pixels,
            )
        } else {
            Texture::solid(
                Color::rgba(
                    factor[0],
                    factor[1],
                    factor[2],
                    factor[3],
                )
            )
        };

    Ok(
        Material::new(
            texture
        )
    )
}

fn apply_base_color_factor(
    pixels: &mut [u8],
    factor: [f32; 4],
) {
    for pixel
        in pixels.chunks_exact_mut(4)
    {
        for channel in 0..4 {
            let value =
                pixel[channel] as f32
                * factor[channel];

            pixel[channel] =
                value
                    .clamp(
                        0.0,
                        255.0,
                    )
                    .round()
                    as u8;
        }
    }
}

struct MeshNode<'a> {
    node: gltf::Node<'a>,
    world_transform: Matrix4,
}

fn collect_mesh_nodes<'a>(
    node: gltf::Node<'a>,
    parent_transform: &Matrix4,
    results: &mut Vec<MeshNode<'a>>,
) {
    let local_transform =
        node.transform().matrix();

    let world_transform =
        multiply_matrices(
            parent_transform,
            &local_transform,
        );

    if node.mesh().is_some() {
        results.push(
            MeshNode {
                node: node.clone(),
                world_transform,
            }
        );
    }

    for child in node.children() {
        collect_mesh_nodes(
            child,
            &world_transform,
            results,
        );
    }
}

fn collect_scene_mesh_nodes<'a>(
    document: &'a gltf::Document,
) -> Vec<MeshNode<'a>> {
    let mut results =
        Vec::new();

    // Prefer the glTF default scene when one exists.
    //
    // If there is no default scene, traverse all scenes.
    if let Some(scene) =
        document.default_scene()
    {
        for node in scene.nodes() {
            collect_mesh_nodes(
                node,
                &IDENTITY_MATRIX,
                &mut results,
            );
        }
    } else {
        for scene in document.scenes() {
            for node in scene.nodes() {
                collect_mesh_nodes(
                    node,
                    &IDENTITY_MATRIX,
                    &mut results,
                );
            }
        }
    }

    results
}

fn select_mesh_node<'a>(
    document: &'a gltf::Document,
    path: &Path,
    subobject: &str,
) -> ContentResult<MeshNode<'a>> {
    let nodes =
        collect_scene_mesh_nodes(
            document
        );

    if !subobject.is_empty() {
        let mut matches =
            nodes
                .into_iter()
                .filter(
                    |mesh_node| {
                        mesh_node
                            .node
                            .name()
                            == Some(subobject)
                    }
                );

        let Some(selected) =
            matches.next()
        else {
            return Err(
                ContentError::SubObjectNotFound {
                    path:
                        path.to_path_buf(),

                    asset_type:
                        "Mesh",

                    subobject:
                        subobject.to_string(),
                }
            );
        };

        if matches.next().is_some() {
            return Err(
                ContentError::AmbiguousSubObject {
                    path:
                        path.to_path_buf(),

                    asset_type:
                        "Mesh",
                }
            );
        }

        return Ok(selected);
    }

    let mut nodes =
        nodes.into_iter();

    let Some(selected) =
        nodes.next()
    else {
        return Err(
            ContentError::ImportFailed {
                path:
                    path.to_path_buf(),

                message:
                    "glTF scene contains no mesh objects"
                        .into(),
            }
        );
    };

    if nodes.next().is_some() {
        return Err(
            ContentError::AmbiguousSubObject {
                path:
                    path.to_path_buf(),

                asset_type:
                    "Mesh",
            }
        );
    }

    Ok(selected)
}

fn generate_normals(
    vertices: &mut [Vertex],
    indices: &[u32],
    base_vertex: u32,
) {
    // Start with no accumulated surface normal.
    for vertex in vertices.iter_mut() {
        vertex.normal =
            Vec3::new(
                0.0,
                0.0,
                0.0,
            );
    }

    for triangle
        in indices.chunks_exact(3)
    {
        let i0 =
            triangle[0]
                .checked_sub(
                    base_vertex
                )
                .expect(
                    "primitive index precedes its base vertex"
                )
                as usize;

        let i1 =
            triangle[1]
                .checked_sub(
                    base_vertex
                )
                .expect(
                    "primitive index precedes its base vertex"
                )
                as usize;

        let i2 =
            triangle[2]
                .checked_sub(
                    base_vertex
                )
                .expect(
                    "primitive index precedes its base vertex"
                )
                as usize;

        let a =
            vertices[i0].position;

        let b =
            vertices[i1].position;

        let c =
            vertices[i2].position;

        let edge_ab =
            b - a;

        let edge_ac =
            c - a;

        let face_normal =
            edge_ab.cross(
                &edge_ac
            );

        // A zero cross product means the triangle is
        // degenerate and contributes no useful normal.
        if face_normal.length()
            <= 1.0e-8
        {
            continue;
        }

        // Deliberately do NOT normalize here.
        //
        // The cross-product magnitude is proportional to
        // triangle area, so larger triangles contribute more
        // strongly to the resulting smooth vertex normal.
        vertices[i0].normal =
            vertices[i0].normal
                + face_normal;

        vertices[i1].normal =
            vertices[i1].normal
                + face_normal;

        vertices[i2].normal =
            vertices[i2].normal
                + face_normal;
    }

    for vertex in vertices.iter_mut() {
        if vertex.normal.length()
            > 1.0e-8
        {
            vertex.normal =
                vertex.normal
                    .normalized();
        }
    }
}

fn select_texture<'a>(
    document: &'a gltf::Document,
    path: &Path,
    subobject: &str,
) -> ContentResult<gltf::Texture<'a>> {
    if !subobject.is_empty() {
        let mut matches =
            document
                .textures()
                .filter(
                    |texture| {
                        texture.name()
                            == Some(subobject)
                    }
                );

        let Some(selected) =
            matches.next()
        else {
            return Err(
                ContentError::SubObjectNotFound {
                    path:
                        path.to_path_buf(),

                    asset_type:
                        "Texture",

                    subobject:
                        subobject.to_string(),
                }
            );
        };

        if matches.next().is_some() {
            return Err(
                ContentError::AmbiguousSubObject {
                    path:
                        path.to_path_buf(),

                    asset_type:
                        "Texture",
                }
            );
        }

        return Ok(selected);
    }

    let mut textures =
        document.textures();

    let Some(selected) =
        textures.next()
    else {
        return Err(
            ContentError::ImportFailed {
                path:
                    path.to_path_buf(),

                message:
                    "glTF file contains no textures"
                        .into(),
            }
        );
    };

    if textures.next().is_some() {
        return Err(
            ContentError::AmbiguousSubObject {
                path:
                    path.to_path_buf(),

                asset_type:
                    "Texture",
            }
        );
    }

    Ok(selected)
}

fn select_material<'a>(
    document: &'a gltf::Document,
    path: &Path,
    subobject: &str,
) -> ContentResult<gltf::Material<'a>> {
    if !subobject.is_empty() {
        let mut matches =
            document
                .materials()
                .filter(
                    |material| {
                        material.name()
                            == Some(subobject)
                    }
                );

        let Some(selected) =
            matches.next()
        else {
            return Err(
                ContentError::SubObjectNotFound {
                    path:
                        path.to_path_buf(),

                    asset_type:
                        "Material",

                    subobject:
                        subobject.to_string(),
                }
            );
        };

        if matches.next().is_some() {
            return Err(
                ContentError::AmbiguousSubObject {
                    path:
                        path.to_path_buf(),

                    asset_type:
                        "Material",
                }
            );
        }

        return Ok(selected);
    }

    let mut materials =
        document.materials();

    let Some(selected) =
        materials.next()
    else {
        return Err(
            ContentError::ImportFailed {
                path:
                    path.to_path_buf(),

                message:
                    "glTF file contains no materials"
                        .into(),
            }
        );
    };

    if materials.next().is_some() {
        return Err(
            ContentError::AmbiguousSubObject {
                path:
                    path.to_path_buf(),

                asset_type:
                    "Material",
            }
        );
    }

    Ok(selected)
}

fn convert_image_to_rgba(
    image: &gltf::image::Data,
    path: &Path,
) -> ContentResult<Vec<u8>> {
    use gltf::image::Format;

    let pixel_count =
        u64::from(image.width)
            .checked_mul(
                u64::from(image.height)
            )
            .ok_or_else(
                || {
                    ContentError::ImportFailed {
                        path:
                            path.to_path_buf(),

                        message:
                            "glTF texture dimensions overflow"
                                .into(),
                    }
                }
            )?;

    let capacity =
        pixel_count
            .checked_mul(4)
            .and_then(
                |value| {
                    usize::try_from(value)
                        .ok()
                }
            )
            .ok_or_else(
                || {
                    ContentError::ImportFailed {
                        path:
                            path.to_path_buf(),

                        message:
                            "glTF texture is too large"
                                .into(),
                    }
                }
            )?;

    let mut rgba =
        Vec::with_capacity(
            capacity
        );

    match image.format {
        Format::R8 => {
            for &r in &image.pixels {
                rgba.extend_from_slice(
                    &[r, r, r, 255]
                );
            }
        }

        Format::R8G8 => {
            for pixel
                in image.pixels
                    .chunks_exact(2)
            {
                rgba.extend_from_slice(
                    &[
                        pixel[0],
                        pixel[0],
                        pixel[0],
                        pixel[1],
                    ]
                );
            }
        }

        Format::R8G8B8 => {
            for pixel
                in image.pixels
                    .chunks_exact(3)
            {
                rgba.extend_from_slice(
                    &[
                        pixel[0],
                        pixel[1],
                        pixel[2],
                        255,
                    ]
                );
            }
        }

        Format::R8G8B8A8 => {
            rgba.extend_from_slice(
                &image.pixels
            );
        }

        _ => {
            return Err(
                ContentError::ImportFailed {
                    path:
                        path.to_path_buf(),

                    message:
                        format!(
                            "unsupported glTF image format {:?}",
                            image.format
                        ),
                }
            );
        }
    }

    Ok(rgba)
}

fn import_primitive(
    primitive: gltf::Primitive<'_>,
    buffers: &[gltf::buffer::Data],
    path: &Path,

    world_transform: &Matrix4,
    normal_transform: &Matrix3,

    vertices: &mut Vec<Vertex>,
    indices: &mut Vec<u32>,
) -> ContentResult<()> {
    use gltf::mesh::Mode;

    if primitive.mode()
        != Mode::Triangles
    {
        return Err(
            ContentError::ImportFailed {
                path:
                    path.to_path_buf(),

                message:
                    format!(
                        "unsupported glTF primitive mode {:?}; Oxide currently requires triangles",
                        primitive.mode()
                    ),
            }
        );
    }

    let reader =
        primitive.reader(
            |buffer| {
                Some(
                    &buffers[
                        buffer.index()
                    ]
                )
            }
        );

    let positions: Vec<[f32; 3]> =
        reader
            .read_positions()
            .ok_or_else(
                || {
                    ContentError::ImportFailed {
                        path:
                            path.to_path_buf(),

                        message:
                            "glTF mesh primitive has no POSITION attribute"
                                .into(),
                    }
                }
            )?
            .collect();

    let normals: Option<Vec<[f32; 3]>> =
        reader
            .read_normals()
            .map(
                |normals| {
                    normals.collect()
                }
            );

    let tex_coords: Vec<[f32; 2]> =
        match reader
            .read_tex_coords(0)
        {
            Some(tex_coords) =>
                tex_coords
                    .into_f32()
                    .collect(),

            None =>
                vec![
                    [0.0, 0.0];
                    positions.len()
                ],
        };

    if let Some(normals) =
        &normals
    {
        if normals.len()
            != positions.len()
        {
            return Err(
                ContentError::ImportFailed {
                    path:
                        path.to_path_buf(),

                    message:
                        "glTF NORMAL count does not match POSITION count"
                            .into(),
                }
            );
        }
    }

    if tex_coords.len()
        != positions.len()
    {
        return Err(
            ContentError::ImportFailed {
                path:
                    path.to_path_buf(),

                message:
                    "glTF TEXCOORD_0 count does not match POSITION count"
                        .into(),
            }
        );
    }

    let base_vertex =
        u32::try_from(
            vertices.len()
        )
        .map_err(
            |_| {
                ContentError::ImportFailed {
                    path:
                        path.to_path_buf(),

                    message:
                        "mesh contains too many vertices"
                            .into(),
                }
            }
        )?;

    let first_vertex =
        vertices.len();

    let first_index =
        indices.len();

    for i in 0..positions.len() {
        let position =
            positions[i];

        let tex_coord =
            tex_coords[i];

        let transformed_position =
            transform_position(
                world_transform,
                position,
            );

        let transformed_normal =
            match &normals {
                Some(normals) => {
                    transform_normal(
                        normal_transform,
                        normals[i],
                    )
                }

                None => {
                    Vec3::new(
                        0.0,
                        0.0,
                        0.0,
                    )
                }
            };

        vertices.push(
            Vertex::new(
                transformed_position,
                transformed_normal,

                Vec2::new(
                    tex_coord[0],
                    tex_coord[1],
                ),
            )
        );
    }

    if let Some(
        primitive_indices
    ) = reader.read_indices()
    {
        let primitive_indices:
            Vec<u32> =
            primitive_indices
                .into_u32()
                .collect();

        if primitive_indices.len() % 3
            != 0
        {
            return Err(
                ContentError::ImportFailed {
                    path:
                        path.to_path_buf(),

                    message:
                        "triangle primitive index count is not divisible by 3"
                            .into(),
                }
            );
        }

        for triangle
            in primitive_indices.chunks_exact(3)
        {
            let i0 =
                base_vertex
                    .checked_add(
                        triangle[0]
                    )
                    .ok_or_else(
                        || {
                            ContentError::ImportFailed {
                                path:
                                    path.to_path_buf(),

                                message:
                                    "mesh index overflow"
                                        .into(),
                            }
                        }
                    )?;

            let i1 =
                base_vertex
                    .checked_add(
                        triangle[1]
                    )
                    .ok_or_else(
                        || {
                            ContentError::ImportFailed {
                                path:
                                    path.to_path_buf(),

                                message:
                                    "mesh index overflow"
                                        .into(),
                            }
                        }
                    )?;

            let i2 =
                base_vertex
                    .checked_add(
                        triangle[2]
                    )
                    .ok_or_else(
                        || {
                            ContentError::ImportFailed {
                                path:
                                    path.to_path_buf(),

                                message:
                                    "mesh index overflow"
                                        .into(),
                            }
                        }
                    )?;

            // glTF and Oxide currently use opposite
            // front-face winding conventions.
            indices.extend_from_slice(
                &[
                    i0,
                    i2,
                    i1,
                ]
            );
        }
    } else {
        // glTF permits non-indexed primitives.
        let vertex_count =
            u32::try_from(
                positions.len()
            )
            .map_err(
                |_| {
                    ContentError::ImportFailed {
                        path:
                            path.to_path_buf(),

                        message:
                            "mesh contains too many vertices"
                                .into(),
                    }
                }
            )?;

        if vertex_count % 3 != 0 {
            return Err(
                ContentError::ImportFailed {
                    path:
                        path.to_path_buf(),

                    message:
                        "non-indexed triangle primitive vertex count is not divisible by 3"
                            .into(),
                }
            );
        }

        for triangle_start
            in (0..vertex_count)
                .step_by(3)
        {
            indices.extend_from_slice(
                &[
                    base_vertex
                        + triangle_start,

                    base_vertex
                        + triangle_start
                        + 2,

                    base_vertex
                        + triangle_start
                        + 1,
                ]
            );
        }
    }

    if normals.is_none() {
        generate_normals(
            &mut vertices[
                first_vertex..
            ],

            &indices[
                first_index..
            ],

            base_vertex,
        );
    }

    Ok(())
}