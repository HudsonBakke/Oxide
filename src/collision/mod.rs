mod aabb;
mod bvh;
mod traversal;
mod triangle;
mod cache;
mod containment;

use crate::{
    graphics::Mesh,
    math::Transform,
};

use cache::bvhs_for;
use traversal::surfaces_intersect;
use containment::point_inside_mesh;

/// Returns whether the surfaces of two transformed meshes intersect.
///
/// Touching at an edge or vertex counts as a collision.
pub fn collides(
    mesh_a: &Mesh,
    transform_a: &Transform,

    mesh_b: &Mesh,
    transform_b: &Transform,
) -> bool {
    let (bvh_a, bvh_b) =
        bvhs_for(
            mesh_a,
            mesh_b,
        );

    if surfaces_intersect(
        mesh_a,
        transform_a,
        &bvh_a,

        mesh_b,
        transform_b,
        &bvh_b,
    ) {
        return true;
    }

    let matrix_a =
        transform_a.matrix_around(
            mesh_a.pivot()
        );

    let matrix_b =
        transform_b.matrix_around(
            mesh_b.pivot()
        );

    let Some(vertex_a) =
        mesh_a.vertices().first()
    else {
        return false;
    };

    let Some(vertex_b) =
        mesh_b.vertices().first()
    else {
        return false;
    };

    let point_a =
        matrix_a.transform_point(
            vertex_a.position
        );

    let point_b =
        matrix_b.transform_point(
            vertex_b.position
        );

    point_inside_mesh(
        point_a,
        mesh_b,
        &matrix_b,
        &bvh_b,
    )
    ||
    point_inside_mesh(
        point_b,
        mesh_a,
        &matrix_a,
        &bvh_a,
    )
}
