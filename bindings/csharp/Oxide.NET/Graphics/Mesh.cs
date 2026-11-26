using System;

using Oxide.Math;
using Oxide.Content;
using Oxide.Native;

namespace Oxide.Graphics;

/// <summary>Indexed triangle mesh containing geometry and orientation metadata.</summary>
public sealed class Mesh : IContentAsset
{
    private readonly MeshSafeHandle
        _handle;

    internal Mesh(
        NativeMeshHandle handle
    )
    {
        _handle =
            new MeshSafeHandle(
                handle
            );
    }

    internal NativeMeshHandle Handle
    {
        get
        {
            if (_handle.IsInvalid
                || _handle.IsClosed)
            {
                throw new ObjectDisposedException(
                    nameof(Mesh)
                );
            }

            return _handle.NativeHandle;
        }
    }

    public int VertexCount
    {
        get
        {
            NativeMethods.Check(
                NativeMethods
                    .oxide_mesh_vertex_count(
                        Handle,
                        out nuint count
                    )
            );

            return checked(
                (int)count
            );
        }
    }

    public int IndexCount
    {
        get
        {
            NativeMethods.Check(
                NativeMethods
                    .oxide_mesh_index_count(
                        Handle,
                        out nuint count
                    )
            );

            return checked(
                (int)count
            );
        }
    }

    public Vec3 Pivot
    {
        get
        {
            NativeMethods.Check(
                NativeMethods
                    .oxide_mesh_get_pivot(
                        Handle,
                        out Vec3 result
                    )
            );

            return result;
        }

        set
        {
            NativeMethods.Check(
                NativeMethods
                    .oxide_mesh_set_pivot(
                        Handle,
                        value
                    )
            );
        }
    }

    public Vec3 Direction
    {
        get
        {
            NativeMethods.Check(
                NativeMethods
                    .oxide_mesh_get_direction(
                        Handle,
                        out Vec3 result
                    )
            );

            return result;
        }

        set
        {
            NativeMethods.Check(
                NativeMethods
                    .oxide_mesh_set_direction(
                        Handle,
                        value
                    )
            );
        }
    }

    public Vec3 Reference
    {
        get
        {
            NativeMethods.Check(
                NativeMethods
                    .oxide_mesh_get_reference(
                        Handle,
                        out Vec3 result
                    )
            );

            return result;
        }

        set
        {
            NativeMethods.Check(
                NativeMethods
                    .oxide_mesh_set_reference(
                        Handle,
                        value
                    )
            );
        }
    }

    public Vec3 Side
    {
        get
        {
            NativeMethods.Check(
                NativeMethods
                    .oxide_mesh_get_side(
                        Handle,
                        out Vec3 result
                    )
            );

            return result;
        }
    }

        /// <summary>Sets the mesh orientation basis from direction and reference vectors.</summary>
public void SetOrientation(
        Vec3 direction,
        Vec3 reference
    )
    {
        NativeMethods.Check(
            NativeMethods
                .oxide_mesh_set_orientation(
                    Handle,
                    direction,
                    reference
                )
        );
    }

        /// <summary>Recomputes the mesh pivot from its current vertices.</summary>
public void RecalculatePivot()
    {
        NativeMethods.Check(
            NativeMethods
                .oxide_mesh_recalculate_pivot(
                    Handle
                )
        );
    }

        /// <summary>Returns a copy of the mesh vertices.</summary>
public unsafe Vertex[] GetVertices()
    {
        int count =
            VertexCount;

        var vertices =
            new Vertex[count];

        if (count == 0)
        {
            return vertices;
        }

        fixed (Vertex* pointer =
            vertices)
        {
            NativeMethods.Check(
                NativeMethods
                    .oxide_mesh_copy_vertices(
                        Handle,
                        pointer,
                        (nuint)count
                    )
            );
        }

        return vertices;
    }

        /// <summary>Returns a copy of the mesh indices.</summary>
public unsafe uint[] GetIndices()
    {
        int count =
            IndexCount;

        var indices =
            new uint[count];

        if (count == 0)
        {
            return indices;
        }

        fixed (uint* pointer =
            indices)
        {
            NativeMethods.Check(
                NativeMethods
                    .oxide_mesh_copy_indices(
                        Handle,
                        pointer,
                        (nuint)count
                    )
            );
        }

        return indices;
    }

    public unsafe void SetVertices(
        Vertex[] vertices
    )
    {
        ArgumentNullException.ThrowIfNull(
            vertices
        );

        fixed (Vertex* pointer =
            vertices)
        {
            NativeMethods.Check(
                NativeMethods
                    .oxide_mesh_set_vertices(
                        Handle,
                        pointer,
                        (nuint)vertices.Length
                    )
            );
        }
    }

    public unsafe void SetIndices(
        uint[] indices
    )
    {
        ArgumentNullException.ThrowIfNull(
            indices
        );

        fixed (uint* pointer =
            indices)
        {
            NativeMethods.Check(
                NativeMethods
                    .oxide_mesh_set_indices(
                        Handle,
                        pointer,
                        (nuint)indices.Length
                    )
            );
        }
    }

    public unsafe Mesh(
        Vertex[] vertices,
        uint[] indices
    )
    {
        ArgumentNullException.ThrowIfNull(
            vertices
        );

        ArgumentNullException.ThrowIfNull(
            indices
        );

        fixed (Vertex* vertexPointer =
            vertices)
        fixed (uint* indexPointer =
            indices)
        {
            NativeMethods.Check(
                NativeMethods
                    .oxide_mesh_create(
                        vertexPointer,
                        (nuint)vertices.Length,

                        indexPointer,
                        (nuint)indices.Length,

                        out NativeMeshHandle handle
                    )
            );

            _handle =
                new MeshSafeHandle(
                    handle
                );
        }
    }
}