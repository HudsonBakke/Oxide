from __future__ import annotations

import ctypes
import math
import weakref

from . import _native
from .math import (
    Angle,
    Quaternion,
    Rect,
    Transform,
    Vec3,
)
from .content import Material, Mesh


class Color(ctypes.Structure):
    _fields_ = [
        ("r", ctypes.c_float),
        ("g", ctypes.c_float),
        ("b", ctypes.c_float),
        ("a", ctypes.c_float),
    ]

    def __init__(
        self,
        r: float = 0.0,
        g: float = 0.0,
        b: float = 0.0,
        a: float = 1.0,
    ) -> None:
        super().__init__(
            float(r),
            float(g),
            float(b),
            float(a),
        )

    @classmethod
    def rgb(
        cls,
        r: float,
        g: float,
        b: float,
    ) -> Color:
        return cls(
            r,
            g,
            b,
            1.0,
        )

    @classmethod
    def rgba(
        cls,
        r: float,
        g: float,
        b: float,
        a: float,
    ) -> Color:
        return cls(
            r,
            g,
            b,
            a,
        )

    def __eq__(
        self,
        other: object,
    ) -> bool:
        if not isinstance(
            other,
            Color,
        ):
            return NotImplemented

        return (
            self.r == other.r
            and self.g == other.g
            and self.b == other.b
            and self.a == other.a
        )

    def __repr__(self) -> str:
        return (
            f"Color("
            f"{self.r}, "
            f"{self.g}, "
            f"{self.b}, "
            f"{self.a}"
            f")"
        )


# Python class attributes corresponding to the Rust/C# constants.
Color.BLACK = Color.rgb(
    0.0,
    0.0,
    0.0,
)

Color.WHITE = Color.rgb(
    1.0,
    1.0,
    1.0,
)

Color.RED = Color.rgb(
    1.0,
    0.0,
    0.0,
)

Color.GREEN = Color.rgb(
    0.0,
    1.0,
    0.0,
)

Color.BLUE = Color.rgb(
    0.0,
    0.0,
    1.0,
)

Color.YELLOW = Color.rgb(
    1.0,
    1.0,
    0.0,
)



class Environment:
    def __init__(
        self,
        ambient_color: Color,
        ambient_intensity: float,
        environment_texture: Texture | None,
        environment_intensity: float,
        background_color: Color,
    ) -> None:
        if not math.isfinite(ambient_intensity) or ambient_intensity < 0.0:
            raise ValueError("ambient intensity must be finite and non-negative")
        if not math.isfinite(environment_intensity) or environment_intensity < 0.0:
            raise ValueError("environment intensity must be finite and non-negative")

        self.ambient_color = ambient_color
        self.ambient_intensity = float(ambient_intensity)
        self.environment_texture = environment_texture
        self.environment_intensity = float(environment_intensity)
        self.background_color = background_color


class PointLight(ctypes.Structure):
    _fields_ = [
        ("position", Vec3),
        ("color", Color),
        ("intensity", ctypes.c_float),
    ]

    def __init__(
        self,
        position: Vec3,
        color: Color,
        intensity: float,
    ) -> None:
        if intensity < 0.0:
            raise ValueError(
                "Light intensity cannot be negative."
            )

        super().__init__(
            position,
            color,
            float(intensity),
        )

    def __setattr__(self, name, value):
        if name == "intensity" and float(value) < 0.0:
            raise ValueError("Light intensity cannot be negative.")

        super().__setattr__(name, value)


class SpotLight(ctypes.Structure):
    _fields_ = [
        ("position", Vec3),
        ("direction", Vec3),
        ("angle", Angle),
        ("color", Color),
        ("intensity", ctypes.c_float),
    ]

    def __init__(
        self,
        position: Vec3,
        direction: Vec3,
        angle: Angle,
        color: Color,
        intensity: float,
    ) -> None:
        if direction.length <= 1.0e-6:
            raise ValueError(
                "Spot light direction cannot be zero."
            )

        if (
            angle.as_radians <= 0.0
            or angle.as_radians >= math.pi
        ):
            raise ValueError(
                "Spot light angle must be "
                "between 0 and pi radians."
            )

        if intensity < 0.0:
            raise ValueError(
                "Light intensity cannot be negative."
            )

        super().__init__(
            position,
            direction.normalized(),
            angle,
            color,
            float(intensity),
        )

    def __setattr__(self, name, value):
        if name == "direction":
            if value.length <= 1.0e-6:
                raise ValueError("Spot light direction cannot be zero.")
            value = value.normalized()
        elif name == "angle":
            if value.as_radians <= 0.0 or value.as_radians >= math.pi:
                raise ValueError(
                    "Spot light angle must be between 0 and pi radians."
                )
        elif name == "intensity" and float(value) < 0.0:
            raise ValueError("Light intensity cannot be negative.")

        super().__setattr__(name, value)


class DirectionalLight(ctypes.Structure):
    _fields_ = [
        ("direction", Vec3),
        ("color", Color),
        ("intensity", ctypes.c_float),
    ]

    def __init__(
        self,
        direction: Vec3,
        color: Color,
        intensity: float,
    ) -> None:
        if direction.length <= 1.0e-6:
            raise ValueError(
                "Directional light direction cannot be zero."
            )

        if intensity < 0.0:
            raise ValueError(
                "Light intensity cannot be negative."
            )

        super().__init__(
            direction.normalized(),
            color,
            float(intensity),
        )

    def __setattr__(self, name, value):
        if name == "direction":
            if value.length <= 1.0e-6:
                raise ValueError(
                    "Directional light direction cannot be zero."
                )
            value = value.normalized()
        elif name == "intensity" and float(value) < 0.0:
            raise ValueError("Light intensity cannot be negative.")

        super().__setattr__(name, value)


class Camera:
    def __init__(self) -> None:
        handle = (
            _native.NativeCameraHandle()
        )

        _native.check(
            _native.lib.oxide_camera_create(
                ctypes.byref(handle)
            )
        )

        self._handle = handle

        self._finalizer = weakref.finalize(
            self,
            Camera._destroy,
            handle,
        )

    @staticmethod
    def _destroy(
        handle: _native.NativeCameraHandle,
    ) -> None:
        # Finalizers must not raise.
        _native.lib.oxide_camera_destroy(
            handle
        )

    @property
    def position(self) -> Vec3:
        result = Vec3()

        _native.check(
            _native.lib
            .oxide_camera_get_position(
                self._handle,
                ctypes.byref(result),
            )
        )

        return result

    @position.setter
    def position(
        self,
        value: Vec3,
    ) -> None:
        _native.check(
            _native.lib
            .oxide_camera_set_position(
                self._handle,
                value,
            )
        )

    @property
    def rotation(self) -> Quaternion:
        result = Quaternion()

        _native.check(
            _native.lib
            .oxide_camera_get_rotation(
                self._handle,
                ctypes.byref(result),
            )
        )

        return result

    @rotation.setter
    def rotation(
        self,
        value: Quaternion,
    ) -> None:
        _native.check(
            _native.lib
            .oxide_camera_set_rotation(
                self._handle,
                value,
            )
        )

    @property
    def field_of_view(self) -> Angle:
        result = Angle()

        _native.check(
            _native.lib
            .oxide_camera_get_field_of_view(
                self._handle,
                ctypes.byref(result),
            )
        )

        return result

    @field_of_view.setter
    def field_of_view(
        self,
        value: Angle,
    ) -> None:
        _native.check(
            _native.lib
            .oxide_camera_set_field_of_view(
                self._handle,
                value,
            )
        )

    @property
    def near_plane(self) -> float:
        result = ctypes.c_float()

        _native.check(
            _native.lib
            .oxide_camera_get_near_plane(
                self._handle,
                ctypes.byref(result),
            )
        )

        return float(
            result.value
        )

    @property
    def far_plane(self) -> float:
        result = ctypes.c_float()

        _native.check(
            _native.lib
            .oxide_camera_get_far_plane(
                self._handle,
                ctypes.byref(result),
            )
        )

        return float(
            result.value
        )

    def translate_by(
        self,
        translation: Vec3,
    ) -> None:
        _native.check(
            _native.lib
            .oxide_camera_translate_by(
                self._handle,
                translation,
            )
        )

    def set_euler(
        self,
        pitch: Angle,
        yaw: Angle,
        roll: Angle,
    ) -> None:
        _native.check(
            _native.lib
            .oxide_camera_set_euler(
                self._handle,
                pitch,
                yaw,
                roll,
            )
        )

    def rotate_pitch_by(
        self,
        angle: Angle,
    ) -> None:
        _native.check(
            _native.lib
            .oxide_camera_rotate_pitch_by(
                self._handle,
                angle,
            )
        )

    def rotate_yaw_by(
        self,
        angle: Angle,
    ) -> None:
        _native.check(
            _native.lib
            .oxide_camera_rotate_yaw_by(
                self._handle,
                angle,
            )
        )

    def rotate_roll_by(
        self,
        angle: Angle,
    ) -> None:
        _native.check(
            _native.lib
            .oxide_camera_rotate_roll_by(
                self._handle,
                angle,
            )
        )

    def set_clip_planes(
        self,
        near_plane: float,
        far_plane: float,
    ) -> None:
        _native.check(
            _native.lib
            .oxide_camera_set_clip_planes(
                self._handle,
                float(near_plane),
                float(far_plane),
            )
        )


class Graphics:
    def __init__(
        self,
        handle: _native.NativeGraphicsHandle,
    ) -> None:
        self._handle = handle
        self._valid = True

    def _require_valid(self) -> None:
        if not self._valid:
            raise RuntimeError(
                "Graphics is only valid during "
                "the current draw callback."
            )

    def _invalidate(self) -> None:
        self._valid = False

    @property
    def preferred_backbuffer_width(self) -> int:
        return _native.lib.oxide_app_window_width()

    @preferred_backbuffer_width.setter
    def preferred_backbuffer_width(
        self,
        value: int,
    ) -> None:
        if value <= 0:
            raise ValueError(
                "backbuffer width must be greater than zero"
            )

        _native.check(
            _native.lib.oxide_app_set_preferred_backbuffer_size(
                value,
                _native.lib.oxide_app_window_height(),
            )
        )

    @property
    def preferred_backbuffer_height(self) -> int:
        return _native.lib.oxide_app_window_height()

    @preferred_backbuffer_height.setter
    def preferred_backbuffer_height(
        self,
        value: int,
    ) -> None:
        if value <= 0:
            raise ValueError(
                "backbuffer height must be greater than zero"
            )

        _native.check(
            _native.lib.oxide_app_set_preferred_backbuffer_size(
                _native.lib.oxide_app_window_width(),
                value,
            )
        )

    @property
    def full_rect(self) -> Rect:
        self._require_valid()

        result = Rect()

        _native.check(
            _native.lib
            .oxide_graphics_full_rect(
                self._handle,
                ctypes.byref(result),
            )
        )

        return result

    def bind_camera(
        self,
        camera: Camera,
        destination: Rect,
    ) -> None:
        self._require_valid()

        _native.check(
            _native.lib
            .oxide_graphics_bind_camera(
                self._handle,
                camera._handle,
                destination,
            )
        )

    def draw_mesh(
        self,
        mesh: Mesh,
        material: Material,
        transform: Transform,
    ) -> None:
        self._require_valid()

        if not isinstance(mesh, Mesh):
            raise TypeError("mesh must be a Mesh")

        if not isinstance(material, Material):
            raise TypeError("material must be a Material")

        _native.check(
            _native.lib.oxide_graphics_draw_mesh(
                self._handle,
                mesh._handle,
                material._handle,
                transform,
            )
        )

    def set_environment(
        self,
        environment: Environment,
    ) -> None:
        self._require_valid()
        if not isinstance(environment, Environment):
            raise TypeError("environment must be an Environment")
        _native.check(
            _native.lib.oxide_graphics_set_environment_explicit(
                self._handle,
                environment.ambient_color,
                environment.ambient_intensity,
                (
                    environment.environment_texture._handle
                    if environment.environment_texture is not None
                    else _native.NativeTextureHandle()
                ),
                environment.environment_texture is not None,
                environment.environment_intensity,
                environment.background_color,
            )
        )

    def clear_environment(self) -> None:
        self._require_valid()
        _native.check(
            _native.lib.oxide_graphics_clear_environment(
                self._handle
            )
        )

    def clear_lights(self) -> None:
        self._require_valid()

        _native.check(
            _native.lib
            .oxide_graphics_clear_lights(
                self._handle
            )
        )

    def add_light(
        self,
        light: (
            PointLight
            | SpotLight
            | DirectionalLight
        ),
    ) -> None:
        self._require_valid()

        if isinstance(
            light,
            PointLight,
        ):
            status = (
                _native.lib
                .oxide_graphics_add_point_light(
                    self._handle,
                    light,
                )
            )

        elif isinstance(
            light,
            SpotLight,
        ):
            status = (
                _native.lib
                .oxide_graphics_add_spot_light(
                    self._handle,
                    light,
                )
            )

        elif isinstance(
            light,
            DirectionalLight,
        ):
            status = (
                _native.lib
                .oxide_graphics_add_directional_light(
                    self._handle,
                    light,
                )
            )

        else:
            raise TypeError(
                "light must be a PointLight, "
                "SpotLight, or DirectionalLight"
            )

        _native.check(
            status
        )



for _name in ("base_color", "emissive_color"):
    _get = getattr(_native.lib, f"oxide_material_get_{_name}")
    _get.argtypes = [
        _native.NativeMaterialHandle,
        ctypes.POINTER(Color),
    ]
    _get.restype = ctypes.c_int

    _set = getattr(_native.lib, f"oxide_material_set_{_name}")
    _set.argtypes = [
        _native.NativeMaterialHandle,
        Color,
    ]
    _set.restype = ctypes.c_int


_native.lib.oxide_graphics_set_environment_explicit.argtypes = [
    _native.NativeGraphicsHandle,
    Color,
    ctypes.c_float,
    _native.NativeTextureHandle,
    ctypes.c_bool,
    ctypes.c_float,
    Color,
]
_native.lib.oxide_graphics_set_environment_explicit.restype = ctypes.c_int
_native.lib.oxide_graphics_clear_environment.argtypes = [
    _native.NativeGraphicsHandle,
]
_native.lib.oxide_graphics_clear_environment.restype = ctypes.c_int

_native.lib.oxide_graphics_draw_mesh.argtypes = [
    _native.NativeGraphicsHandle,
    _native.NativeMeshHandle,
    _native.NativeMaterialHandle,
    Transform,
]
_native.lib.oxide_graphics_draw_mesh.restype = ctypes.c_int

_native.lib.oxide_texture_solid.argtypes = [
    Color,
    ctypes.POINTER(_native.NativeTextureHandle),
]
_native.lib.oxide_texture_solid.restype = ctypes.c_int

_native.lib.oxide_material_solid.argtypes = [
    Color,
    ctypes.POINTER(_native.NativeMaterialHandle),
]
_native.lib.oxide_material_solid.restype = ctypes.c_int

_native.lib.oxide_graphics_add_point_light.argtypes = [
    _native.NativeGraphicsHandle,
    PointLight,
]
_native.lib.oxide_graphics_add_point_light.restype = (
    ctypes.c_int
)

_native.lib.oxide_graphics_add_spot_light.argtypes = [
    _native.NativeGraphicsHandle,
    SpotLight,
]
_native.lib.oxide_graphics_add_spot_light.restype = (
    ctypes.c_int
)

_native.lib.oxide_graphics_add_directional_light.argtypes = [
    _native.NativeGraphicsHandle,
    DirectionalLight,
]
_native.lib.oxide_graphics_add_directional_light.restype = (
    ctypes.c_int
)