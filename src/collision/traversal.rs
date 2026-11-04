use crate::{
    graphics::Mesh,
    math::Transform,
};

use super::bvh::Bvh;

const LOCAL_STACK_CAPACITY: usize = 64;

struct TraversalStack {
    local: [NodePair; LOCAL_STACK_CAPACITY],
    len: usize,

    overflow: Vec<NodePair>,
}

impl TraversalStack {
    fn new() -> Self {
        Self {
            local:
                [NodePair::default();
                    LOCAL_STACK_CAPACITY],

            len: 0,

            overflow: Vec::new(),
        }
    }

    fn push(
        &mut self,
        pair: NodePair,
    ) {
        if self.len
            < LOCAL_STACK_CAPACITY
        {
            self.local[self.len] = pair;
            self.len += 1;
        } else {
            self.overflow.push(pair);
        }
    }

    fn pop(&mut self) -> Option<NodePair> {
        if let Some(pair) =
            self.overflow.pop()
        {
            return Some(pair);
        }

        if self.len == 0 {
            return None;
        }

        self.len -= 1;

        Some(self.local[self.len])
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct NodePair {
    a: u32,
    b: u32,
}

pub(super) fn surfaces_intersect(
    mesh_a: &Mesh,
    transform_a: &Transform,
    bvh_a: &Bvh,

    mesh_b: &Mesh,
    transform_b: &Transform,
    bvh_b: &Bvh,
) -> bool {
    if bvh_a.nodes.is_empty()
        || bvh_b.nodes.is_empty()
    {
        return false;
    }

    let matrix_a =
        transform_a.matrix_around(
            mesh_a.pivot()
        );

    let matrix_b =
        transform_b.matrix_around(
            mesh_b.pivot()
        );

    let root_a =
        bvh_a.nodes[0]
            .bounds
            .transformed(&matrix_a);

    let root_b =
        bvh_b.nodes[0]
            .bounds
            .transformed(&matrix_b);

    // Cheapest possible rejection.
    if !root_a.intersects(&root_b) {
        return false;
    }

    let mut stack =
        TraversalStack::new();

    stack.push(
        NodePair {
            a: 0,
            b: 0,
        }
    );

    let mut overlapping_leaves = 0;

    while let Some(pair) = stack.pop() {
        let node_a =
            &bvh_a.nodes[pair.a as usize];

        let node_b =
            &bvh_b.nodes[pair.b as usize];

        let bounds_a =
            node_a
                .bounds
                .transformed(&matrix_a);

        let bounds_b =
            node_b
                .bounds
                .transformed(&matrix_b);

        if !bounds_a.intersects(
            &bounds_b
        ) {
            continue;
        }

        match (
            node_a.is_leaf(),
            node_b.is_leaf(),
        ) {
            (true, true) => {
                let start_a =
                    node_a.first as usize;

                let end_a =
                    start_a + node_a.count as usize;

                let start_b =
                    node_b.first as usize;

                let end_b =
                    start_b + node_b.count as usize;

                for triangle_a_index
                    in start_a..end_a
                {
                    let triangle_a =
                        bvh_a
                            .triangle(
                                mesh_a,
                                triangle_a_index,
                            )
                            .transformed(
                                &matrix_a
                            );

                    for triangle_b_index
                        in start_b..end_b
                    {
                        let triangle_b =
                            bvh_b
                                .triangle(
                                    mesh_b,
                                    triangle_b_index,
                                )
                                .transformed(
                                    &matrix_b
                                );

                        if triangle_a.intersects(
                            &triangle_b
                        ) {
                            return true;
                        }
                    }
                }
            }

            (true, false) => {
                stack.push(NodePair {
                    a: pair.a,
                    b: node_b.first,
                });

                stack.push(NodePair {
                    a: pair.a,
                    b: node_b.second,
                });
            }

            (false, true) => {
                stack.push(NodePair {
                    a: node_a.first,
                    b: pair.b,
                });

                stack.push(NodePair {
                    a: node_a.second,
                    b: pair.b,
                });
            }

            (false, false) => {
                if bounds_a.surface_area()
                    >= bounds_b.surface_area()
                {
                    stack.push(NodePair {
                        a: node_a.first,
                        b: pair.b,
                    });

                    stack.push(NodePair {
                        a: node_a.second,
                        b: pair.b,
                    });
                } else {
                    stack.push(NodePair {
                        a: pair.a,
                        b: node_b.first,
                    });

                    stack.push(NodePair {
                        a: pair.a,
                        b: node_b.second,
                    });
                }
            }
        }
    }

    false
}
