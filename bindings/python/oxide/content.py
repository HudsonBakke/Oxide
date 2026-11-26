from __future__ import annotations

import ctypes
import weakref
from os import PathLike
from typing import TypeVar

from . import _native
from .math import Vec2, Vec3, Vec4


class Vertex(ctypes.Structure):
    """Vertex containing position, normal, texture coordinates, and tangent."""
    _fields_ = [
        ("position", Vec3),
        ("normal", Vec3),
        ("tex_coord", Vec2),
        ("tangent", Vec4),
    ]

    def __init__(
        self,
        position: Vec3,
        normal: Vec3,
        tex_coord: Vec2,
        tangent: Vec4 | None = None,
    ) -> None:
        super().__init__(
            position,
            normal,
            tex_coord,
            tangent if tangent is not None else Vec4(0.0, 0.0, 0.0, 1.0),
        )

    def __eq__(
        self,
        other: object,
    ) -> bool:
        if not isinstance(other, Vertex):
            return NotImplemented

        return (
            self.position == other.position
            and self.normal == other.normal
            and self.tex_coord == other.tex_coord
            and self.tangent == other.tangent
        )


if ctypes.sizeof(Vertex) != 48:
    raise RuntimeError(
        f"Oxide Python Vertex ABI mismatch: expected 48 bytes, "
        f"got {ctypes.sizeof(Vertex)}"
    )


class _OwnedResource:
    _destroy_function = None

    @classmethod
    def _from_handle(cls, handle):
        instance = cls.__new__(cls)
        instance._attach_handle(handle)
        return instance

    def _attach_handle(self, handle) -> None:
        self._handle = handle
        self._finalizer = weakref.finalize(
            self,
            type(self)._destroy,
            handle,
        )

    @classmethod
    def _destroy(cls, handle) -> None:
        destroy = cls._destroy_function

        if destroy is not None:
            destroy(handle)


class Mesh(_OwnedResource):
    """Indexed triangle mesh with mutable geometry and orientation metadata."""
    _destroy_function = (
        _native.lib.oxide_mesh_destroy
    )

    def __init__(
        self,
        vertices: list[Vertex],
        indices: list[int],
    ) -> None:
        vertex_array_type = (
            Vertex * len(vertices)
        )

        index_array_type = (
            ctypes.c_uint32 * len(indices)
        )

        vertex_array = vertex_array_type(
            *vertices
        )

        index_array = index_array_type(
            *indices
        )

        handle = _native.NativeMeshHandle()

        _native.check(
            _native.lib.oxide_mesh_create(
                vertex_array,
                len(vertices),
                index_array,
                len(indices),
                ctypes.byref(handle),
            )
        )

        self._attach_handle(handle)

    @property
    def vertex_count(self) -> int:
        """Public Oxide vertex count operation."""
        result = ctypes.c_size_t()

        _native.check(
            _native.lib.oxide_mesh_vertex_count(
                self._handle,
                ctypes.byref(result),
            )
        )

        return result.value

    @property
    def index_count(self) -> int:
        """Public Oxide index count operation."""
        result = ctypes.c_size_t()

        _native.check(
            _native.lib.oxide_mesh_index_count(
                self._handle,
                ctypes.byref(result),
            )
        )

        return result.value

    @property
    def pivot(self) -> Vec3:
        """Public Oxide pivot operation."""
        result = Vec3()

        _native.check(
            _native.lib.oxide_mesh_get_pivot(
                self._handle,
                ctypes.byref(result),
            )
        )

        return result

    @pivot.setter
    def pivot(self, value: Vec3) -> None:
        """Public Oxide pivot operation."""
        _native.check(
            _native.lib.oxide_mesh_set_pivot(
                self._handle,
                value,
            )
        )

    def recalculate_pivot(self) -> None:
        """Public Oxide recalculate pivot operation."""
        _native.check(
            _native.lib.oxide_mesh_recalculate_pivot(
                self._handle
            )
        )

    @property
    def direction(self) -> Vec3:
        """Public Oxide direction operation."""
        result = Vec3()

        _native.check(
            _native.lib.oxide_mesh_get_direction(
                self._handle,
                ctypes.byref(result),
            )
        )

        return result

    @direction.setter
    def direction(self, value: Vec3) -> None:
        """Public Oxide direction operation."""
        _native.check(
            _native.lib.oxide_mesh_set_direction(
                self._handle,
                value,
            )
        )

    @property
    def reference(self) -> Vec3:
        """Public Oxide reference operation."""
        result = Vec3()

        _native.check(
            _native.lib.oxide_mesh_get_reference(
                self._handle,
                ctypes.byref(result),
            )
        )

        return result

    @reference.setter
    def reference(self, value: Vec3) -> None:
        """Public Oxide reference operation."""
        _native.check(
            _native.lib.oxide_mesh_set_reference(
                self._handle,
                value,
            )
        )

    @property
    def side(self) -> Vec3:
        """Public Oxide side operation."""
        result = Vec3()

        _native.check(
            _native.lib.oxide_mesh_get_side(
                self._handle,
                ctypes.byref(result),
            )
        )

        return result

    def set_orientation(
        self,
        direction: Vec3,
        reference: Vec3,
    ) -> None:
        """Public Oxide set orientation operation."""
        _native.check(
            _native.lib.oxide_mesh_set_orientation(
                self._handle,
                direction,
                reference,
            )
        )

    def get_vertices(self) -> list[Vertex]:
        """Public Oxide get vertices operation."""
        count = self.vertex_count

        if count == 0:
            return []

        array_type = Vertex * count
        values = array_type()

        _native.check(
            _native.lib.oxide_mesh_copy_vertices(
                self._handle,
                values,
                count,
            )
        )

        return list(values)

    def set_vertices(
        self,
        vertices: list[Vertex],
    ) -> None:
        """Public Oxide set vertices operation."""
        array_type = Vertex * len(vertices)
        values = array_type(*vertices)

        _native.check(
            _native.lib.oxide_mesh_set_vertices(
                self._handle,
                values,
                len(vertices),
            )
        )

    def get_indices(self) -> list[int]:
        """Public Oxide get indices operation."""
        count = self.index_count

        if count == 0:
            return []

        array_type = ctypes.c_uint32 * count
        values = array_type()

        _native.check(
            _native.lib.oxide_mesh_copy_indices(
                self._handle,
                values,
                count,
            )
        )

        return list(values)

    def set_indices(
        self,
        indices: list[int],
    ) -> None:
        """Public Oxide set indices operation."""
        array_type = (
            ctypes.c_uint32 * len(indices)
        )

        values = array_type(*indices)

        _native.check(
            _native.lib.oxide_mesh_set_indices(
                self._handle,
                values,
                len(indices),
            )
        )


class Texture(_OwnedResource):
    """Immutable 2D texture resource loaded from content or created procedurally."""
    _destroy_function = (
        _native.lib.oxide_texture_destroy
    )

    @classmethod
    def from_rgba(
        cls,
        width: int,
        height: int,
        pixels: bytes | bytearray,
    ) -> Texture:
        """Public Oxide from rgba operation."""
        data = bytes(pixels)
        array_type = ctypes.c_uint8 * len(data)
        array = array_type.from_buffer_copy(data)
        handle = _native.NativeTextureHandle()
        _native.check(
            _native.lib.oxide_texture_create_rgba(
                width,
                height,
                array,
                len(data),
                ctypes.byref(handle),
            )
        )
        return cls._from_handle(handle)

    @classmethod
    def solid(cls, color) -> Texture:
        """Public Oxide solid operation."""
        # Imported lazily to avoid graphics.py -> resources.py
        # import cycles during module initialization.
        from .graphics import Color

        if not isinstance(color, Color):
            raise TypeError(
                "color must be a Color"
            )

        handle = _native.NativeTextureHandle()

        _native.check(
            _native.lib.oxide_texture_solid(
                color,
                ctypes.byref(handle),
            )
        )

        return cls._from_handle(handle)

    @property
    def width(self) -> int:
        """Public Oxide width operation."""
        result = ctypes.c_uint32()

        _native.check(
            _native.lib.oxide_texture_width(
                self._handle,
                ctypes.byref(result),
            )
        )

        return result.value

    @property
    def height(self) -> int:
        """Public Oxide height operation."""
        result = ctypes.c_uint32()

        _native.check(
            _native.lib.oxide_texture_height(
                self._handle,
                ctypes.byref(result),
            )
        )

        return result.value


class Material(_OwnedResource):
    """Physically based surface material."""
    _destroy_function = (
        _native.lib.oxide_material_destroy
    )

    def __init__(
        self,
        base_texture: Texture,
    ) -> None:
        if not isinstance(
            base_texture,
            Texture,
        ):
            raise TypeError(
                "base_texture must be a Texture"
            )

        handle = (
            _native.NativeMaterialHandle()
        )

        _native.check(
            _native.lib.oxide_material_create(
                base_texture._handle,
                ctypes.byref(handle),
            )
        )

        self._attach_handle(handle)

    @classmethod
    def solid(cls, color) -> Material:
        """Public Oxide solid operation."""
        from .graphics import Color

        if not isinstance(color, Color):
            raise TypeError(
                "color must be a Color"
            )

        handle = (
            _native.NativeMaterialHandle()
        )

        _native.check(
            _native.lib.oxide_material_solid(
                color,
                ctypes.byref(handle),
            )
        )

        return cls._from_handle(handle)


    def _get_float(self, name: str) -> float:
        result = ctypes.c_float()
        _native.check(
            getattr(_native.lib, f"oxide_material_get_{name}")(
                self._handle, ctypes.byref(result)
            )
        )
        return float(result.value)

    def _set_float(self, name: str, value: float) -> None:
        _native.check(
            getattr(_native.lib, f"oxide_material_set_{name}")(
                self._handle, float(value)
            )
        )

    @property
    def metallic(self) -> float:
        """Public Oxide metallic operation."""
        return self._get_float("metallic")

    @metallic.setter
    def metallic(self, value: float) -> None:
        """Public Oxide metallic operation."""
        self._set_float("metallic", value)

    @property
    def roughness(self) -> float:
        """Public Oxide roughness operation."""
        return self._get_float("roughness")

    @roughness.setter
    def roughness(self, value: float) -> None:
        """Public Oxide roughness operation."""
        self._set_float("roughness", value)

    @property
    def emissive_strength(self) -> float:
        """Public Oxide emissive strength operation."""
        return self._get_float("emissive_strength")

    @emissive_strength.setter
    def emissive_strength(self, value: float) -> None:
        """Public Oxide emissive strength operation."""
        self._set_float("emissive_strength", value)

    def _get_color(self, name: str):
        from .graphics import Color
        result = Color()
        _native.check(
            getattr(_native.lib, f"oxide_material_get_{name}")(
                self._handle, ctypes.byref(result)
            )
        )
        return result

    def _set_color(self, name: str, value) -> None:
        from .graphics import Color
        if not isinstance(value, Color):
            raise TypeError("value must be a Color")
        _native.check(
            getattr(_native.lib, f"oxide_material_set_{name}")(
                self._handle, value
            )
        )

    @property
    def base_color(self):
        """Public Oxide base color operation."""
        return self._get_color("base_color")

    @base_color.setter
    def base_color(self, value) -> None:
        """Public Oxide base color operation."""
        self._set_color("base_color", value)

    @property
    def emissive_color(self):
        """Public Oxide emissive color operation."""
        return self._get_color("emissive_color")

    @emissive_color.setter
    def emissive_color(self, value) -> None:
        """Public Oxide emissive color operation."""
        self._set_color("emissive_color", value)

    @property
    def base_texture(self) -> Texture:
        """Public Oxide base texture operation."""
        handle = (
            _native.NativeTextureHandle()
        )

        _native.check(
            _native.lib.oxide_material_base_texture(
                self._handle,
                ctypes.byref(handle),
            )
        )

        return Texture._from_handle(handle)


ContentType = TypeVar(
    "ContentType",
    Mesh,
    Material,
    Texture,
)


class ContentManager(_OwnedResource):
    """Loads source assets and manages Oxide's compiled-content cache."""
    _destroy_function = (
        _native.lib
        .oxide_content_manager_destroy
    )

    def __init__(
        self,
        cache_directory: (
            str | PathLike[str] | None
        ) = None,
    ) -> None:
        handle = (
            _native.NativeContentManagerHandle()
        )

        if cache_directory is None:
            status = (
                _native.lib
                .oxide_content_manager_create(
                    ctypes.byref(handle)
                )
            )
        else:
            path = str(
                cache_directory
            ).encode("utf-8")

            status = (
                _native.lib
                .oxide_content_manager_create_with_cache_directory(
                    path,
                    ctypes.byref(handle),
                )
            )

        _native.check(status)
        self._attach_handle(handle)

    @property
    def cache_directory(self) -> str:
        """Public Oxide cache directory operation."""
        length = ctypes.c_size_t()

        _native.check(
            _native.lib
            .oxide_content_manager_cache_directory_length(
                self._handle,
                ctypes.byref(length),
            )
        )

        buffer = ctypes.create_string_buffer(
            length.value + 1
        )

        _native.check(
            _native.lib
            .oxide_content_manager_copy_cache_directory(
                self._handle,
                buffer,
                len(buffer),
            )
        )

        return buffer.value.decode(
            "utf-8"
        )

    def load(
        self,
        asset_type: type[ContentType],
        path: str | PathLike[str],
        subobject: str | None = None,
    ) -> ContentType:
        """Load a Mesh, Material, or Texture from the content pipeline."""
        encoded_path = str(path).encode(
            "utf-8"
        )

        if asset_type is Mesh:
            handle = _native.NativeMeshHandle()
            default_fn = (
                _native.lib.oxide_content_load_mesh
            )
            sub_fn = (
                _native.lib.oxide_content_load_mesh_sub
            )
        elif asset_type is Material:
            handle = (
                _native.NativeMaterialHandle()
            )
            default_fn = (
                _native.lib.oxide_content_load_material
            )
            sub_fn = (
                _native.lib.oxide_content_load_material_sub
            )
        elif asset_type is Texture:
            handle = (
                _native.NativeTextureHandle()
            )
            default_fn = (
                _native.lib.oxide_content_load_texture
            )
            sub_fn = (
                _native.lib.oxide_content_load_texture_sub
            )
        else:
            raise TypeError(
                "asset_type must be Mesh, Material, or Texture"
            )

        if subobject is None or subobject == "":
            status = default_fn(
                self._handle,
                encoded_path,
                ctypes.byref(handle),
            )
        else:
            status = sub_fn(
                self._handle,
                encoded_path,
                subobject.encode("utf-8"),
                ctypes.byref(handle),
            )

        _native.check(status)

        return asset_type._from_handle(
            handle
        )


# Signatures that depend on Vertex are declared here to avoid making
# _native.py depend on this public module.
_native.lib.oxide_mesh_create.argtypes = [
    ctypes.POINTER(Vertex), ctypes.c_size_t,
    ctypes.POINTER(ctypes.c_uint32), ctypes.c_size_t,
    ctypes.POINTER(_native.NativeMeshHandle),
]
_native.lib.oxide_mesh_create.restype = ctypes.c_int
_native.lib.oxide_mesh_copy_vertices.argtypes = [
    _native.NativeMeshHandle, ctypes.POINTER(Vertex), ctypes.c_size_t,
]
_native.lib.oxide_mesh_copy_vertices.restype = ctypes.c_int
_native.lib.oxide_mesh_set_vertices.argtypes = [
    _native.NativeMeshHandle, ctypes.POINTER(Vertex), ctypes.c_size_t,
]
_native.lib.oxide_mesh_set_vertices.restype = ctypes.c_int
_native.lib.oxide_mesh_copy_indices.argtypes = [
    _native.NativeMeshHandle, ctypes.POINTER(ctypes.c_uint32), ctypes.c_size_t,
]
_native.lib.oxide_mesh_copy_indices.restype = ctypes.c_int
_native.lib.oxide_mesh_set_indices.argtypes = [
    _native.NativeMeshHandle, ctypes.POINTER(ctypes.c_uint32), ctypes.c_size_t,
]
_native.lib.oxide_mesh_set_indices.restype = ctypes.c_int
