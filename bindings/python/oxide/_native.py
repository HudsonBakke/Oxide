from __future__ import annotations

import ctypes
import os
from enum import IntEnum
from pathlib import Path

from .math import (
    Angle,
    Quaternion,
    Rect,
    Transform,
    Vec2,
    Vec3,
    Vec4,
)


class OxideStatus(IntEnum):
    OK = 0
    INVALID_ARGUMENT = 1
    INVALID_OPERATION = 2
    CONTENT_ERROR = 3
    GRAPHICS_ERROR = 4
    INTERNAL_ERROR = 5


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

    raise RuntimeError(
        last_error_message()
    )