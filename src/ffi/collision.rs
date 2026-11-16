use crate::{
    collision::collides,
    math::Transform,
};

use super::{
    error::{
        ffi_boundary,
        set_last_error,
    },
    resources::with_two_meshes,
    OxideMeshHandle,
    OxideStatus,
};

#[unsafe(no_mangle)]
pub extern "C" fn oxide_collision_collides(
    mesh_a: OxideMeshHandle,
    transform_a: Transform,

    mesh_b: OxideMeshHandle,
    transform_b: Transform,

    result: *mut u8,
) -> OxideStatus {
    ffi_boundary(
        || {
            if result.is_null() {
                set_last_error(
                    "result pointer cannot be null"
                );

                return OxideStatus::InvalidArgument;
            }

            let Some(collision) =
                with_two_meshes(
                    mesh_a,
                    mesh_b,

                    |mesh_a, mesh_b| {
                        collides(
                            mesh_a,
                            &transform_a,

                            mesh_b,
                            &transform_b,
                        )
                    },
                )
            else {
                set_last_error(
                    "mesh handle is invalid"
                );

                return OxideStatus::InvalidArgument;
            };

            unsafe {
                *result =
                    if collision {
                        1
                    } else {
                        0
                    };
            }

            OxideStatus::Ok
        }
    )
}