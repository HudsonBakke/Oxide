from __future__ import annotations

import ctypes
import os
from pathlib import Path

from .errors import OxideStatus, error_type_for
from .math import (
    Angle,
    Quaternion,
    Rect,
    Transform,
    Vec2,
    Vec3,
    Vec4,
)


class NativeGameTime(ctypes.Structure):
    _fields_ = [
        ("delta_seconds", ctypes.c_double),
        ("total_seconds", ctypes.c_double),
    ]


class NativeGraphicsHandle(ctypes.Structure):
    _fields_ = [
        ("ptr", ctypes.c_void_p),
    ]


class NativeCameraHandle(
    ctypes.Structure
):
    _fields_ = [
        ("value", ctypes.c_uint64),
    ]


class NativeMeshHandle(ctypes.Structure):
    _fields_ = [("value", ctypes.c_uint64)]


class NativeMaterialHandle(ctypes.Structure):
    _fields_ = [("value", ctypes.c_uint64)]


class NativeTextureHandle(ctypes.Structure):
    _fields_ = [("value", ctypes.c_uint64)]


class NativeContentManagerHandle(ctypes.Structure):
    _fields_ = [("value", ctypes.c_uint64)]


class NativeAppConfig(ctypes.Structure):
    _fields_ = [
        ("title", ctypes.c_char_p),
        ("width", ctypes.c_uint32),
        ("height", ctypes.c_uint32),
    ]


InitializeCallback = ctypes.CFUNCTYPE(
    None,
    ctypes.c_void_p,
)

UpdateCallback = ctypes.CFUNCTYPE(
    None,
    ctypes.c_void_p,
    NativeGameTime,
)

DrawCallback = ctypes.CFUNCTYPE(
    None,
    ctypes.c_void_p,
    NativeGraphicsHandle,
)


class NativeGameCallbacks(ctypes.Structure):
    _fields_ = [
        ("user_data", ctypes.c_void_p),
        ("initialize", InitializeCallback),
        ("update", UpdateCallback),
        ("draw", DrawCallback),
    ]


def _find_library() -> Path:
    override = os.environ.get(
        "OXIDE_LIBRARY"
    )

    if override:
        path = Path(override)

        if path.is_file():
            return path

        raise RuntimeError(
            f"OXIDE_LIBRARY does not exist: {path}"
        )

    # Development-tree fallback.
    #
    # _native.py:
    # bindings/python/oxide/_native.py
    #
    # parents[3]:
    # repository root
    root = Path(__file__).resolve().parents[3]

    candidates = [
        root / "target" / "debug" / "oxide.dll",
        root / "target" / "release" / "oxide.dll",
    ]

    for candidate in candidates:
        if candidate.is_file():
            return candidate

    raise RuntimeError(
        "Could not locate the Oxide native library. "
        "Run `cargo build` from the Oxide repository root "
        "or set OXIDE_LIBRARY."
    )


_library_path = _find_library()

lib = ctypes.CDLL(
    str(_library_path)
)


lib.oxide_last_error_message.argtypes = []
lib.oxide_last_error_message.restype = (
    ctypes.c_char_p
)


lib.oxide_run.argtypes = [
    NativeAppConfig,
    NativeGameCallbacks,
]

lib.oxide_run.restype = ctypes.c_int


lib.oxide_camera_create.argtypes = [
    ctypes.POINTER(
        NativeCameraHandle
    ),
]
lib.oxide_camera_create.restype = (
    ctypes.c_int
)

lib.oxide_camera_destroy.argtypes = [
    NativeCameraHandle,
]
lib.oxide_camera_destroy.restype = (
    ctypes.c_int
)

lib.oxide_camera_get_position.argtypes = [
    NativeCameraHandle,
    ctypes.POINTER(Vec3),
]
lib.oxide_camera_get_position.restype = (
    ctypes.c_int
)

lib.oxide_camera_set_position.argtypes = [
    NativeCameraHandle,
    Vec3,
]
lib.oxide_camera_set_position.restype = (
    ctypes.c_int
)

lib.oxide_camera_translate_by.argtypes = [
    NativeCameraHandle,
    Vec3,
]
lib.oxide_camera_translate_by.restype = (
    ctypes.c_int
)

lib.oxide_camera_get_rotation.argtypes = [
    NativeCameraHandle,
    ctypes.POINTER(Quaternion),
]
lib.oxide_camera_get_rotation.restype = (
    ctypes.c_int
)

lib.oxide_camera_set_rotation.argtypes = [
    NativeCameraHandle,
    Quaternion,
]
lib.oxide_camera_set_rotation.restype = (
    ctypes.c_int
)

lib.oxide_camera_set_euler.argtypes = [
    NativeCameraHandle,
    Angle,
    Angle,
    Angle,
]
lib.oxide_camera_set_euler.restype = (
    ctypes.c_int
)

lib.oxide_camera_rotate_pitch_by.argtypes = [
    NativeCameraHandle,
    Angle,
]
lib.oxide_camera_rotate_pitch_by.restype = (
    ctypes.c_int
)

lib.oxide_camera_rotate_yaw_by.argtypes = [
    NativeCameraHandle,
    Angle,
]
lib.oxide_camera_rotate_yaw_by.restype = (
    ctypes.c_int
)

lib.oxide_camera_rotate_roll_by.argtypes = [
    NativeCameraHandle,
    Angle,
]
lib.oxide_camera_rotate_roll_by.restype = (
    ctypes.c_int
)

lib.oxide_camera_get_field_of_view.argtypes = [
    NativeCameraHandle,
    ctypes.POINTER(Angle),
]
lib.oxide_camera_get_field_of_view.restype = (
    ctypes.c_int
)

lib.oxide_camera_set_field_of_view.argtypes = [
    NativeCameraHandle,
    Angle,
]
lib.oxide_camera_set_field_of_view.restype = (
    ctypes.c_int
)

lib.oxide_camera_get_near_plane.argtypes = [
    NativeCameraHandle,
    ctypes.POINTER(
        ctypes.c_float
    ),
]
lib.oxide_camera_get_near_plane.restype = (
    ctypes.c_int
)

lib.oxide_camera_get_far_plane.argtypes = [
    NativeCameraHandle,
    ctypes.POINTER(
        ctypes.c_float
    ),
]
lib.oxide_camera_get_far_plane.restype = (
    ctypes.c_int
)

lib.oxide_camera_set_clip_planes.argtypes = [
    NativeCameraHandle,
    ctypes.c_float,
    ctypes.c_float,
]
lib.oxide_camera_set_clip_planes.restype = (
    ctypes.c_int
)

lib.oxide_graphics_full_rect.argtypes = [
    NativeGraphicsHandle,
    ctypes.POINTER(Rect),
]
lib.oxide_graphics_full_rect.restype = (
    ctypes.c_int
)

lib.oxide_graphics_bind_camera.argtypes = [
    NativeGraphicsHandle,
    NativeCameraHandle,
    Rect,
]
lib.oxide_graphics_bind_camera.restype = (
    ctypes.c_int
)

lib.oxide_graphics_clear_lights.argtypes = [
    NativeGraphicsHandle,
]
lib.oxide_graphics_clear_lights.restype = (
    ctypes.c_int
)


# Resource/content signatures. Vertex and Color are assigned in their
# defining public modules to avoid circular imports.
lib.oxide_mesh_destroy.argtypes = [NativeMeshHandle]
lib.oxide_mesh_destroy.restype = ctypes.c_int
lib.oxide_mesh_vertex_count.argtypes = [NativeMeshHandle, ctypes.POINTER(ctypes.c_size_t)]
lib.oxide_mesh_vertex_count.restype = ctypes.c_int
lib.oxide_mesh_index_count.argtypes = [NativeMeshHandle, ctypes.POINTER(ctypes.c_size_t)]
lib.oxide_mesh_index_count.restype = ctypes.c_int
lib.oxide_mesh_get_pivot.argtypes = [NativeMeshHandle, ctypes.POINTER(Vec3)]
lib.oxide_mesh_get_pivot.restype = ctypes.c_int
lib.oxide_mesh_set_pivot.argtypes = [NativeMeshHandle, Vec3]
lib.oxide_mesh_set_pivot.restype = ctypes.c_int
lib.oxide_mesh_recalculate_pivot.argtypes = [NativeMeshHandle]
lib.oxide_mesh_recalculate_pivot.restype = ctypes.c_int
lib.oxide_mesh_get_direction.argtypes = [NativeMeshHandle, ctypes.POINTER(Vec3)]
lib.oxide_mesh_get_direction.restype = ctypes.c_int
lib.oxide_mesh_set_direction.argtypes = [NativeMeshHandle, Vec3]
lib.oxide_mesh_set_direction.restype = ctypes.c_int
lib.oxide_mesh_get_reference.argtypes = [NativeMeshHandle, ctypes.POINTER(Vec3)]
lib.oxide_mesh_get_reference.restype = ctypes.c_int
lib.oxide_mesh_set_reference.argtypes = [NativeMeshHandle, Vec3]
lib.oxide_mesh_set_reference.restype = ctypes.c_int
lib.oxide_mesh_get_side.argtypes = [NativeMeshHandle, ctypes.POINTER(Vec3)]
lib.oxide_mesh_get_side.restype = ctypes.c_int
lib.oxide_mesh_set_orientation.argtypes = [NativeMeshHandle, Vec3, Vec3]
lib.oxide_mesh_set_orientation.restype = ctypes.c_int

lib.oxide_texture_destroy.argtypes = [NativeTextureHandle]
lib.oxide_texture_destroy.restype = ctypes.c_int
lib.oxide_texture_width.argtypes = [NativeTextureHandle, ctypes.POINTER(ctypes.c_uint32)]
lib.oxide_texture_width.restype = ctypes.c_int
lib.oxide_texture_height.argtypes = [NativeTextureHandle, ctypes.POINTER(ctypes.c_uint32)]
lib.oxide_texture_height.restype = ctypes.c_int

lib.oxide_material_destroy.argtypes = [NativeMaterialHandle]
lib.oxide_material_destroy.restype = ctypes.c_int
lib.oxide_material_create.argtypes = [NativeTextureHandle, ctypes.POINTER(NativeMaterialHandle)]
lib.oxide_material_create.restype = ctypes.c_int
lib.oxide_material_base_texture.argtypes = [NativeMaterialHandle, ctypes.POINTER(NativeTextureHandle)]
lib.oxide_material_base_texture.restype = ctypes.c_int

lib.oxide_content_manager_create.argtypes = [ctypes.POINTER(NativeContentManagerHandle)]
lib.oxide_content_manager_create.restype = ctypes.c_int
lib.oxide_content_manager_create_with_cache_directory.argtypes = [ctypes.c_char_p, ctypes.POINTER(NativeContentManagerHandle)]
lib.oxide_content_manager_create_with_cache_directory.restype = ctypes.c_int
lib.oxide_content_manager_destroy.argtypes = [NativeContentManagerHandle]
lib.oxide_content_manager_destroy.restype = ctypes.c_int
lib.oxide_content_manager_cache_directory_length.argtypes = [NativeContentManagerHandle, ctypes.POINTER(ctypes.c_size_t)]
lib.oxide_content_manager_cache_directory_length.restype = ctypes.c_int
lib.oxide_content_manager_copy_cache_directory.argtypes = [NativeContentManagerHandle, ctypes.c_void_p, ctypes.c_size_t]
lib.oxide_content_manager_copy_cache_directory.restype = ctypes.c_int

for _name, _handle in (
    ("mesh", NativeMeshHandle),
    ("material", NativeMaterialHandle),
    ("texture", NativeTextureHandle),
):
    _default = getattr(lib, f"oxide_content_load_{_name}")
    _default.argtypes = [
        NativeContentManagerHandle,
        ctypes.c_char_p,
        ctypes.POINTER(_handle),
    ]
    _default.restype = ctypes.c_int

    _sub = getattr(lib, f"oxide_content_load_{_name}_sub")
    _sub.argtypes = [
        NativeContentManagerHandle,
        ctypes.c_char_p,
        ctypes.c_char_p,
        ctypes.POINTER(_handle),
    ]
    _sub.restype = ctypes.c_int


lib.oxide_transform_from_attachment.argtypes = [
    NativeMeshHandle,
    Transform,
    Vec3,
    NativeMeshHandle,
    Vec3,
    Vec3,
    Angle,
    Vec3,
    ctypes.POINTER(Transform),
]
lib.oxide_transform_from_attachment.restype = ctypes.c_int

lib.oxide_collision_collides.argtypes = [
    NativeMeshHandle,
    Transform,
    NativeMeshHandle,
    Transform,
    ctypes.POINTER(ctypes.c_uint8),
]
lib.oxide_collision_collides.restype = ctypes.c_int


def last_error_message() -> str:
    message = (
        lib.oxide_last_error_message()
    )

    if not message:
        return "Unknown Oxide error."

    return message.decode(
        "utf-8",
        errors="replace",
    )


def check(
    status: int,
) -> None:
    if status == OxideStatus.OK:
        return

    try:
        typed_status = OxideStatus(status)
    except ValueError:
        typed_status = OxideStatus.INTERNAL_ERROR

    error_type = error_type_for(
        typed_status
    )

    raise error_type(
        typed_status,
        last_error_message(),
    )