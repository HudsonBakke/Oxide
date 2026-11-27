use std::{
    ffi::CStr,
    sync::{Mutex, MutexGuard, OnceLock},
};

use crate::{
    content::ContentManager,
    graphics::{Material, Mesh, Texture},
};

use super::{
    OxideContentManagerHandle, OxideMaterialHandle, OxideMeshHandle, OxideStatus,
    OxideTextureHandle,
    error::{ffi_boundary, set_last_error},
    registry::Registry,
    resources::{insert_material, insert_mesh, insert_texture},
};

fn read_path(path: *const std::ffi::c_char) -> Result<String, OxideStatus> {
    if path.is_null() {
        set_last_error("content path cannot be null");

        return Err(OxideStatus::InvalidArgument);
    }

    let path = unsafe { CStr::from_ptr(path) };

    match path.to_str() {
        Ok(path) => Ok(path.to_owned()),

        Err(_) => {
            set_last_error("content path must be valid UTF-8");

            Err(OxideStatus::InvalidArgument)
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_content_load_mesh(
    manager: OxideContentManagerHandle,
    path: *const std::ffi::c_char,
    result: *mut OxideMeshHandle,
) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() {
            set_last_error("result pointer cannot be null");

            return OxideStatus::InvalidArgument;
        }

        let path = match read_path(path) {
            Ok(path) => path,
            Err(status) => return status,
        };

        let Some(load_result) = with_manager_mut(manager, |manager| manager.load::<Mesh>(&path))
        else {
            return invalid_handle();
        };

        match load_result {
            Ok(mesh) => {
                unsafe {
                    *result = insert_mesh(mesh);
                }

                OxideStatus::Ok
            }

            Err(error) => {
                set_last_error(error.to_string());

                OxideStatus::ContentError
            }
        }
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_content_load_material(
    manager: OxideContentManagerHandle,
    path: *const std::ffi::c_char,
    result: *mut OxideMaterialHandle,
) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() {
            set_last_error("result pointer cannot be null");

            return OxideStatus::InvalidArgument;
        }

        let path = match read_path(path) {
            Ok(path) => path,
            Err(status) => return status,
        };

        let Some(load_result) =
            with_manager_mut(manager, |manager| manager.load::<Material>(&path))
        else {
            return invalid_handle();
        };

        match load_result {
            Ok(material) => {
                unsafe {
                    *result = insert_material(material);
                }

                OxideStatus::Ok
            }

            Err(error) => {
                set_last_error(error.to_string());

                OxideStatus::ContentError
            }
        }
    })
}

static CONTENT_MANAGERS: OnceLock<Mutex<Registry<ContentManager>>> = OnceLock::new();

fn managers() -> &'static Mutex<Registry<ContentManager>> {
    CONTENT_MANAGERS.get_or_init(|| Mutex::new(Registry::new()))
}

fn lock_managers() -> MutexGuard<'static, Registry<ContentManager>> {
    managers()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn invalid_handle() -> OxideStatus {
    set_last_error("content manager handle is invalid");

    OxideStatus::InvalidArgument
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_content_manager_create(
    result: *mut OxideContentManagerHandle,
) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() {
            set_last_error("result pointer cannot be null");

            return OxideStatus::InvalidArgument;
        }

        let handle = lock_managers().insert(ContentManager::new());

        unsafe {
            *result = OxideContentManagerHandle(handle);
        }

        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_content_manager_destroy(manager: OxideContentManagerHandle) -> OxideStatus {
    ffi_boundary(|| {
        if manager.0 == 0 {
            return invalid_handle();
        }

        let removed = lock_managers().remove(manager.0);

        if removed.is_none() {
            return invalid_handle();
        }

        OxideStatus::Ok
    })
}

pub(crate) fn with_manager_mut<R>(
    handle: OxideContentManagerHandle,

    operation: impl FnOnce(&mut ContentManager) -> R,
) -> Option<R> {
    let mut registry = lock_managers();

    registry.get_mut(handle.0).map(operation)
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_content_load_texture(
    manager: OxideContentManagerHandle,
    path: *const std::ffi::c_char,
    result: *mut OxideTextureHandle,
) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() {
            set_last_error("result pointer cannot be null");

            return OxideStatus::InvalidArgument;
        }

        let path = match read_path(path) {
            Ok(path) => path,
            Err(status) => return status,
        };

        let Some(load_result) = with_manager_mut(manager, |manager| manager.load::<Texture>(&path))
        else {
            return invalid_handle();
        };

        match load_result {
            Ok(texture) => {
                unsafe {
                    *result = insert_texture(texture);
                }

                OxideStatus::Ok
            }

            Err(error) => {
                set_last_error(error.to_string());

                OxideStatus::ContentError
            }
        }
    })
}

fn read_subobject(subobject: *const std::ffi::c_char) -> Result<String, OxideStatus> {
    if subobject.is_null() {
        set_last_error("subobject cannot be null");

        return Err(OxideStatus::InvalidArgument);
    }

    let subobject = unsafe { CStr::from_ptr(subobject) };

    match subobject.to_str() {
        Ok(subobject) if !subobject.is_empty() => Ok(subobject.to_owned()),

        Ok(_) => {
            set_last_error("subobject cannot be empty");

            Err(OxideStatus::InvalidArgument)
        }

        Err(_) => {
            set_last_error("subobject must be valid UTF-8");

            Err(OxideStatus::InvalidArgument)
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_content_load_mesh_sub(
    manager: OxideContentManagerHandle,
    path: *const std::ffi::c_char,
    subobject: *const std::ffi::c_char,
    result: *mut OxideMeshHandle,
) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() {
            set_last_error("result pointer cannot be null");

            return OxideStatus::InvalidArgument;
        }

        let path = match read_path(path) {
            Ok(value) => value,
            Err(status) => return status,
        };

        let subobject = match read_subobject(subobject) {
            Ok(value) => value,
            Err(status) => return status,
        };

        let Some(load_result) = with_manager_mut(manager, |manager| {
            manager.load_sub::<Mesh>(&path, &subobject)
        }) else {
            return invalid_handle();
        };

        match load_result {
            Ok(mesh) => {
                unsafe {
                    *result = insert_mesh(mesh);
                }

                OxideStatus::Ok
            }

            Err(error) => {
                set_last_error(error.to_string());

                OxideStatus::ContentError
            }
        }
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_content_load_material_sub(
    manager: OxideContentManagerHandle,
    path: *const std::ffi::c_char,
    subobject: *const std::ffi::c_char,
    result: *mut OxideMaterialHandle,
) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() {
            set_last_error("result pointer cannot be null");

            return OxideStatus::InvalidArgument;
        }

        let path = match read_path(path) {
            Ok(value) => value,
            Err(status) => return status,
        };

        let subobject = match read_subobject(subobject) {
            Ok(value) => value,
            Err(status) => return status,
        };

        let Some(load_result) = with_manager_mut(manager, |manager| {
            manager.load_sub::<Material>(&path, &subobject)
        }) else {
            return invalid_handle();
        };

        match load_result {
            Ok(material) => {
                unsafe {
                    *result = insert_material(material);
                }

                OxideStatus::Ok
            }

            Err(error) => {
                set_last_error(error.to_string());

                OxideStatus::ContentError
            }
        }
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_content_load_texture_sub(
    manager: OxideContentManagerHandle,
    path: *const std::ffi::c_char,
    subobject: *const std::ffi::c_char,
    result: *mut OxideTextureHandle,
) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() {
            set_last_error("result pointer cannot be null");

            return OxideStatus::InvalidArgument;
        }

        let path = match read_path(path) {
            Ok(value) => value,
            Err(status) => return status,
        };

        let subobject = match read_subobject(subobject) {
            Ok(value) => value,
            Err(status) => return status,
        };

        let Some(load_result) = with_manager_mut(manager, |manager| {
            manager.load_sub::<Texture>(&path, &subobject)
        }) else {
            return invalid_handle();
        };

        match load_result {
            Ok(texture) => {
                unsafe {
                    *result = insert_texture(texture);
                }

                OxideStatus::Ok
            }

            Err(error) => {
                set_last_error(error.to_string());

                OxideStatus::ContentError
            }
        }
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_content_manager_create_with_cache_directory(
    path: *const std::ffi::c_char,
    result: *mut OxideContentManagerHandle,
) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() {
            set_last_error("result pointer cannot be null");

            return OxideStatus::InvalidArgument;
        }

        let path = match read_path(path) {
            Ok(path) => path,
            Err(status) => return status,
        };

        let handle = lock_managers().insert(ContentManager::with_cache_directory(path));

        unsafe {
            *result = OxideContentManagerHandle(handle);
        }

        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_content_manager_cache_directory_length(
    manager: OxideContentManagerHandle,
    result: *mut usize,
) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() {
            set_last_error("result pointer cannot be null");

            return OxideStatus::InvalidArgument;
        }

        let Some(length) = with_manager_mut(manager, |manager| {
            manager.cache_directory().to_string_lossy().as_bytes().len()
        }) else {
            return invalid_handle();
        };

        unsafe {
            *result = length;
        }

        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_content_manager_copy_cache_directory(
    manager: OxideContentManagerHandle,
    destination: *mut u8,
    capacity: usize,
) -> OxideStatus {
    ffi_boundary(|| {
        let Some(result) = with_manager_mut(manager, |manager| {
            let path = manager.cache_directory().to_string_lossy();

            let bytes = path.as_bytes();

            // +1 for the null terminator.
            if capacity < bytes.len() + 1 {
                return false;
            }

            if destination.is_null() {
                return false;
            }

            unsafe {
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), destination, bytes.len());

                *destination.add(bytes.len()) = 0;
            }

            true
        }) else {
            return invalid_handle();
        };

        if !result {
            set_last_error("cache directory destination buffer is null or too small");

            return OxideStatus::InvalidArgument;
        }

        OxideStatus::Ok
    })
}


#[unsafe(no_mangle)]
pub extern "C" fn oxide_content_load_audio(
    manager: OxideContentManagerHandle,
    path: *const std::ffi::c_char,
    result: *mut u64,
) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() {
            set_last_error("result pointer cannot be null");

            return OxideStatus::InvalidArgument;
        }

        let path = match read_path(path) {
            Ok(path) => path,
            Err(status) => return status,
        };

        let Some(load_result) = with_manager_mut(manager, |manager| manager.load::<crate::audio::Audio>(&path))
        else {
            return invalid_handle();
        };

        match load_result {
            Ok(texture) => {
                unsafe {
                    *result = super::audio::insert_audio(texture);
                }

                OxideStatus::Ok
            }

            Err(error) => {
                set_last_error(error.to_string());

                OxideStatus::ContentError
            }
        }
    })
}


#[unsafe(no_mangle)]
pub extern "C" fn oxide_content_load_audio_sub(
    manager: OxideContentManagerHandle,
    path: *const std::ffi::c_char,
    subobject: *const std::ffi::c_char,
    result: *mut u64,
) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() {
            set_last_error("result pointer cannot be null");

            return OxideStatus::InvalidArgument;
        }

        let path = match read_path(path) {
            Ok(value) => value,
            Err(status) => return status,
        };

        let subobject = match read_subobject(subobject) {
            Ok(value) => value,
            Err(status) => return status,
        };

        let Some(load_result) = with_manager_mut(manager, |manager| {
            manager.load_sub::<crate::audio::Audio>(&path, &subobject)
        }) else {
            return invalid_handle();
        };

        match load_result {
            Ok(texture) => {
                unsafe {
                    *result = super::audio::insert_audio(texture);
                }

                OxideStatus::Ok
            }

            Err(error) => {
                set_last_error(error.to_string());

                OxideStatus::ContentError
            }
        }
    })
}
