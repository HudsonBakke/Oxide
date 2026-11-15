use std::sync::{
    Mutex,
    MutexGuard,
    OnceLock,
};

use crate::graphics::{
    Material,
    Mesh,
};

use super::{
    error::{
        ffi_boundary,
        set_last_error,
    },
    registry::Registry,
    OxideMaterialHandle,
    OxideMeshHandle,
    OxideStatus,
};

static MESHES:
    OnceLock<
        Mutex<Registry<Mesh>>
    > =
    OnceLock::new();

static MATERIALS:
    OnceLock<
        Mutex<Registry<Material>>
    > =
    OnceLock::new();

fn meshes()
    -> &'static Mutex<Registry<Mesh>>
{
    MESHES.get_or_init(
        || {
            Mutex::new(
                Registry::new()
            )
        }
    )
}

fn materials()
    -> &'static Mutex<
        Registry<Material>
    >
{
    MATERIALS.get_or_init(
        || {
            Mutex::new(
                Registry::new()
            )
        }
    )
}

fn lock_meshes()
    -> MutexGuard<
        'static,
        Registry<Mesh>,
    >
{
    meshes()
        .lock()
        .unwrap_or_else(
            |poisoned| {
                poisoned.into_inner()
            }
        )
}

fn lock_materials()
    -> MutexGuard<
        'static,
        Registry<Material>,
    >
{
    materials()
        .lock()
        .unwrap_or_else(
            |poisoned| {
                poisoned.into_inner()
            }
        )
}

pub(crate) fn insert_mesh(
    mesh: Mesh,
) -> OxideMeshHandle {
    OxideMeshHandle(
        lock_meshes()
            .insert(mesh)
    )
}

pub(crate) fn insert_material(
    material: Material,
) -> OxideMaterialHandle {
    OxideMaterialHandle(
        lock_materials()
            .insert(material)
    )
}

pub(crate) fn remove_mesh(
    handle: OxideMeshHandle,
) -> bool {
    lock_meshes()
        .remove(handle.0)
        .is_some()
}

pub(crate) fn remove_material(
    handle: OxideMaterialHandle,
) -> bool {
    lock_materials()
        .remove(handle.0)
        .is_some()
}

pub(crate) fn with_mesh<R>(
    handle: OxideMeshHandle,
    operation:
        impl FnOnce(&Mesh) -> R,
) -> Option<R> {
    let registry =
        lock_meshes();

    registry
        .get(handle.0)
        .map(operation)
}

pub(crate) fn with_material<R>(
    handle: OxideMaterialHandle,
    operation:
        impl FnOnce(&Material) -> R,
) -> Option<R> {
    let registry =
        lock_materials();

    registry
        .get(handle.0)
        .map(operation)
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_mesh_destroy(
    mesh: OxideMeshHandle,
) -> OxideStatus {
    ffi_boundary(
        || {
            if mesh.0 == 0
                || !remove_mesh(mesh)
            {
                set_last_error(
                    "mesh handle is invalid"
                );

                return OxideStatus::InvalidArgument;
            }

            OxideStatus::Ok
        }
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_material_destroy(
    material: OxideMaterialHandle,
) -> OxideStatus {
    ffi_boundary(
        || {
            if material.0 == 0
                || !remove_material(
                    material
                )
            {
                set_last_error(
                    "material handle is invalid"
                );

                return OxideStatus::InvalidArgument;
            }

            OxideStatus::Ok
        }
    )
}