"""ctypes-compatible math values. Angles are explicit; world axes are right-handed with +Y up."""

from __future__ import annotations

import ctypes
import math as _math


class Angle(ctypes.Structure):
    """Angle stored in radians; construct from radians or degrees explicitly.

    Attributes:
        radians: Creates an angle from a value measured in radians."""
    _fields_ = [
        ("radians", ctypes.c_float),
    ]

    def __init__(
        self,
        radians: float = 0.0,
    ) -> None:
        """Creates an angle from a value measured in radians."""
        super().__init__(
            float(radians)
        )

    @classmethod
    def zero(cls) -> Angle:
        """An angle of zero radians."""
        return cls(0.0)

    @classmethod
    def from_radians(
        cls,
        radians: float,
    ) -> Angle:
        """Creates an angle from a value measured in radians."""
        return cls(radians)

    @classmethod
    def degrees(
        cls,
        degrees: float,
    ) -> Angle:
        """Creates an angle from a value measured in degrees."""
        return cls(
            _math.radians(degrees)
        )

    @property
    def as_radians(self) -> float:
        """The angle measured in radians."""
        return float(self.radians)

    @property
    def as_degrees(self) -> float:
        """The angle measured in degrees."""
        return _math.degrees(
            self.radians
        )

    def __eq__(
        self,
        other: object,
    ) -> bool:
        """Tests exact component equality; returns NotImplemented for an unrelated type."""
        if not isinstance(
            other,
            Angle,
        ):
            return NotImplemented

        return self.radians == other.radians

    def __repr__(self) -> str:
        """Returns a representation containing the component values."""
        return (
            f"Angle({self.as_degrees:.3f}°)"
        )


class Vec2(ctypes.Structure):
    """Two-component floating-point vector with X and Y coordinates.

    Attributes:
        x: X component.
        y: Y component."""
    _fields_ = [
        ("x", ctypes.c_float),
        ("y", ctypes.c_float),
    ]

    def __init__(
        self,
        x: float = 0.0,
        y: float = 0.0,
    ) -> None:
        """Creates a vector from its X and Y components."""
        super().__init__(
            float(x),
            float(y),
        )

    @property
    def length(self) -> float:
        """Euclidean magnitude of the components."""
        return _math.sqrt(
            self.x * self.x
            + self.y * self.y
        )

    def normalized(self) -> Vec2:
        """Returns a unit-length copy; a zero-length vector remains zero."""
        length = self.length

        if length == 0.0:
            return Vec2()

        return Vec2(
            self.x / length,
            self.y / length,
        )

    def normalize(self) -> None:
        """Normalizes the vector in place; leaves a zero-length vector unchanged."""
        length = self.length

        if length == 0.0:
            return

        self.x /= length
        self.y /= length

    def dot(
        self,
        other: Vec2,
    ) -> float:
        """Returns the scalar dot product with the other vector."""
        return (
            self.x * other.x
            + self.y * other.y
        )

    def __add__(
        self,
        other: Vec2,
    ) -> Vec2:
        """Adds corresponding vector components."""
        return Vec2(
            self.x + other.x,
            self.y + other.y,
        )

    def __sub__(
        self,
        other: Vec2,
    ) -> Vec2:
        """Subtracts corresponding vector components."""
        return Vec2(
            self.x - other.x,
            self.y - other.y,
        )

    def __mul__(
        self,
        scalar: float,
    ) -> Vec2:
        """Multiplies each component by a scalar."""
        return Vec2(
            self.x * scalar,
            self.y * scalar,
        )

    def __truediv__(
        self,
        scalar: float,
    ) -> Vec2:
        """Divides each component by a nonzero scalar."""
        return Vec2(
            self.x / scalar,
            self.y / scalar,
        )

    def __eq__(
        self,
        other: object,
    ) -> bool:
        """Tests exact component equality; returns NotImplemented for an unrelated type."""
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
        """Returns a representation containing the component values."""
        return (
            f"Vec2({self.x}, {self.y})"
        )


class Vec3(ctypes.Structure):
    """Three-component floating-point vector for positions, directions, and scales.

    Attributes:
        x: X component.
        y: Y component.
        z: Z component."""
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
        """Creates a vector from its X, Y, and Z components."""
        super().__init__(
            float(x),
            float(y),
            float(z),
        )

    @property
    def length(self) -> float:
        """Euclidean magnitude of the components."""
        return _math.sqrt(
            self.x * self.x
            + self.y * self.y
            + self.z * self.z
        )

    def normalized(self) -> Vec3:
        """Returns a unit-length copy; a zero-length vector remains zero."""
        length = self.length

        if length == 0.0:
            return Vec3()

        return Vec3(
            self.x / length,
            self.y / length,
            self.z / length,
        )

    def normalize(self) -> None:
        """Normalizes the vector in place; leaves a zero-length vector unchanged."""
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
        """Returns the scalar dot product with the other vector."""
        return (
            self.x * other.x
            + self.y * other.y
            + self.z * other.z
        )

    def cross(
        self,
        other: Vec3,
    ) -> Vec3:
        """Returns the right-handed cross product with the other vector."""
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
        """Adds corresponding vector components."""
        return Vec3(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
        )

    def __sub__(
        self,
        other: Vec3,
    ) -> Vec3:
        """Subtracts corresponding vector components."""
        return Vec3(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
        )

    def __mul__(
        self,
        scalar: float,
    ) -> Vec3:
        """Multiplies each component by a scalar."""
        return Vec3(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
        )

    def __truediv__(
        self,
        scalar: float,
    ) -> Vec3:
        """Divides each component by a nonzero scalar."""
        return Vec3(
            self.x / scalar,
            self.y / scalar,
            self.z / scalar,
        )

    def __eq__(
        self,
        other: object,
    ) -> bool:
        """Tests exact component equality; returns NotImplemented for an unrelated type."""
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
        """Returns a representation containing the component values."""
        return (
            f"Vec3("
            f"{self.x}, "
            f"{self.y}, "
            f"{self.z}"
            f")"
        )


class Vec4(ctypes.Structure):
    """Four-component floating-point vector; W distinguishes homogeneous positions from directions.

    Attributes:
        x: X component.
        y: Y component.
        z: Z component.
        w: W component."""
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
        """Creates a vector from its X, Y, Z, and W components."""
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
        """Creates homogeneous coordinates from XYZ with W = 1."""
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
        """Creates homogeneous direction coordinates from XYZ with W = 0."""
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
        """Adds corresponding vector components."""
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
        """Subtracts corresponding vector components."""
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
        """Multiplies each component by a scalar."""
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
        """Divides each component by a nonzero scalar."""
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
        """Tests exact component equality; returns NotImplemented for an unrelated type."""
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
        """Returns a representation containing the component values."""
        return (
            f"Vec4("
            f"{self.x}, "
            f"{self.y}, "
            f"{self.z}, "
            f"{self.w}"
            f")"
        )


class Quaternion(ctypes.Structure):
    """Rotation quaternion with XYZW components; normalize manually constructed values before rotating vectors.

    Attributes:
        x: X component.
        y: Y component.
        z: Z component.
        w: W component."""
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
        """Creates a quaternion from XYZW components without normalizing it."""
        super().__init__(
            float(x),
            float(y),
            float(z),
            float(w),
        )

    @classmethod
    def identity(cls) -> Quaternion:
        """Rotation quaternion (0, 0, 0, 1), which leaves vectors unchanged."""
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
        """Creates a rotation about a nonzero axis, normalizing the axis before use."""
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
        """Constructs yaw * pitch * roll about Y, X, and Z; the rightmost rotation acts first on a vector."""
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
        """Returns the shortest rotation aligning two nonzero directions; vector lengths are ignored."""
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
        """Euclidean magnitude of the components."""
        return _math.sqrt(
            self.x * self.x
            + self.y * self.y
            + self.z * self.z
            + self.w * self.w
        )

    def normalized(self) -> Quaternion:
        """Returns a unit-length quaternion; a zero quaternion becomes the identity."""
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
        """Returns the quaternion with negated XYZ components; the inverse for a unit quaternion."""
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
        """Rotates a vector by this quaternion; use a unit quaternion for a pure rotation."""
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
        """Composes two rotations; the right-hand quaternion acts first on a vector."""
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
        """Tests exact component equality; returns NotImplemented for an unrelated type."""
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
        """Returns a representation containing the component values."""
        return (
            f"Quaternion("
            f"{self.x}, "
            f"{self.y}, "
            f"{self.z}, "
            f"{self.w}"
            f")"
        )


class Rect(ctypes.Structure):
    """Pixel rectangle with top-left X/Y coordinates and width/height extents.

    Attributes:
        x: Left edge in physical pixels.
        y: Top edge in physical pixels.
        width: Horizontal extent in physical pixels.
        height: Vertical extent in physical pixels."""
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
        """Creates a pixel rectangle from top-left coordinates and width/height extents."""
        super().__init__(
            float(x),
            float(y),
            float(width),
            float(height),
        )

    @property
    def aspect_ratio(self) -> float:
        """Width divided by height; height must be nonzero for a finite result."""
        return (
            self.width
            / self.height
        )

    def __eq__(
        self,
        other: object,
    ) -> bool:
        """Tests exact component equality; returns NotImplemented for an unrelated type."""
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
        """Returns a representation containing the component values."""
        return (
            f"Rect("
            f"{self.x}, "
            f"{self.y}, "
            f"{self.width}, "
            f"{self.height}"
            f")"
        )


class Transform(ctypes.Structure):
    """World-space position, quaternion orientation, and component-wise local scale. A default constructed transform has unit scale.

    Attributes:
        position: Position in world space.
        rotation: Orientation quaternion; assigned values are normalized.
        scale: Component-wise scale along the model's local axes."""
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
        """Creates a transform at the origin with identity rotation and unit scale."""
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
        """Assigns a field, applying this type's normalization and validation rules."""
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
        """Adds a world-space translation without rotating it by the current orientation."""
        self.position = (
            self.position
            + translation
        )

    def set_rotation(
        self,
        rotation: Quaternion,
    ) -> None:
        """Sets the orientation to a normalized copy of the quaternion."""
        self.rotation = (
            rotation.normalized()
        )

    def set_euler(
        self,
        pitch: Angle,
        yaw: Angle,
        roll: Angle,
    ) -> None:
        """Sets orientation to yaw * pitch * roll, about Y, X, and Z respectively."""
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
        """Applies an additional rotation about the world X axis."""
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
        """Applies an additional rotation about the world Y axis."""
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
        """Applies an additional rotation about the world Z axis."""
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
        """Multiplies the current scale component by component."""
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
        """Returns a child transform aligning model-space attachment points using a world-space direction, twist angle, and child scale."""
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
        """Tests exact component equality; returns NotImplemented for an unrelated type."""
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
        """Returns a representation containing the component values."""
        return (
            "Transform("
            f"position={self.position!r}, "
            f"rotation={self.rotation!r}, "
            f"scale={self.scale!r}"
            ")"
        )
