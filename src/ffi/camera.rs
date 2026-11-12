use std::{
    collections::HashMap,
    sync::{
        Mutex,
        OnceLock,
    },
};

use crate::{
    graphics::Camera,
    math::{
        Angle,
        Quaternion,
        Vec3,
    },
};

use super::{
    error::{
        ffi_boundary,
        set_last_error,
    },
    OxideCameraHandle,
    OxideGraphicsHandle,
    OxideStatus,
};

struct CameraRegistry {
    next_handle: u64,
    cameras:
        HashMap<u64, Camera>,
}

impl CameraRegistry {
    fn new() -> Self {
        Self {
            next_handle: 1,
            cameras:
                HashMap::new(),
        }
    }

    fn insert(
        &mut self,
        camera: Camera,
    ) -> OxideCameraHandle {
        let handle =
            self.next_handle;

        self.next_handle =
            self.next_handle
                .wrapping_add(1);

        // Zero is reserved as the null/invalid handle.
        if self.next_handle == 0 {
            self.next_handle = 1;
        }

        self.cameras.insert(
            handle,
            camera,
        );

        OxideCameraHandle(
            handle
        )
    }
}

static CAMERAS:
    OnceLock<
        Mutex<CameraRegistry>
    > =
    OnceLock::new();

fn cameras()
    -> &'static Mutex<CameraRegistry>
{
    CAMERAS.get_or_init(
        || {
            Mutex::new(
                CameraRegistry::new()
            )
        }
    )
}

fn lock_cameras()
    -> std::sync::MutexGuard<
        'static,
        CameraRegistry,
    >
{
    cameras()
        .lock()
        .unwrap_or_else(
            |poisoned| {
                poisoned.into_inner()
            }
        )
}

fn invalid_handle() -> OxideStatus {
    set_last_error(
        "camera handle is invalid"
    );

    OxideStatus::InvalidArgument
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_camera_create(
    result:
        *mut OxideCameraHandle,
) -> OxideStatus {
    ffi_boundary(
        || {
            if result.is_null() {
                set_last_error(
                    "result pointer cannot be null"
                );

                return OxideStatus::InvalidArgument;
            }

            let handle =
                lock_cameras()
                    .insert(
                        Camera::new()
                    );

            unsafe {
                *result = handle;
            }

            OxideStatus::Ok
        }
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_camera_destroy(
    camera: OxideCameraHandle,
) -> OxideStatus {
    ffi_boundary(
        || {
            if camera.0 == 0 {
                return invalid_handle();
            }

            let removed =
                lock_cameras()
                    .cameras
                    .remove(
                        &camera.0
                    );

            if removed.is_none() {
                return invalid_handle();
            }

            OxideStatus::Ok
        }
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_camera_get_position(
    camera: OxideCameraHandle,
    result: *mut Vec3,
) -> OxideStatus {
    ffi_boundary(
        || {
            if result.is_null() {
                set_last_error(
                    "result pointer cannot be null"
                );

                return OxideStatus::InvalidArgument;
            }

            let registry =
                lock_cameras();

            let Some(camera) =
                registry.cameras.get(
                    &camera.0
                )
            else {
                return invalid_handle();
            };

            unsafe {
                *result =
                    camera.position();
            }

            OxideStatus::Ok
        }
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_camera_set_position(
    camera: OxideCameraHandle,
    position: Vec3,
) -> OxideStatus {
    ffi_boundary(
        || {
            let mut registry =
                lock_cameras();

            let Some(camera) =
                registry.cameras.get_mut(
                    &camera.0
                )
            else {
                return invalid_handle();
            };

            camera.set_position(
                position
            );

            OxideStatus::Ok
        }
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_camera_translate_by(
    camera: OxideCameraHandle,
    translation: Vec3,
) -> OxideStatus {
    ffi_boundary(
        || {
            let mut registry =
                lock_cameras();

            let Some(camera) =
                registry.cameras.get_mut(
                    &camera.0
                )
            else {
                return invalid_handle();
            };

            camera.translate_by(
                translation
            );

            OxideStatus::Ok
        }
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_camera_get_rotation(
    camera: OxideCameraHandle,
    result: *mut Quaternion,
) -> OxideStatus {
    ffi_boundary(
        || {
            if result.is_null() {
                set_last_error(
                    "result pointer cannot be null"
                );

                return OxideStatus::InvalidArgument;
            }

            let registry =
                lock_cameras();

            let Some(camera) =
                registry.cameras.get(
                    &camera.0
                )
            else {
                return invalid_handle();
            };

            unsafe {
                *result =
                    camera.rotation();
            }

            OxideStatus::Ok
        }
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_camera_set_rotation(
    camera: OxideCameraHandle,
    rotation: Quaternion,
) -> OxideStatus {
    ffi_boundary(
        || {
            let mut registry =
                lock_cameras();

            let Some(camera) =
                registry.cameras.get_mut(
                    &camera.0
                )
            else {
                return invalid_handle();
            };

            camera.set_rotation(
                rotation
            );

            OxideStatus::Ok
        }
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_camera_set_euler(
    camera: OxideCameraHandle,
    pitch: Angle,
    yaw: Angle,
    roll: Angle,
) -> OxideStatus {
    ffi_boundary(
        || {
            let mut registry =
                lock_cameras();

            let Some(camera) =
                registry.cameras.get_mut(
                    &camera.0
                )
            else {
                return invalid_handle();
            };

            camera.set_euler(
                pitch,
                yaw,
                roll,
            );

            OxideStatus::Ok
        }
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_camera_get_field_of_view(
    camera: OxideCameraHandle,
    result: *mut Angle,
) -> OxideStatus {
    ffi_boundary(
        || {
            if result.is_null() {
                set_last_error(
                    "result pointer cannot be null"
                );

                return OxideStatus::InvalidArgument;
            }

            let registry =
                lock_cameras();

            let Some(camera) =
                registry.cameras.get(
                    &camera.0
                )
            else {
                return invalid_handle();
            };

            unsafe {
                *result =
                    camera.field_of_view();
            }

            OxideStatus::Ok
        }
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_camera_set_field_of_view(
    camera: OxideCameraHandle,
    field_of_view: Angle,
) -> OxideStatus {
    ffi_boundary(
        || {
            let mut registry =
                lock_cameras();

            let Some(camera) =
                registry.cameras.get_mut(
                    &camera.0
                )
            else {
                return invalid_handle();
            };

            match camera.set_field_of_view(
                field_of_view
            ) {
                Ok(()) =>
                    OxideStatus::Ok,

                Err(error) => {
                    set_last_error(
                        error.to_string()
                    );

                    OxideStatus::InvalidArgument
                }
            }
        }
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_camera_set_clip_planes(
    camera: OxideCameraHandle,
    near_plane: f32,
    far_plane: f32,
) -> OxideStatus {
    ffi_boundary(
        || {
            let mut registry =
                lock_cameras();

            let Some(camera) =
                registry.cameras.get_mut(
                    &camera.0
                )
            else {
                return invalid_handle();
            };

            match camera.set_clip_planes(
                near_plane,
                far_plane,
            ) {
                Ok(()) =>
                    OxideStatus::Ok,

                Err(error) => {
                    set_last_error(
                        error.to_string()
                    );

                    OxideStatus::InvalidArgument
                }
            }
        }
    )
}

pub(crate) fn with_camera<R>(
    handle: OxideCameraHandle,
    operation:
        impl FnOnce(&Camera) -> R,
) -> Option<R> {
    let registry =
        lock_cameras();

    registry
        .cameras
        .get(&handle.0)
        .map(operation)
}