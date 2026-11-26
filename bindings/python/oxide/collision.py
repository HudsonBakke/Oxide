from __future__ import annotations

import ctypes

from . import _native
from .content import Mesh
from .math import Transform


def collides(
    mesh_a: Mesh,
    transform_a: Transform,
    mesh_b: Mesh,
    transform_b: Transform,
) -> bool:
    """Return whether the two transformed meshes intersect."""
    if not isinstance(mesh_a, Mesh):
        raise TypeError("mesh_a must be a Mesh")

    if not isinstance(mesh_b, Mesh):
        raise TypeError("mesh_b must be a Mesh")

    result = ctypes.c_uint8()

    _native.check(
        _native.lib.oxide_collision_collides(
            mesh_a._handle,
            transform_a,
            mesh_b._handle,
            transform_b,
            ctypes.byref(result),
        )
    )

    return result.value != 0
