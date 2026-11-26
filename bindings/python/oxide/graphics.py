"""Camera, lighting, and rendering commands for a draw callback."""

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
    """Floating-point RGBA color; values are stored without clamping.

    Attributes:
        r: Red component.
        g: Green component.
        b: Blue component.
        a: Alpha component: zero is transparent and one is opaque."""

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
        """Creates a color from RGBA components without clamping."""
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
        """Creates an RGB color with alpha set to one."""
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
        """Creates a color from red, green, blue, and alpha components without clamping."""
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
        """Tests exact component equality; returns NotImplemented for an unrelated type."""
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
        """Returns a representation containing the component values."""
        return f"Color(" f"{self.r}, " f"{self.g}, " f"{self.b}, " f"{self.a}" f")"


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
    """Independent ambient, image-based lighting, and equirectangular background settings.

    Attributes:
        ambient_color: RGB color of constant ambient lighting.
        ambient_intensity: Finite, non-negative multiplier for constant ambient lighting.
        environment_texture: Optional equirectangular texture used for diffuse and specular image-based lighting.
        environment_intensity: Finite, non-negative multiplier for image-based lighting.
        background_texture: Optional equirectangular texture displayed behind geometry, independent of lighting.
        background_color: Clear color used when no background texture is displayed."""

    def __init__(
        self,
        ambient_color: Color,
        ambient_intensity: float,
        environment_texture: Texture | None,
        environment_intensity: float,
        background_texture: Texture | None,
        background_color: Color,
    ) -> None:
        """Creates explicit lighting and background settings; intensity multipliers must be finite and non-negative."""
        if not math.isfinite(ambient_intensity) or ambient_intensity < 0.0:
            raise ValueError("ambient intensity must be finite and non-negative")
        if not math.isfinite(environment_intensity) or environment_intensity < 0.0:
            raise ValueError("environment intensity must be finite and non-negative")

        self.ambient_color = ambient_color
        self.ambient_intensity = float(ambient_intensity)
        self.environment_texture = environment_texture
        self.environment_intensity = float(environment_intensity)
        self.background_texture = background_texture
        self.background_color = background_color


class PointLight(ctypes.Structure):
    """Omnidirectional point light with inverse-square distance attenuation.

    Attributes:
        position: Position in world space.
        color: RGB light color; alpha is not used for lighting.
        intensity: Non-negative light intensity; point and spot lights attenuate with squared distance.
    """

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
        """Creates a point light; intensity must not be negative."""
        if intensity < 0.0:
            raise ValueError("Light intensity cannot be negative.")

        super().__init__(
            position,
            color,
            float(intensity),
        )

    def __setattr__(self, name, value):
        """Assigns a field, applying this type's normalization and validation rules."""
        if name == "intensity" and float(value) < 0.0:
            raise ValueError("Light intensity cannot be negative.")

        super().__setattr__(name, value)


class SpotLight(ctypes.Structure):
    """Point light restricted to a directional cone, with inverse-square distance attenuation.

    Attributes:
        position: Position in world space.
        direction: Normalized world-space direction in which light rays travel.
        angle: Spotlight cone half-angle measured from its direction; use an angle strictly between zero and pi radians.
        color: RGB light color; alpha is not used for lighting.
        intensity: Non-negative light intensity; point and spot lights attenuate with squared distance.
    """

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
        """Creates a cone light; requires a nonzero direction, non-negative intensity, and half-angle in (0, pi) radians."""
        if direction.length <= 1.0e-6:
            raise ValueError("Spot light direction cannot be zero.")

        if angle.as_radians <= 0.0 or angle.as_radians >= math.pi:
            raise ValueError("Spot light angle must be " "between 0 and pi radians.")

        if intensity < 0.0:
            raise ValueError("Light intensity cannot be negative.")

        super().__init__(
            position,
            direction.normalized(),
            angle,
            color,
            float(intensity),
        )

    def __setattr__(self, name, value):
        """Assigns a field, applying this type's normalization and validation rules."""
        if name == "direction":
            if value.length <= 1.0e-6:
                raise ValueError("Spot light direction cannot be zero.")
            value = value.normalized()
        elif name == "angle":
            if value.as_radians <= 0.0 or value.as_radians >= math.pi:
                raise ValueError("Spot light angle must be between 0 and pi radians.")
        elif name == "intensity" and float(value) < 0.0:
            raise ValueError("Light intensity cannot be negative.")

        super().__setattr__(name, value)


class DirectionalLight(ctypes.Structure):
    """Light whose rays travel in one world-space direction, with no distance attenuation.

    Attributes:
        direction: Normalized world-space direction in which light rays travel.
        color: RGB light color; alpha is not used for lighting.
        intensity: Non-negative light intensity; point and spot lights attenuate with squared distance.
    """

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
        """Creates a directional light; requires a nonzero direction and non-negative intensity."""
        if direction.length <= 1.0e-6:
            raise ValueError("Directional light direction cannot be zero.")

        if intensity < 0.0:
            raise ValueError("Light intensity cannot be negative.")

        super().__init__(
            direction.normalized(),
            color,
            float(intensity),
        )

    def __setattr__(self, name, value):
        """Assigns a field, applying this type's normalization and validation rules."""
        if name == "direction":
            if value.length <= 1.0e-6:
                raise ValueError("Directional light direction cannot be zero.")
            value = value.normalized()
        elif name == "intensity" and float(value) < 0.0:
            raise ValueError("Light intensity cannot be negative.")

        super().__setattr__(name, value)


class Camera:
    """Perspective camera looking along local -Z with +Y up at its default orientation.

    Native resources are released automatically when their Python owner is collected."""

    def __init__(self) -> None:
        """Creates a camera at the origin with identity rotation, a 60-degree vertical field of view, and clipping distances 0.1 and 1000."""
        handle = _native.NativeCameraHandle()

        _native.check(_native.lib.oxide_camera_create(ctypes.byref(handle)))

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
        _native.lib.oxide_camera_destroy(handle)

    @property
    def position(self) -> Vec3:
        """Position in world space."""
        result = Vec3()

        _native.check(
            _native.lib.oxide_camera_get_position(
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
        """Position in world space."""
        _native.check(
            _native.lib.oxide_camera_set_position(
                self._handle,
                value,
            )
        )

    @property
    def rotation(self) -> Quaternion:
        """Orientation quaternion; assigned values are normalized."""
        result = Quaternion()

        _native.check(
            _native.lib.oxide_camera_get_rotation(
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
        """Orientation quaternion; assigned values are normalized."""
        _native.check(
            _native.lib.oxide_camera_set_rotation(
                self._handle,
                value,
            )
        )

    @property
    def field_of_view(self) -> Angle:
        """Vertical perspective field of view; use an angle strictly between zero and pi radians."""
        result = Angle()

        _native.check(
            _native.lib.oxide_camera_get_field_of_view(
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
        """Vertical perspective field of view; use an angle strictly between zero and pi radians."""
        _native.check(
            _native.lib.oxide_camera_set_field_of_view(
                self._handle,
                value,
            )
        )

    @property
    def near_plane(self) -> float:
        """Distance from the camera to the near clipping plane in world units."""
        result = ctypes.c_float()

        _native.check(
            _native.lib.oxide_camera_get_near_plane(
                self._handle,
                ctypes.byref(result),
            )
        )

        return float(result.value)

    @property
    def far_plane(self) -> float:
        """Distance from the camera to the far clipping plane in world units."""
        result = ctypes.c_float()

        _native.check(
            _native.lib.oxide_camera_get_far_plane(
                self._handle,
                ctypes.byref(result),
            )
        )

        return float(result.value)

    def translate_by(
        self,
        translation: Vec3,
    ) -> None:
        """Adds a world-space translation without rotating it by the current orientation."""
        _native.check(
            _native.lib.oxide_camera_translate_by(
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
        """Sets orientation to yaw * pitch * roll, about Y, X, and Z respectively."""
        _native.check(
            _native.lib.oxide_camera_set_euler(
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
        """Applies an additional rotation about the world X axis."""
        _native.check(
            _native.lib.oxide_camera_rotate_pitch_by(
                self._handle,
                angle,
            )
        )

    def rotate_yaw_by(
        self,
        angle: Angle,
    ) -> None:
        """Applies an additional rotation about the world Y axis."""
        _native.check(
            _native.lib.oxide_camera_rotate_yaw_by(
                self._handle,
                angle,
            )
        )

    def rotate_roll_by(
        self,
        angle: Angle,
    ) -> None:
        """Applies an additional rotation about the world Z axis."""
        _native.check(
            _native.lib.oxide_camera_rotate_roll_by(
                self._handle,
                angle,
            )
        )

    def set_clip_planes(
        self,
        near_plane: float,
        far_plane: float,
    ) -> None:
        """Sets clipping distances in world units; requires 0 < near < far."""
        _native.check(
            _native.lib.oxide_camera_set_clip_planes(
                self._handle,
                float(near_plane),
                float(far_plane),
            )
        )


class Graphics:
    """Rendering interface supplied to the draw callback; bind a camera before drawing geometry.

    Valid only during Game.draw; retaining this object for later use raises RuntimeError.
    """

    def __init__(
        self,
        handle: _native.NativeGraphicsHandle,
    ) -> None:
        """Wraps a borrowed rendering handle supplied by the current draw callback."""
        self._handle = handle
        self._valid = True

    def _require_valid(self) -> None:
        if not self._valid:
            raise RuntimeError(
                "Graphics is only valid during " "the current draw callback."
            )

    def _invalidate(self) -> None:
        self._valid = False

    @property
    def exposure(self) -> float:
        """Linear exposure multiplier; defaults to one and persists across frames.

        The last value set during draw applies to all viewports and backgrounds.
        """
        self._require_valid()
        result = ctypes.c_float()
        _native.check(
            _native.lib.oxide_graphics_get_exposure(self._handle, ctypes.byref(result))
        )
        return result.value

    @exposure.setter
    def exposure(self, value: float) -> None:
        """Set finite, non-negative exposure; zero produces black."""
        self._require_valid()
        _native.check(
            _native.lib.oxide_graphics_set_exposure(self._handle, float(value))
        )

    @property
    def preferred_backbuffer_width(self) -> int:
        """Current client width in physical pixels; assignment requests a resize."""
        return _native.lib.oxide_app_window_width()

    @preferred_backbuffer_width.setter
    def preferred_backbuffer_width(
        self,
        value: int,
    ) -> None:
        """Current client width in physical pixels; assignment requests a resize."""
        if value <= 0:
            raise ValueError("backbuffer width must be greater than zero")

        _native.check(
            _native.lib.oxide_app_set_preferred_backbuffer_size(
                value,
                _native.lib.oxide_app_window_height(),
            )
        )

    @property
    def preferred_backbuffer_height(self) -> int:
        """Current client height in physical pixels; assignment requests a resize."""
        return _native.lib.oxide_app_window_height()

    @preferred_backbuffer_height.setter
    def preferred_backbuffer_height(
        self,
        value: int,
    ) -> None:
        """Current client height in physical pixels; assignment requests a resize."""
        if value <= 0:
            raise ValueError("backbuffer height must be greater than zero")

        _native.check(
            _native.lib.oxide_app_set_preferred_backbuffer_size(
                _native.lib.oxide_app_window_width(),
                value,
            )
        )

    @property
    def full_rect(self) -> Rect:
        """Rectangle covering the entire render surface in physical pixels."""
        self._require_valid()

        result = Rect()

        _native.check(
            _native.lib.oxide_graphics_full_rect(
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
        """Binds a camera and a positive-size pixel viewport for subsequent draw calls."""
        self._require_valid()

        _native.check(
            _native.lib.oxide_graphics_bind_camera(
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
        """Draws a mesh with a material and world transform in the currently bound camera viewport."""
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
        """Sets ambient lighting, image-based lighting, and the independent visible background."""
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
                (
                    environment.background_texture._handle
                    if environment.background_texture is not None
                    else _native.NativeTextureHandle()
                ),
                environment.background_texture is not None,
                environment.background_color,
            )
        )

    def clear_environment(self) -> None:
        """Removes environment lighting and its background settings."""
        self._require_valid()
        _native.check(_native.lib.oxide_graphics_clear_environment(self._handle))

    def clear_lights(self) -> None:
        """Removes lights added so far in the current frame."""
        self._require_valid()

        _native.check(_native.lib.oxide_graphics_clear_lights(self._handle))

    def add_light(
        self,
        light: PointLight | SpotLight | DirectionalLight,
    ) -> None:
        """Adds a point, spot, or directional light for subsequent draws; lights reset at frame start."""
        self._require_valid()

        if isinstance(
            light,
            PointLight,
        ):
            status = _native.lib.oxide_graphics_add_point_light(
                self._handle,
                light,
            )

        elif isinstance(
            light,
            SpotLight,
        ):
            status = _native.lib.oxide_graphics_add_spot_light(
                self._handle,
                light,
            )

        elif isinstance(
            light,
            DirectionalLight,
        ):
            status = _native.lib.oxide_graphics_add_directional_light(
                self._handle,
                light,
            )

        else:
            raise TypeError(
                "light must be a PointLight, " "SpotLight, or DirectionalLight"
            )

        _native.check(status)


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
    _native.NativeTextureHandle,
    ctypes.c_bool,
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
_native.lib.oxide_graphics_add_point_light.restype = ctypes.c_int

_native.lib.oxide_graphics_add_spot_light.argtypes = [
    _native.NativeGraphicsHandle,
    SpotLight,
]
_native.lib.oxide_graphics_add_spot_light.restype = ctypes.c_int

_native.lib.oxide_graphics_add_directional_light.argtypes = [
    _native.NativeGraphicsHandle,
    DirectionalLight,
]
_native.lib.oxide_graphics_add_directional_light.restype = ctypes.c_int
