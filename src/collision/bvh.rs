use crate::{
    graphics::Mesh,
    math::Vec3,
};

use super::{
    aabb::Aabb,
    triangle::Triangle,
};

const LEAF_TRIANGLE_COUNT: usize = 4;
const SAH_BIN_COUNT: usize = 16;

#[derive(Debug, Clone, Copy)]
struct TriangleInfo {
    indices: [u32; 3],
    bounds: Aabb,
    centroid: Vec3,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct BvhNode {
    pub bounds: Aabb,

    pub first: u32,
    pub second: u32,

    pub count: u32,
}

impl BvhNode {
    pub fn is_leaf(&self) -> bool {
        self.count != 0
    }
}

#[derive(Clone, Copy)]
struct Bin {
    bounds: Aabb,
    count: usize,
}

impl Bin {
    fn empty() -> Self {
        Self {
            bounds: Aabb::empty(),
            count: 0,
        }
    }
}

fn axis_value(
    value: Vec3,
    axis: usize,
) -> f32 {
    match axis {
        0 => value.x,
        1 => value.y,
        2 => value.z,
        _ => unreachable!(),
    }
}

pub(super) struct Bvh {
    pub nodes: Vec<BvhNode>,

    // Triangle index triplets arranged in leaf order.
    pub ordered_triangles: Vec<[u32; 3]>,
}

impl Bvh {
    pub fn build(mesh: &Mesh) -> Self {
        assert!(
            mesh.indices().len() % 3 == 0,
            "mesh index count must be divisible by 3"
        );

        let vertices = mesh.vertices();

        let mut triangles = Vec::with_capacity(
            mesh.indices().len() / 3
        );

        for indices in mesh.indices().chunks_exact(3) {
            let i0 = indices[0] as usize;
            let i1 = indices[1] as usize;
            let i2 = indices[2] as usize;

            assert!(
                i0 < vertices.len()
                    && i1 < vertices.len()
                    && i2 < vertices.len(),
                "mesh contains an out-of-range index"
            );

            let a = vertices[i0].position;
            let b = vertices[i1].position;
            let c = vertices[i2].position;

            let bounds =
                Aabb::from_triangle(a, b, c);

            triangles.push(
                TriangleInfo {
                    indices: [
                        indices[0],
                        indices[1],
                        indices[2],
                    ],

                    bounds,
                    centroid: bounds.centroid(),
                }
            );
        }

        if triangles.is_empty() {
            return Self {
                nodes: Vec::new(),
                ordered_triangles: Vec::new(),
            };
        }

        let mut bvh = Self {
            nodes: Vec::with_capacity(
                triangles.len() * 2
            ),

            ordered_triangles:
                Vec::with_capacity(
                    triangles.len()
                ),
        };

        bvh.build_node(&mut triangles);

        bvh
    }

    fn build_node(
        &mut self,
        triangles: &mut [TriangleInfo],
    ) -> u32 {
        let node_index = self.nodes.len() as u32;

        // Reserve the node before building children.
        self.nodes.push(
            BvhNode {
                bounds: Aabb::empty(),
                first: 0,
                second: 0,
                count: 0,
            }
        );

        let mut bounds = Aabb::empty();
        let mut centroid_bounds = Aabb::empty();

        for triangle in triangles.iter() {
            bounds.expand_aabb(&triangle.bounds);
            centroid_bounds.expand_point(
                triangle.centroid
            );
        }

        if triangles.len() <= LEAF_TRIANGLE_COUNT {
            self.make_leaf(
                node_index,
                bounds,
                triangles,
            );

            return node_index;
        }

        let Some((axis, split_position)) =
            Self::find_best_split(
                triangles,
                &centroid_bounds,
            )
        else {
            self.make_leaf(
                node_index,
                bounds,
                triangles,
            );

            return node_index;
        };

        let mut middle = 0;

        for i in 0..triangles.len() {
            if axis_value(
                triangles[i].centroid,
                axis,
            ) < split_position
            {
                triangles.swap(i, middle);
                middle += 1;
            }
        }

        // Degenerate split: don't recurse forever.
        if middle == 0
            || middle == triangles.len()
        {
            self.make_leaf(
                node_index,
                bounds,
                triangles,
            );

            return node_index;
        }

        let (left, right) =
            triangles.split_at_mut(middle);

        let left_index =
            self.build_node(left);

        let right_index =
            self.build_node(right);

        debug_assert_eq!(
            right_index,
            left_index + 1
        );

        self.nodes[node_index as usize] =
            BvhNode {
                bounds,
                first: left_index,
                second: right_index,
                count: 0,
            };

        node_index
    }

    fn make_leaf(
        &mut self,
        node_index: u32,
        bounds: Aabb,
        triangles: &[TriangleInfo],
    ) {
        let first =
            self.ordered_triangles.len() as u32;

        for triangle in triangles {
            self.ordered_triangles.push(
                triangle.indices
            );
        }

        self.nodes[node_index as usize] =
            BvhNode {
                bounds,
                first,
                second: 0,
                count: triangles.len() as u32,
            };
    }

    fn find_best_split(
        triangles: &[TriangleInfo],
        centroid_bounds: &Aabb,
    ) -> Option<(usize, f32)> {
        let mut best_cost = f32::INFINITY;
        let mut best_axis = 0;
        let mut best_split = 0.0;
        let mut found = false;

        for axis in 0..3 {
            let min =
                axis_value(centroid_bounds.min, axis);

            let max =
                axis_value(centroid_bounds.max, axis);

            let extent = max - min;

            if extent <= f32::EPSILON {
                continue;
            }

            let mut bins =
                [Bin::empty(); SAH_BIN_COUNT];

            for triangle in triangles {
                let centroid =
                    axis_value(
                        triangle.centroid,
                        axis,
                    );

                let normalized =
                    (centroid - min) / extent;

                let bin_index = (
                    normalized
                        * SAH_BIN_COUNT as f32
                ) as usize;

                let bin_index =
                    bin_index.min(SAH_BIN_COUNT - 1);

                bins[bin_index].count += 1;

                bins[bin_index]
                    .bounds
                    .expand_aabb(
                        &triangle.bounds
                    );
            }

            let mut left_bounds =
                [Aabb::empty(); SAH_BIN_COUNT];

            let mut right_bounds =
                [Aabb::empty(); SAH_BIN_COUNT];

            let mut left_counts =
                [0usize; SAH_BIN_COUNT];

            let mut right_counts =
                [0usize; SAH_BIN_COUNT];

            // Prefix information.
            let mut bounds = Aabb::empty();
            let mut count = 0;

            for i in 0..SAH_BIN_COUNT {
                if bins[i].count > 0 {
                    bounds.expand_aabb(
                        &bins[i].bounds
                    );

                    count += bins[i].count;
                }

                left_bounds[i] = bounds;
                left_counts[i] = count;
            }

            // Suffix information.
            bounds = Aabb::empty();
            count = 0;

            for i in (0..SAH_BIN_COUNT).rev() {
                if bins[i].count > 0 {
                    bounds.expand_aabb(
                        &bins[i].bounds
                    );

                    count += bins[i].count;
                }

                right_bounds[i] = bounds;
                right_counts[i] = count;
            }

            // Split between bin i - 1 and bin i.
            for split in 1..SAH_BIN_COUNT {
                let left_count =
                    left_counts[split - 1];

                let right_count =
                    right_counts[split];

                if left_count == 0
                    || right_count == 0
                {
                    continue;
                }

                let cost =
                    left_bounds[split - 1]
                        .surface_area()
                        * left_count as f32
                    +
                    right_bounds[split]
                        .surface_area()
                        * right_count as f32;

                if cost < best_cost {
                    best_cost = cost;
                    best_axis = axis;

                    best_split =
                        min
                        + extent
                            * (
                                split as f32
                                / SAH_BIN_COUNT as f32
                            );

                    found = true;
                }
            }
        }

        let parent_bounds = {
            let mut bounds = Aabb::empty();

            for triangle in triangles {
                bounds.expand_aabb(
                    &triangle.bounds
                );
            }

            bounds
        };

        let leaf_cost =
            parent_bounds.surface_area()
                * triangles.len() as f32;

        if found && best_cost < leaf_cost {
            Some((
                best_axis,
                best_split,
            ))
        } else {
            None
        }
    }

    pub(super) fn triangle(
        &self,
        mesh: &Mesh,
        triangle_index: usize,
    ) -> Triangle {
        let indices =
            self.ordered_triangles[triangle_index];

        let vertices = mesh.vertices();

        Triangle::new(
            vertices[indices[0] as usize].position,
            vertices[indices[1] as usize].position,
            vertices[indices[2] as usize].position,
        )
    }
}
