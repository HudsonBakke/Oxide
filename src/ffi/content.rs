use std::{
    ffi::CStr,
    sync::{
        Mutex,
        MutexGuard,
        OnceLock,
    },
};

use crate::{
    content::ContentManager,
    graphics::{
        Material,
        Mesh,
    },
};

use super::{
    error::{
        ffi_boundary,
        set_last_error,
    },
    resources::{
        insert_material,
        insert_mesh,
    },
    OxideMaterialHandle,
    OxideMeshHandle,
    registry::Registry,
    OxideContentManagerHandle,
    OxideStatus,
};

fn read_path(
    path: *const std::ffi::c_char,
) -> Result<String, OxideStatus> {
    if path.is_null() {
        set_last_error(
            "content path cannot be null"
        );

        return Err(
            OxideStatus::InvalidArgument
        );
    }

    let path =
        unsafe {
            CStr::from_ptr(path)
        };

    match path.to_str() {
        Ok(path) =>
            Ok(path.to_owned()),

        Err(_) => {
            set_last_error(
                "content path must be valid UTF-8"
            );

            Err(
                OxideStatus::InvalidArgument
            )
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_content_load_mesh(
    manager: OxideContentManagerHandle,
    path: *const std::ffi::c_char,
    result: *mut OxideMeshHandle,
) -> OxideStatus {
    ffi_boundary(
        || {
            if result.is_null() {
                set_last_error(
                    "result pointer cannot be null"
                );

                return OxideStatus::InvalidArgument;
            }

            let path =
                match read_path(path) {
                    Ok(path) => path,
                    Err(status) =>
                        return status,
                };

            let Some(load_result) =
                with_manager_mut(
                    manager,
                    |manager| {
                        manager.load::<Mesh>(
                            &path
                        )
                    },
                )
            else {
                return invalid_handle();
            };

            match load_result {
                Ok(mesh) => {
                    unsafe {
                        *result =
                            insert_mesh(mesh);
                    }

                    OxideStatus::Ok
                }

                Err(error) => {
                    set_last_error(
                        error.to_string()
                    );

                    OxideStatus::ContentError
                }
            }
        }
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_content_load_material(
    manager: OxideContentManagerHandle,
    path: *const std::ffi::c_char,
    result:
        *mut OxideMaterialHandle,
) -> OxideStatus {
    ffi_boundary(
        || {
            if result.is_null() {
                set_last_error(
                    "result pointer cannot be null"
                );

                return OxideStatus::InvalidArgument;
            }

            let path =
                match read_path(path) {
                    Ok(path) => path,
                    Err(status) =>
                        return status,
                };

            let Some(load_result) =
                with_manager_mut(
                    manager,
                    |manager| {
                        manager.load::<Material>(
                            &path
                        )
                    },
                )
            else {
                return invalid_handle();
            };

            match load_result {
                Ok(material) => {
                    unsafe {
                        *result =
                            insert_material(
                                material
                            );
                    }

                    OxideStatus::Ok
                }

                Err(error) => {
                    set_last_error(
                        error.to_string()
                    );

                    OxideStatus::ContentError
                }
            }
        }
    )
}

static CONTENT_MANAGERS:
    OnceLock<
        Mutex<Registry<ContentManager>>
    > =
    OnceLock::new();

fn managers()
    -> &'static Mutex<
        Registry<ContentManager>
    >
{
    CONTENT_MANAGERS.get_or_init(
        || {
            Mutex::new(
                Registry::new()
            )
        }
    )
}

fn lock_managers()
    -> MutexGuard<
        'static,
        Registry<ContentManager>,
    >
{
    managers()
        .lock()
        .unwrap_or_else(
            |poisoned| {
                poisoned.into_inner()
            }
        )
}

fn invalid_handle()
    -> OxideStatus
{
    set_last_error(
        "content manager handle is invalid"
    );

    OxideStatus::InvalidArgument
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_content_manager_create(
    result:
        *mut OxideContentManagerHandle,
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
                lock_managers()
                    .insert(
                        ContentManager::new()
                    );

            unsafe {
                *result =
                    OxideContentManagerHandle(
                        handle
                    );
            }

            OxideStatus::Ok
        }
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_content_manager_destroy(
    manager:
        OxideContentManagerHandle,
) -> OxideStatus {
    ffi_boundary(
        || {
            if manager.0 == 0 {
                return invalid_handle();
            }

            let removed =
                lock_managers()
                    .remove(
                        manager.0
                    );

            if removed.is_none() {
                return invalid_handle();
            }

            OxideStatus::Ok
        }
    )
}

pub(crate) fn with_manager_mut<R>(
    handle:
        OxideContentManagerHandle,

    operation:
        impl FnOnce(
            &mut ContentManager
        ) -> R,
) -> Option<R> {
    let mut registry =
        lock_managers();

    registry
        .get_mut(
            handle.0
        )
        .map(operation)
}