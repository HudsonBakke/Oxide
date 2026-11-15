use std::{
    ffi::CStr,
    sync::{
        Mutex,
        MutexGuard,
        OnceLock,
    },
};

use crate::content::ContentManager;

use super::{
    error::{
        ffi_boundary,
        set_last_error,
    },
    registry::Registry,
    OxideContentManagerHandle,
    OxideStatus,
};

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