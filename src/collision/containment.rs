use crate::{
    graphics::Mesh,
    math::{
        Mat4,
        Vec3,
    },
};

use super::bvh::Bvh;

const RAY_DIRECTION: Vec3 = Vec3 {
    x: 0.8111071,
    y: 0.3244428,
    z: 0.4866643,
};

pub(super) fn point_inside_mesh(
    point: Vec3,
    mesh: &Mesh,
    matrix: &Mat4,
    bvh: &Bvh,
) -> bool {
    if bvh.nodes.is_empty() {
        return false;
    }

    let root_bounds =
        bvh.nodes[0]
            .bounds
            .transformed(matrix);

    if !root_bounds.intersects_ray(
        point,
        RAY_DIRECTION,
    ) {
        return false;
    }

    let mut stack = Vec::with_capacity(32);
    stack.push(0u32);

    let mut intersections =
        Vec::<f32>::new();

    while let Some(node_index) =
        stack.pop()
    {
        let node =
            &bvh.nodes[node_index as usize];

        let bounds =
            node.bounds.transformed(matrix);

        if !bounds.intersects_ray(
            point,
            RAY_DIRECTION,
        ) {
            continue;
        }

        if node.is_leaf() {
            let start =
                node.first as usize;

            let end =
                start + node.count as usize;

            for triangle_index
                in start..end
            {
                let triangle =
                    bvh.triangle(
                        mesh,
                        triangle_index,
                    )
                    .transformed(matrix);

                if let Some(distance) =
                    triangle
                        .ray_intersection_distance(
                            point,
                            RAY_DIRECTION,
                        )
                {
                    intersections.push(
                        distance
                    );
                }
            }
        } else {
            stack.push(node.first);
            stack.push(node.second);
        }
    }

    if intersections.is_empty() {
        return false;
    }

    intersections.sort_by(
        |a, b| {
            a.partial_cmp(b)
                .unwrap_or(
                    std::cmp::Ordering::Equal
                )
        }
    );

    let mut unique_count = 1usize;
    let mut previous =
        intersections[0];

    for &distance
        in &intersections[1..]
    {
        let tolerance =
            1.0e-5
            * previous
                .abs()
                .max(distance.abs())
                .max(1.0);

        if (
            distance - previous
        ).abs() > tolerance
        {
            unique_count += 1;
            previous = distance;
        }
    }

    unique_count % 2 == 1
}