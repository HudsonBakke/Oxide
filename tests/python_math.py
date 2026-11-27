"""Math regression smoke checks; exits unsuccessfully on a numerical mismatch."""
import math

from oxide.math import (
    Angle,
    Quaternion,
    Transform,
    Vec3,
)


a = Vec3(
    1.0,
    2.0,
    3.0,
)

b = Vec3(
    4.0,
    5.0,
    6.0,
)

print(
    "a + b:",
    a + b,
)

print(
    "dot:",
    a.dot(b),
)

print(
    "cross:",
    a.cross(b),
)

direction = Vec3(
    1.0,
    1.0,
    0.0,
).normalized()

print(
    "direction:",
    direction,
)

print(
    "direction length:",
    direction.length,
)

rotation = Quaternion.from_euler(
    Angle.degrees(0.0),
    Angle.degrees(90.0),
    Angle.degrees(0.0),
)

print(
    "rotation:",
    rotation,
)

print(
    "rotated forward:",
    rotation.rotate_vector(
        Vec3(
            0.0,
            0.0,
            -1.0,
        )
    ),
)

transform = Transform()

transform.position = Vec3(
    1.0,
    2.0,
    3.0,
)

transform.rotate_yaw_by(Angle.degrees(45.0))

transform.scale_by(
    Vec3(
        2.0,
        2.0,
        2.0,
    )
)

print(
    "transform:",
    transform,
)

assert a + b == Vec3(5.0, 7.0, 9.0)
assert a.dot(b) == 32.0
assert a.cross(b) == Vec3(-3.0, 6.0, -3.0)
assert math.isclose(direction.length, 1.0, abs_tol=1e-6)
forward = rotation.rotate_vector(Vec3(0.0, 0.0, -1.0))
assert math.isclose(forward.x, -1.0, abs_tol=1e-6)
assert math.isclose(forward.y, 0.0, abs_tol=1e-6)
assert math.isclose(forward.z, 0.0, abs_tol=1e-6)
assert transform.position == Vec3(1.0, 2.0, 3.0)
assert transform.scale == Vec3(2.0, 2.0, 2.0)
