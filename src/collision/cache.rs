use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

use crate::graphics::{
    mesh::MeshId,
    Mesh,
};

use super::bvh::Bvh;

struct CachedBvh {
    vertex_generation: u64,
    index_generation: u64,
    bvh: Rc<Bvh>,
}

thread_local! {
    static BVH_CACHE:
        RefCell<HashMap<MeshId, CachedBvh>> =
            RefCell::new(HashMap::new());
}

pub(super) fn bvhs_for(
    mesh_a: &Mesh,
    mesh_b: &Mesh,
) -> (Rc<Bvh>, Rc<Bvh>) {
    BVH_CACHE.with(|cache| {
        let mut cache =
            cache.borrow_mut();

        ensure_cached(
            &mut cache,
            mesh_a,
        );

        ensure_cached(
            &mut cache,
            mesh_b,
        );

        let a = Rc::clone(
            &cache
                .get(&mesh_a.id())
                .unwrap()
                .bvh
        );

        let b = Rc::clone(
            &cache
                .get(&mesh_b.id())
                .unwrap()
                .bvh
        );

        (a, b)
    })
}

fn ensure_cached(
    cache: &mut HashMap<
        MeshId,
        CachedBvh,
    >,
    mesh: &Mesh,
) {
    let needs_rebuild =
        match cache.get(&mesh.id()) {
            Some(entry) => {
                entry.vertex_generation
                    != mesh.vertex_generation()
                    ||
                entry.index_generation
                    != mesh.index_generation()
            }

            None => true,
        };

    if needs_rebuild {
        cache.insert(
            mesh.id(),
            CachedBvh {
                vertex_generation:
                    mesh.vertex_generation(),

                index_generation:
                    mesh.index_generation(),

                bvh: Rc::new(
                    Bvh::build(mesh)
                ),
            },
        );
    }
}