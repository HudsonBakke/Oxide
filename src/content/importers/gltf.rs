use std::path::Path;

use crate::{
    content::{
        ContentError,
        ContentResult,
    },
    graphics::{
        Mesh,
        Vertex,
    },
    math::{
        Vec2,
        Vec3,
    },
};

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

    let selected_mesh =
        select_mesh(
            &document,
            path,
            subobject,
        )?;

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

fn select_mesh<'a>(
    document: &'a gltf::Document,
    path: &Path,
    subobject: &str,
) -> ContentResult<gltf::Mesh<'a>> {
    if !subobject.is_empty() {
        return document
            .meshes()
            .find(
                |mesh| {
                    mesh.name()
                        == Some(subobject)
                }
            )
            .ok_or_else(
                || {
                    ContentError::SubObjectNotFound {
                        path:
                            path.to_path_buf(),

                        asset_type:
                            "Mesh",

                        subobject:
                            subobject.to_string(),
                    }
                }
            );
    }

    let mut meshes =
        document.meshes();

    let Some(first) =
        meshes.next()
    else {
        return Err(
            ContentError::ImportFailed {
                path:
                    path.to_path_buf(),

                message:
                    "glTF file contains no meshes"
                        .into(),
            }
        );
    };

    if meshes.next().is_some() {
        return Err(
            ContentError::AmbiguousSubObject {
                path:
                    path.to_path_buf(),

                asset_type:
                    "Mesh",
            }
        );
    }

    Ok(first)
}

fn import_primitive(
    primitive: gltf::Primitive<'_>,
    buffers: &[gltf::buffer::Data],
    path: &Path,
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

    let normals: Vec<[f32; 3]> =
        match reader.read_normals() {
            Some(normals) =>
                normals.collect(),

            None =>
                vec![
                    [0.0, 0.0, 0.0];
                    positions.len()
                ],
        };

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

    for i in 0..positions.len() {
        let position =
            positions[i];

        let normal =
            normals[i];

        let tex_coord =
            tex_coords[i];

        vertices.push(
            Vertex::new(
                Vec3::new(
                    position[0],
                    position[1],
                    position[2],
                ),

                Vec3::new(
                    normal[0],
                    normal[1],
                    normal[2],
                ),

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
        for index in
            primitive_indices.into_u32()
        {
            let index =
                base_vertex
                    .checked_add(index)
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

            indices.push(index);
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

        for index in 0..vertex_count {
            indices.push(
                base_vertex + index
            );
        }
    }

    Ok(())
}