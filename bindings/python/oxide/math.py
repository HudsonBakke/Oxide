from __future__ import annotations

import ctypes
import math as _math


class Angle(ctypes.Structure):
    """Angle stored in radians with degree conversion helpers."""
    _fields_ = [
        ("radians", ctypes.c_float),
    ]

    def __init__(
        self,
        radians: float = 0.0,
    ) -> None:
        super().__init__(
            float(radians)
        )

    @classmethod
    def zero(cls) -> Angle:
        """Public Oxide zero operation."""
        return cls(0.0)

    @classmethod
    def from_radians(
        cls,
        radians: float,
    ) -> Angle:
        """Public Oxide from radians operation."""
        return cls(radians)

    @classmethod
    def degrees(
        cls,
        degrees: float,
    ) -> Angle:
        """Public Oxide degrees operation."""
        return cls(
            _math.radians(degrees)
        )

    @property
    def as_radians(self) -> float:
        """Public Oxide as radians operation."""
        return float(self.radians)

    @property
    def as_degrees(self) -> float:
        """Public Oxide as degrees operation."""
        return _math.degrees(
            self.radians
        )

    def __eq__(
        self,
        other: object,
    ) -> bool:
        if not isinstance(
            other,
            Angle,
        ):
            return NotImplemented

        return self.radians == other.radians

    def __repr__(self) -> str:
        return (
            f"Angle({self.as_degrees:.3f}°)"
        )


class Vec2(ctypes.Structure):
    """Two-component floating-point vector."""
    _fields_ = [
        ("x", ctypes.c_float),
        ("y", ctypes.c_float),
    ]

    def __init__(
        self,
        x: float = 0.0,
        y: float = 0.0,
    ) -> None:
        super().__init__(
            float(x),
            float(y),
        )

    @property
    def length(self) -> float:
        """Public Oxide length operation."""
        return _math.sqrt(
            self.x * self.x
            + self.y * self.y
        )

    def normalized(self) -> Vec2:
        """Return a normalized copy."""
        length = self.length

        if length == 0.0:
            return Vec2()

        return Vec2(
            self.x / length,
            self.y / length,
        )

    def normalize(self) -> None:
        """Normalize this value in place."""
        length = self.length

        if length == 0.0:
            return

        self.x /= length
        self.y /= length

    def dot(
        self,
        other: Vec2,
    ) -> float:
        """Return the dot product."""
        return (
            self.x * other.x
            + self.y * other.y
        )

    def __add__(
        self,
        other: Vec2,
    ) -> Vec2:
        return Vec2(
            self.x + other.x,
            self.y + other.y,
        )

    def __sub__(
        self,
        other: Vec2,
    ) -> Vec2:
        return Vec2(
            self.x - other.x,
            self.y - other.y,
        )

    def __mul__(
        self,
        scalar: float,
    ) -> Vec2:
        return Vec2(
            self.x * scalar,
            self.y * scalar,
        )

    def __truediv__(
        self,
        scalar: float,
    ) -> Vec2:
        return Vec2(
            self.x / scalar,
            self.y / scalar,
        )

    def __eq__(
        self,
        other: object,
    ) -> bool:
        if not isinstance(
            other,
            Vec2,
        ):
            return NotImplemented

        return (
            self.x == other.x
            and self.y == other.y
        )

    def __repr__(self) -> str:
        return (
            f"Vec2({self.x}, {self.y})"
        )


class Vec3(ctypes.Structure):
    """Three-component floating-point vector."""
    _fields_ = [
        ("x", ctypes.c_float),
        ("y", ctypes.c_float),
        ("z", ctypes.c_float),
    ]

    def __init__(
        self,
        x: float = 0.0,
        y: float = 0.0,
        z: float = 0.0,
    ) -> None:
        super().__init__(
            float(x),
            float(y),
            float(z),
        )

    @property
    def length(self) -> float:
        """Public Oxide length operation."""
        return _math.sqrt(
            self.x * self.x
            + self.y * self.y
            + self.z * self.z
        )

    def normalized(self) -> Vec3:
        """Return a normalized copy."""
        length = self.length

        if length == 0.0:
            return Vec3()

        return Vec3(
            self.x / length,
            self.y / length,
            self.z / length,
        )

    def normalize(self) -> None:
        """Normalize this value in place."""
        length = self.length

        if length == 0.0:
            return

        self.x /= length
        self.y /= length
        self.z /= length

    def dot(
        self,
        other: Vec3,
    ) -> float:
        """Return the dot product."""
        return (
            self.x * other.x
            + self.y * other.y
            + self.z * other.z
        )

    def cross(
        self,
        other: Vec3,
    ) -> Vec3:
        """Return the three-dimensional cross product."""
        return Vec3(
            self.y * other.z
            - self.z * other.y,

            self.z * other.x
            - self.x * other.z,

            self.x * other.y
            - self.y * other.x,
        )

    def __add__(
        self,
        other: Vec3,
    ) -> Vec3:
        return Vec3(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
        )

    def __sub__(
        self,
        other: Vec3,
    ) -> Vec3:
        return Vec3(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
        )

    def __mul__(
        self,
        scalar: float,
    ) -> Vec3:
        return Vec3(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
        )

    def __truediv__(
        self,
        scalar: float,
    ) -> Vec3:
        return Vec3(
            self.x / scalar,
            self.y / scalar,
            self.z / scalar,
        )

    def __eq__(
        self,
        other: object,
    ) -> bool:
        if not isinstance(
            other,
            Vec3,
        ):
            return NotImplemented

        return (
            self.x == other.x
            and self.y == other.y
            and self.z == other.z
        )

    def __repr__(self) -> str:
        return (
            f"Vec3("
            f"{self.x}, "
            f"{self.y}, "
            f"{self.z}"
            f")"
        )


class Vec4(ctypes.Structure):
    """Four-component floating-point vector."""
    _fields_ = [
        ("x", ctypes.c_float),
        ("y", ctypes.c_float),
        ("z", ctypes.c_float),
        ("w", ctypes.c_float),
    ]

    def __init__(
        self,
        x: float = 0.0,
        y: float = 0.0,
        z: float = 0.0,
        w: float = 0.0,
    ) -> None:
        super().__init__(
            float(x),
            float(y),
            float(z),
            float(w),
        )

    @classmethod
    def from_position(
        cls,
        value: Vec3,
    ) -> Vec4:
        """Public Oxide from position operation."""
        return cls(
            value.x,
            value.y,
            value.z,
            1.0,
        )

    @classmethod
    def from_direction(
        cls,
        value: Vec3,
    ) -> Vec4:
        """Public Oxide from direction operation."""
        return cls(
            value.x,
            value.y,
            value.z,
            0.0,
        )

    def __add__(
        self,
        other: Vec4,
    ) -> Vec4:
        return Vec4(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )

    def __sub__(
        self,
        other: Vec4,
    ) -> Vec4:
        return Vec4(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )

    def __mul__(
        self,
        scalar: float,
    ) -> Vec4:
        return Vec4(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
            self.w * scalar,
        )

    def __truediv__(
        self,
        scalar: float,
    ) -> Vec4:
        return Vec4(
            self.x / scalar,
            self.y / scalar,
            self.z / scalar,
            self.w / scalar,
        )

    def __eq__(
        self,
        other: object,
    ) -> bool:
        if not isinstance(
            other,
            Vec4,
        ):
            return NotImplemented

        return (
            self.x == other.x
            and self.y == other.y
            and self.z == other.z
            and self.w == other.w
        )

    def __repr__(self) -> str:
        return (
            f"Vec4("
            f"{self.x}, "
            f"{self.y}, "
            f"{self.z}, "
            f"{self.w}"
            f")"
        )


class Quaternion(ctypes.Structure):
    """Quaternion rotation for three-dimensional orientation."""
    _fields_ = [
        ("x", ctypes.c_float),
        ("y", ctypes.c_float),
        ("z", ctypes.c_float),
        ("w", ctypes.c_float),
    ]

    def __init__(
        self,
        x: float = 0.0,
        y: float = 0.0,
        z: float = 0.0,
        w: float = 1.0,
    ) -> None:
        super().__init__(
            float(x),
            float(y),
            float(z),
            float(w),
        )

    @classmethod
    def identity(cls) -> Quaternion:
        """Public Oxide identity operation."""
        return cls(
            0.0,
            0.0,
            0.0,
            1.0,
        )

    @classmethod
    def from_axis_angle(
        cls,
        axis: Vec3,
        angle: Angle,
    ) -> Quaternion:
        """Public Oxide from axis angle operation."""
        axis = axis.normalized()

        half_angle = (
            angle.as_radians
            * 0.5
        )

        sin = _math.sin(
            half_angle
        )

        return cls(
            axis.x * sin,
            axis.y * sin,
            axis.z * sin,
            _math.cos(
                half_angle
            ),
        )

    @classmethod
    def from_euler(
        cls,
        pitch: Angle,
        yaw: Angle,
        roll: Angle,
    ) -> Quaternion:
        """Public Oxide from euler operation."""
        pitch_rotation = (
            cls.from_axis_angle(
                Vec3(
                    1.0,
                    0.0,
                    0.0,
                ),
                pitch,
            )
        )

        yaw_rotation = (
            cls.from_axis_angle(
                Vec3(
                    0.0,
                    1.0,
                    0.0,
                ),
                yaw,
            )
        )

        roll_rotation = (
            cls.from_axis_angle(
                Vec3(
                    0.0,
                    0.0,
                    1.0,
                ),
                roll,
            )
        )

        return (
            yaw_rotation
            * pitch_rotation
            * roll_rotation
        ).normalized()

    @classmethod
    def from_to(
        cls,
        from_vector: Vec3,
        to_vector: Vec3,
    ) -> Quaternion:
        """Public Oxide from to operation."""
        from_vector = (
            from_vector.normalized()
        )

        to_vector = (
            to_vector.normalized()
        )

        dot = from_vector.dot(
            to_vector
        )

        if dot > 1.0 - 1.0e-6:
            return cls.identity()

        if dot < -1.0 + 1.0e-6:
            if (
                abs(from_vector.x)
                <= abs(from_vector.y)
                and abs(from_vector.x)
                <= abs(from_vector.z)
            ):
                candidate = Vec3(
                    1.0,
                    0.0,
                    0.0,
                )
            elif (
                abs(from_vector.y)
                <= abs(from_vector.z)
            ):
                candidate = Vec3(
                    0.0,
                    1.0,
                    0.0,
                )
            else:
                candidate = Vec3(
                    0.0,
                    0.0,
                    1.0,
                )

            axis = (
                from_vector
                .cross(candidate)
                .normalized()
            )

            return cls.from_axis_angle(
                axis,
                Angle.from_radians(
                    _math.pi
                ),
            )

        axis = from_vector.cross(
            to_vector
        )

        return cls(
            axis.x,
            axis.y,
            axis.z,
            1.0 + dot,
        ).normalized()

    @property
    def length(self) -> float:
        """Public Oxide length operation."""
        return _math.sqrt(
            self.x * self.x
            + self.y * self.y
            + self.z * self.z
            + self.w * self.w
        )

    def normalized(self) -> Quaternion:
        """Return a normalized copy."""
        length = self.length

        if length == 0.0:
            return Quaternion.identity()

        return Quaternion(
            self.x / length,
            self.y / length,
            self.z / length,
            self.w / length,
        )

    def conjugate(self) -> Quaternion:
        """Public Oxide conjugate operation."""
        return Quaternion(
            -self.x,
            -self.y,
            -self.z,
            self.w,
        )

    def rotate_vector(
        self,
        vector: Vec3,
    ) -> Vec3:
        """Public Oxide rotate vector operation."""
        q = Vec3(
            self.x,
            self.y,
            self.z,
        )

        t = q.cross(vector) * 2.0

        return (
            vector
            + t * self.w
            + q.cross(t)
        )

    def __mul__(
        self,
        other: Quaternion,
    ) -> Quaternion:
        return Quaternion(
            self.w * other.x
            + self.x * other.w
            + self.y * other.z
            - self.z * other.y,

            self.w * other.y
            - self.x * other.z
            + self.y * other.w
            + self.z * other.x,

            self.w * other.z
            + self.x * other.y
            - self.y * other.x
            + self.z * other.w,

            self.w * other.w
            - self.x * other.x
            - self.y * other.y
            - self.z * other.z,
        )

    def __eq__(
        self,
        other: object,
    ) -> bool:
        if not isinstance(
            other,
            Quaternion,
        ):
            return NotImplemented

        return (
            self.x == other.x
            and self.y == other.y
            and self.z == other.z
            and self.w == other.w
        )

    def __repr__(self) -> str:
        return (
            f"Quaternion("
            f"{self.x}, "
            f"{self.y}, "
            f"{self.z}, "
            f"{self.w}"
            f")"
        )


class Rect(ctypes.Structure):
    """Rectangle expressed by position and size."""
    _fields_ = [
        ("x", ctypes.c_float),
        ("y", ctypes.c_float),
        ("width", ctypes.c_float),
        ("height", ctypes.c_float),
    ]

    def __init__(
        self,
        x: float = 0.0,
        y: float = 0.0,
        width: float = 0.0,
        height: float = 0.0,
    ) -> None:
        super().__init__(
            float(x),
            float(y),
            float(width),
            float(height),
        )

    @property
    def aspect_ratio(self) -> float:
        """Public Oxide aspect ratio operation."""
        return (
            self.width
            / self.height
        )

    def __eq__(
        self,
        other: object,
    ) -> bool:
        if not isinstance(
            other,
            Rect,
        ):
            return NotImplemented

        return (
            self.x == other.x
            and self.y == other.y
            and self.width == other.width
            and self.height == other.height
        )

    def __repr__(self) -> str:
        return (
            f"Rect("
            f"{self.x}, "
            f"{self.y}, "
            f"{self.width}, "
            f"{self.height}"
            f")"
        )


class Transform(ctypes.Structure):
    """Position, rotation, and scale describing an object's placement."""
    _fields_ = [
        ("position", Vec3),
        ("rotation", Quaternion),
        ("scale", Vec3),
    ]

    def __init__(
        self,
        position: Vec3 | None = None,
        rotation: Quaternion | None = None,
        scale: Vec3 | None = None,
    ) -> None:
        super().__init__(
            position
            if position is not None
            else Vec3(),

            (
                rotation.normalized()
                if rotation is not None
                else Quaternion.identity()
            ),

            scale
            if scale is not None
            else Vec3(
                1.0,
                1.0,
                1.0,
            ),
        )

    def __setattr__(self, name, value):
        if name == "rotation":
            if not isinstance(value, Quaternion):
                raise TypeError("rotation must be a Quaternion")
            value = value.normalized()
        elif name == "position" and not isinstance(value, Vec3):
            raise TypeError("position must be a Vec3")
        elif name == "scale" and not isinstance(value, Vec3):
            raise TypeError("scale must be a Vec3")

        super().__setattr__(name, value)

    def translate_by(
        self,
        translation: Vec3,
    ) -> None:
        """Add a world-space translation."""
        self.position = (
            self.position
            + translation
        )

    def set_rotation(
        self,
        rotation: Quaternion,
    ) -> None:
        """Public Oxide set rotation operation."""
        self.rotation = (
            rotation.normalized()
        )

    def set_euler(
        self,
        pitch: Angle,
        yaw: Angle,
        roll: Angle,
    ) -> None:
        """Set orientation from pitch, yaw, and roll."""
        self.rotation = (
            Quaternion.from_euler(
                pitch,
                yaw,
                roll,
            )
        )

    def rotate_pitch_by(
        self,
        angle: Angle,
    ) -> None:
        """Public Oxide rotate pitch by operation."""
        rotation = (
            Quaternion.from_axis_angle(
                Vec3(
                    1.0,
                    0.0,
                    0.0,
                ),
                angle,
            )
        )

        self.rotation = (
            rotation
            * self.rotation
        ).normalized()

    def rotate_yaw_by(
        self,
        angle: Angle,
    ) -> None:
        """Public Oxide rotate yaw by operation."""
        rotation = (
            Quaternion.from_axis_angle(
                Vec3(
                    0.0,
                    1.0,
                    0.0,
                ),
                angle,
            )
        )

        self.rotation = (
            rotation
            * self.rotation
        ).normalized()

    def rotate_roll_by(
        self,
        angle: Angle,
    ) -> None:
        """Public Oxide rotate roll by operation."""
        rotation = (
            Quaternion.from_axis_angle(
                Vec3(
                    0.0,
                    0.0,
                    1.0,
                ),
                angle,
            )
        )

        self.rotation = (
            rotation
            * self.rotation
        ).normalized()

    def scale_by(
        self,
        scale: Vec3,
    ) -> None:
        """Public Oxide scale by operation."""
        self.scale = Vec3(
            self.scale.x * scale.x,
            self.scale.y * scale.y,
            self.scale.z * scale.z,
        )

    @classmethod
    def from_attachment(
        cls,
        parent,
        parent_transform: Transform,
        parent_attach_point: Vec3,
        child,
        child_attach_point: Vec3,
        direction: Vec3,
        rotation: Angle,
        scale: Vec3,
    ) -> Transform:
        """Compute a child transform aligning two mesh attachment points."""
        # Import lazily so the ABI math types remain independent
        # of the resource layer during module initialization.
        import ctypes
        from . import _native
        from .content import Mesh

        if not isinstance(parent, Mesh):
            raise TypeError("parent must be a Mesh")

        if not isinstance(child, Mesh):
            raise TypeError("child must be a Mesh")

        result = cls()

        _native.check(
            _native.lib.oxide_transform_from_attachment(
                parent._handle,
                parent_transform,
                parent_attach_point,
                child._handle,
                child_attach_point,
                direction,
                rotation,
                scale,
                ctypes.byref(result),
            )
        )

        return result

    def __eq__(
        self,
        other: object,
    ) -> bool:
        if not isinstance(
            other,
            Transform,
        ):
            return NotImplemented

        return (
            self.position
            == other.position
            and self.rotation
            == other.rotation
            and self.scale
            == other.scale
        )

    def __repr__(self) -> str:
        return (
            "Transform("
            f"position={self.position!r}, "
            f"rotation={self.rotation!r}, "
            f"scale={self.scale!r}"
            ")"
        )