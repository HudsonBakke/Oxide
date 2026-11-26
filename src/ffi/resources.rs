use std::sync::{Mutex, MutexGuard, OnceLock};

use crate::{
    graphics::{Color, Material, Mesh, Texture, Vertex},
    math::Vec3,
};

use super::{
    OxideMaterialHandle, OxideMeshHandle, OxideStatus, OxideTextureHandle,
    error::{ffi_boundary, set_last_error},
    registry::Registry,
};

static TEXTURES: OnceLock<Mutex<Registry<Texture>>> = OnceLock::new();

fn textures() -> &'static Mutex<Registry<Texture>> {
    TEXTURES.get_or_init(|| Mutex::new(Registry::new()))
}

fn lock_textures() -> MutexGuard<'static, Registry<Texture>> {
    textures()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

pub(crate) fn insert_texture(texture: Texture) -> OxideTextureHandle {
    OxideTextureHandle(lock_textures().insert(texture))
}

pub(crate) fn with_texture<R>(
    handle: OxideTextureHandle,
    operation: impl FnOnce(&Texture) -> R,
) -> Option<R> {
    let registry = lock_textures();

    registry.get(handle.0).map(operation)
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_texture_solid(
    color: Color,
    result: *mut OxideTextureHandle,
) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() {
            set_last_error("result pointer cannot be null");

            return OxideStatus::InvalidArgument;
        }

        let texture = Texture::solid(color);

        unsafe {
            *result = insert_texture(texture);
        }

        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_texture_width(
    texture: OxideTextureHandle,
    result: *mut u32,
) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() {
            set_last_error("result pointer cannot be null");

            return OxideStatus::InvalidArgument;
        }

        let Some(width) = with_texture(texture, |texture| texture.width()) else {
            set_last_error("texture handle is invalid");

            return OxideStatus::InvalidArgument;
        };

        unsafe {
            *result = width;
        }

        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_texture_height(
    texture: OxideTextureHandle,
    result: *mut u32,
) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() {
            set_last_error("result pointer cannot be null");

            return OxideStatus::InvalidArgument;
        }

        let Some(height) = with_texture(texture, |texture| texture.height()) else {
            set_last_error("texture handle is invalid");

            return OxideStatus::InvalidArgument;
        };

        unsafe {
            *result = height;
        }

        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_material_create(
    texture: OxideTextureHandle,
    result: *mut OxideMaterialHandle,
) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() {
            set_last_error("result pointer cannot be null");

            return OxideStatus::InvalidArgument;
        }

        let Some(texture) = with_texture(texture, |texture| texture.clone()) else {
            set_last_error("texture handle is invalid");

            return OxideStatus::InvalidArgument;
        };

        let material = Material::new(texture);

        unsafe {
            *result = insert_material(material);
        }

        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_material_solid(
    color: Color,
    result: *mut OxideMaterialHandle,
) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() {
            set_last_error("result pointer cannot be null");

            return OxideStatus::InvalidArgument;
        }

        unsafe {
            *result = insert_material(Material::solid(color));
        }

        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_material_base_texture(
    material: OxideMaterialHandle,
    result: *mut OxideTextureHandle,
) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() {
            set_last_error("result pointer cannot be null");

            return OxideStatus::InvalidArgument;
        }

        let Some(texture) = with_material(material, |material| material.base_texture().clone())
        else {
            set_last_error("material handle is invalid");

            return OxideStatus::InvalidArgument;
        };

        unsafe {
            *result = insert_texture(texture);
        }

        OxideStatus::Ok
    })
}


macro_rules! material_color_property {
    ($get:ident, $set:ident, $getter:ident, $setter:ident) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn $get(material: OxideMaterialHandle, result: *mut Color) -> OxideStatus {
            ffi_boundary(|| {
                if result.is_null() {
                    set_last_error("result pointer cannot be null");
                    return OxideStatus::InvalidArgument;
                }
                let Some(value) = with_material(material, |m| m.$getter()) else {
                    set_last_error("material handle is invalid");
                    return OxideStatus::InvalidArgument;
                };
                unsafe { *result = value; }
                OxideStatus::Ok
            })
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn $set(material: OxideMaterialHandle, value: Color) -> OxideStatus {
            ffi_boundary(|| {
                let Some(()) = with_material_mut(material, |m| m.$setter(value)) else {
                    set_last_error("material handle is invalid");
                    return OxideStatus::InvalidArgument;
                };
                OxideStatus::Ok
            })
        }
    };
}

material_color_property!(
    oxide_material_get_base_color,
    oxide_material_set_base_color,
    base_color,
    set_base_color
);
material_color_property!(
    oxide_material_get_emissive_color,
    oxide_material_set_emissive_color,
    emissive_color,
    set_emissive_color
);

macro_rules! material_float_property {
    ($get:ident, $set:ident, $getter:ident, $setter:ident) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn $get(material: OxideMaterialHandle, result: *mut f32) -> OxideStatus {
            ffi_boundary(|| {
                if result.is_null() {
                    set_last_error("result pointer cannot be null");
                    return OxideStatus::InvalidArgument;
                }
                let Some(value) = with_material(material, |m| m.$getter()) else {
                    set_last_error("material handle is invalid");
                    return OxideStatus::InvalidArgument;
                };
                unsafe { *result = value; }
                OxideStatus::Ok
            })
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn $set(material: OxideMaterialHandle, value: f32) -> OxideStatus {
            ffi_boundary(|| {
                let Some(result) = with_material_mut(material, |m| m.$setter(value)) else {
                    set_last_error("material handle is invalid");
                    return OxideStatus::InvalidArgument;
                };
                if let Err(error) = result {
                    set_last_error(error.to_string());
                    return OxideStatus::InvalidArgument;
                }
                OxideStatus::Ok
            })
        }
    };
}

material_float_property!(oxide_material_get_metallic, oxide_material_set_metallic, metallic, set_metallic);
material_float_property!(oxide_material_get_roughness, oxide_material_set_roughness, roughness, set_roughness);
material_float_property!(
    oxide_material_get_emissive_strength,
    oxide_material_set_emissive_strength,
    emissive_strength,
    set_emissive_strength
);

#[unsafe(no_mangle)]
pub extern "C" fn oxide_texture_destroy(texture: OxideTextureHandle) -> OxideStatus {
    ffi_boundary(|| {
        if texture.0 == 0 || lock_textures().remove(texture.0).is_none() {
            set_last_error("texture handle is invalid");

            return OxideStatus::InvalidArgument;
        }

        OxideStatus::Ok
    })
}

static MESHES: OnceLock<Mutex<Registry<Mesh>>> = OnceLock::new();

static MATERIALS: OnceLock<Mutex<Registry<Material>>> = OnceLock::new();

fn meshes() -> &'static Mutex<Registry<Mesh>> {
    MESHES.get_or_init(|| Mutex::new(Registry::new()))
}

fn materials() -> &'static Mutex<Registry<Material>> {
    MATERIALS.get_or_init(|| Mutex::new(Registry::new()))
}

fn lock_meshes() -> MutexGuard<'static, Registry<Mesh>> {
    meshes()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn lock_materials() -> MutexGuard<'static, Registry<Material>> {
    materials()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

pub(crate) fn insert_mesh(mesh: Mesh) -> OxideMeshHandle {
    OxideMeshHandle(lock_meshes().insert(mesh))
}

pub(crate) fn insert_material(material: Material) -> OxideMaterialHandle {
    OxideMaterialHandle(lock_materials().insert(material))
}

pub(crate) fn remove_mesh(handle: OxideMeshHandle) -> bool {
    lock_meshes().remove(handle.0).is_some()
}

pub(crate) fn remove_material(handle: OxideMaterialHandle) -> bool {
    lock_materials().remove(handle.0).is_some()
}

pub(crate) fn with_mesh<R>(
    handle: OxideMeshHandle,
    operation: impl FnOnce(&Mesh) -> R,
) -> Option<R> {
    let registry = lock_meshes();

    registry.get(handle.0).map(operation)
}

pub(crate) fn with_material<R>(
    handle: OxideMaterialHandle,
    operation: impl FnOnce(&Material) -> R,
) -> Option<R> {
    let registry = lock_materials();
    registry.get(handle.0).map(operation)
}

fn with_material_mut<R>(
    handle: OxideMaterialHandle,
    operation: impl FnOnce(&mut Material) -> R,
) -> Option<R> {
    let mut registry = lock_materials();
    registry.get_mut(handle.0).map(operation)
}

pub(crate) fn with_mesh_mut<R>(
    handle: OxideMeshHandle,
    operation: impl FnOnce(&mut Mesh) -> R,
) -> Option<R> {
    let mut registry = lock_meshes();

    registry.get_mut(handle.0).map(operation)
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_mesh_destroy(mesh: OxideMeshHandle) -> OxideStatus {
    ffi_boundary(|| {
        if mesh.0 == 0 || !remove_mesh(mesh) {
            set_last_error("mesh handle is invalid");

            return OxideStatus::InvalidArgument;
        }

        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_material_destroy(material: OxideMaterialHandle) -> OxideStatus {
    ffi_boundary(|| {
        if material.0 == 0 || !remove_material(material) {
            set_last_error("material handle is invalid");

            return OxideStatus::InvalidArgument;
        }

        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_mesh_vertex_count(
    mesh: OxideMeshHandle,
    result: *mut usize,
) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() {
            set_last_error("result pointer cannot be null");

            return OxideStatus::InvalidArgument;
        }

        let Some(count) = with_mesh(mesh, |mesh| mesh.vertex_count()) else {
            set_last_error("mesh handle is invalid");

            return OxideStatus::InvalidArgument;
        };

        unsafe {
            *result = count;
        }

        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_mesh_index_count(mesh: OxideMeshHandle, result: *mut usize) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() {
            set_last_error("result pointer cannot be null");

            return OxideStatus::InvalidArgument;
        }

        let Some(count) = with_mesh(mesh, |mesh| mesh.index_count()) else {
            set_last_error("mesh handle is invalid");

            return OxideStatus::InvalidArgument;
        };

        unsafe {
            *result = count;
        }

        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_mesh_get_pivot(mesh: OxideMeshHandle, result: *mut Vec3) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() {
            set_last_error("result pointer cannot be null");

            return OxideStatus::InvalidArgument;
        }

        let Some(pivot) = with_mesh(mesh, |mesh| mesh.pivot()) else {
            set_last_error("mesh handle is invalid");

            return OxideStatus::InvalidArgument;
        };

        unsafe {
            *result = pivot;
        }

        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_mesh_set_pivot(mesh: OxideMeshHandle, pivot: Vec3) -> OxideStatus {
    ffi_boundary(|| {
        let Some(()) = with_mesh_mut(mesh, |mesh| {
            mesh.set_pivot(pivot);
        }) else {
            set_last_error("mesh handle is invalid");

            return OxideStatus::InvalidArgument;
        };

        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_mesh_recalculate_pivot(mesh: OxideMeshHandle) -> OxideStatus {
    ffi_boundary(|| {
        let Some(()) = with_mesh_mut(mesh, |mesh| {
            mesh.recalculate_pivot();
        }) else {
            set_last_error("mesh handle is invalid");

            return OxideStatus::InvalidArgument;
        };

        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_mesh_get_direction(
    mesh: OxideMeshHandle,
    result: *mut Vec3,
) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() {
            set_last_error("result pointer cannot be null");

            return OxideStatus::InvalidArgument;
        }

        let Some(value) = with_mesh(mesh, |mesh| mesh.direction()) else {
            set_last_error("mesh handle is invalid");

            return OxideStatus::InvalidArgument;
        };

        unsafe {
            *result = value;
        }

        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_mesh_get_reference(
    mesh: OxideMeshHandle,
    result: *mut Vec3,
) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() {
            set_last_error("result pointer cannot be null");

            return OxideStatus::InvalidArgument;
        }

        let Some(value) = with_mesh(mesh, |mesh| mesh.reference()) else {
            set_last_error("mesh handle is invalid");

            return OxideStatus::InvalidArgument;
        };

        unsafe {
            *result = value;
        }

        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_mesh_get_side(mesh: OxideMeshHandle, result: *mut Vec3) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() {
            set_last_error("result pointer cannot be null");

            return OxideStatus::InvalidArgument;
        }

        let Some(value) = with_mesh(mesh, |mesh| mesh.side()) else {
            set_last_error("mesh handle is invalid");

            return OxideStatus::InvalidArgument;
        };

        unsafe {
            *result = value;
        }

        OxideStatus::Ok
    })
}

fn mesh_result_status(result: crate::OxideResult<()>) -> OxideStatus {
    match result {
        Ok(()) => OxideStatus::Ok,

        Err(error) => {
            set_last_error(error.to_string());

            OxideStatus::InvalidArgument
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_mesh_set_direction(mesh: OxideMeshHandle, direction: Vec3) -> OxideStatus {
    ffi_boundary(|| {
        let Some(result) = with_mesh_mut(mesh, |mesh| mesh.set_direction(direction)) else {
            set_last_error("mesh handle is invalid");

            return OxideStatus::InvalidArgument;
        };

        mesh_result_status(result)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_mesh_set_reference(mesh: OxideMeshHandle, reference: Vec3) -> OxideStatus {
    ffi_boundary(|| {
        let Some(result) = with_mesh_mut(mesh, |mesh| mesh.set_reference(reference)) else {
            set_last_error("mesh handle is invalid");

            return OxideStatus::InvalidArgument;
        };

        mesh_result_status(result)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_mesh_set_orientation(
    mesh: OxideMeshHandle,
    direction: Vec3,
    reference: Vec3,
) -> OxideStatus {
    ffi_boundary(|| {
        let Some(result) = with_mesh_mut(mesh, |mesh| mesh.set_orientation(direction, reference))
        else {
            set_last_error("mesh handle is invalid");

            return OxideStatus::InvalidArgument;
        };

        mesh_result_status(result)
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_mesh_copy_vertices(
    mesh: OxideMeshHandle,
    destination: *mut Vertex,
    capacity: usize,
) -> OxideStatus {
    ffi_boundary(|| {
        let Some(result) = with_mesh(mesh, |mesh| {
            if capacity < mesh.vertex_count() {
                return false;
            }

            if !mesh.vertices().is_empty() && destination.is_null() {
                return false;
            }

            if !mesh.vertices().is_empty() {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        mesh.vertices().as_ptr(),
                        destination,
                        mesh.vertex_count(),
                    );
                }
            }

            true
        }) else {
            set_last_error("mesh handle is invalid");

            return OxideStatus::InvalidArgument;
        };

        if !result {
            set_last_error("vertex destination buffer is null or too small");

            return OxideStatus::InvalidArgument;
        }

        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_mesh_copy_indices(
    mesh: OxideMeshHandle,
    destination: *mut u32,
    capacity: usize,
) -> OxideStatus {
    ffi_boundary(|| {
        let Some(result) = with_mesh(mesh, |mesh| {
            if capacity < mesh.index_count() {
                return false;
            }

            if !mesh.indices().is_empty() && destination.is_null() {
                return false;
            }

            if !mesh.indices().is_empty() {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        mesh.indices().as_ptr(),
                        destination,
                        mesh.index_count(),
                    );
                }
            }

            true
        }) else {
            set_last_error("mesh handle is invalid");

            return OxideStatus::InvalidArgument;
        };

        if !result {
            set_last_error("index destination buffer is null or too small");

            return OxideStatus::InvalidArgument;
        }

        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_mesh_set_vertices(
    mesh: OxideMeshHandle,
    vertices: *const Vertex,
    count: usize,
) -> OxideStatus {
    ffi_boundary(|| {
        if count > 0 && vertices.is_null() {
            set_last_error("vertex source pointer cannot be null");

            return OxideStatus::InvalidArgument;
        }

        let Some(result) = with_mesh_mut(mesh, |mesh| {
            if count != mesh.vertex_count() {
                return false;
            }

            if count > 0 {
                let source = unsafe { std::slice::from_raw_parts(vertices, count) };

                mesh.vertices_mut().copy_from_slice(source);
            }

            true
        }) else {
            set_last_error("mesh handle is invalid");

            return OxideStatus::InvalidArgument;
        };

        if !result {
            set_last_error("vertex count must match the mesh's existing vertex count");

            return OxideStatus::InvalidArgument;
        }

        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_mesh_set_indices(
    mesh: OxideMeshHandle,
    indices: *const u32,
    count: usize,
) -> OxideStatus {
    ffi_boundary(|| {
        if count > 0 && indices.is_null() {
            set_last_error("index source pointer cannot be null");

            return OxideStatus::InvalidArgument;
        }

        let Some(result) = with_mesh_mut(mesh, |mesh| {
            if count != mesh.index_count() {
                return false;
            }

            if count > 0 {
                let source = unsafe { std::slice::from_raw_parts(indices, count) };

                mesh.indices_mut().copy_from_slice(source);
            }

            true
        }) else {
            set_last_error("mesh handle is invalid");

            return OxideStatus::InvalidArgument;
        };

        if !result {
            set_last_error("index count must match the mesh's existing index count");

            return OxideStatus::InvalidArgument;
        }

        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_mesh_create(
    vertices: *const Vertex,
    vertex_count: usize,

    indices: *const u32,
    index_count: usize,

    result: *mut OxideMeshHandle,
) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() {
            set_last_error("result pointer cannot be null");

            return OxideStatus::InvalidArgument;
        }

        if vertex_count > 0 && vertices.is_null() {
            set_last_error("vertex source pointer cannot be null");

            return OxideStatus::InvalidArgument;
        }

        if index_count > 0 && indices.is_null() {
            set_last_error("index source pointer cannot be null");

            return OxideStatus::InvalidArgument;
        }

        let vertices = if vertex_count == 0 {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(vertices, vertex_count) }.to_vec()
        };

        let indices = if index_count == 0 {
            Vec::new()
        } else {
            unsafe { std::slice::from_raw_parts(indices, index_count) }.to_vec()
        };

        // Reject indices that cannot refer to a vertex.
        if indices
            .iter()
            .any(|&index| index as usize >= vertices.len())
        {
            set_last_error("mesh contains an out-of-range index");

            return OxideStatus::InvalidArgument;
        }

        let mesh = Mesh::new(vertices, indices);

        unsafe {
            *result = insert_mesh(mesh);
        }

        OxideStatus::Ok
    })
}

pub(crate) fn with_two_meshes<R>(
    first: OxideMeshHandle,
    second: OxideMeshHandle,
    operation: impl FnOnce(&Mesh, &Mesh) -> R,
) -> Option<R> {
    let registry = lock_meshes();

    let first = registry.get(first.0)?;

    let second = registry.get(second.0)?;

    Some(operation(first, second))
}
