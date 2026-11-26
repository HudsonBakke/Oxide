use crate::{
    graphics::from_attachment,
    math::{Angle, Transform, Vec3},
};

use super::{
    OxideMeshHandle, OxideStatus,
    error::{ffi_boundary, set_last_error},
    resources::with_two_meshes,
};

#[unsafe(no_mangle)]
pub extern "C" fn oxide_transform_from_attachment(
    parent: OxideMeshHandle,
    parent_transform: Transform,
    parent_attach_point: Vec3,

    child: OxideMeshHandle,
    child_attach_point: Vec3,

    direction: Vec3,
    rotation: Angle,
    scale: Vec3,

    result: *mut Transform,
) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() {
            set_last_error("result pointer cannot be null");

            return OxideStatus::InvalidArgument;
        }

        let Some(attachment_result) = with_two_meshes(parent, child, |parent, child| {
            from_attachment(
                parent,
                &parent_transform,
                parent_attach_point,
                child,
                child_attach_point,
                direction,
                rotation,
                scale,
            )
        }) else {
            set_last_error("parent or child mesh handle is invalid");

            return OxideStatus::InvalidArgument;
        };

        match attachment_result {
            Ok(transform) => {
                unsafe {
                    *result = transform;
                }

                OxideStatus::Ok
            }

            Err(error) => {
                let status = match error {
                    crate::OxideError::InvalidArgument { .. } => OxideStatus::InvalidArgument,

                    crate::OxideError::InvalidOperation { .. } => OxideStatus::InvalidOperation,
                };

                set_last_error(error.to_string());

                status
            }
        }
    })
}
