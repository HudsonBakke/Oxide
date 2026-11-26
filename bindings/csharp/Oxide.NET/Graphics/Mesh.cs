using System;

using Oxide.Math;
using Oxide.Content;
using Oxide.Native;

namespace Oxide.Graphics;

/// <summary>Indexed triangle geometry with model-space pivot and attachment orientation; owns a native resource in language bindings.</summary>
/// <remarks>Owns a native handle released by a SafeHandle finalizer. Native failures raise OxideException.</remarks>
public sealed class Mesh : IContentAsset
{
    private readonly MeshSafeHandle _handle;

    /// <summary>Copies vertices and triangle indices into a new mesh and computes its pivot and tangent frame.</summary>
    public unsafe Mesh(Vertex[] vertices, uint[] indices)
    {
        ArgumentNullException.ThrowIfNull(vertices);

        ArgumentNullException.ThrowIfNull(indices);

        fixed(Vertex *vertexPointer = vertices) fixed(uint *indexPointer = indices)
        {
            NativeMethods.Check(NativeMethods.oxide_mesh_create(vertexPointer,
                                                                (nuint)vertices.Length,

                                                                indexPointer, (nuint)indices.Length,

                                                                out NativeMeshHandle handle));

            _handle = new MeshSafeHandle(handle);
        }
    }

    internal Mesh(NativeMeshHandle handle)
    {
        _handle = new MeshSafeHandle(handle);
    }

    internal NativeMeshHandle Handle
    {
        get {
            if (_handle.IsInvalid || _handle.IsClosed)
            {
                throw new ObjectDisposedException(nameof(Mesh));
            }

            return _handle.NativeHandle;
        }
    }
    /// <summary>Number of vertices in the mesh.</summary>
    public int VertexCount
    {
        get {
            NativeMethods.Check(NativeMethods.oxide_mesh_vertex_count(Handle, out nuint count));

            return checked((int)count);
        }
    }
    /// <summary>Number of vertex indices; each group of three describes a triangle.</summary>
    public int IndexCount
    {
        get {
            NativeMethods.Check(NativeMethods.oxide_mesh_index_count(Handle, out nuint count));

            return checked((int)count);
        }
    }
    /// <summary>Model-space point placed at the transform's position when drawing or attaching this mesh.</summary>
    public Vec3 Pivot
    {
        get {
            NativeMethods.Check(NativeMethods.oxide_mesh_get_pivot(Handle, out Vec3 result));

            return result;
        }

        set {
            NativeMethods.Check(NativeMethods.oxide_mesh_set_pivot(Handle, value));
        }
    }
    /// <summary>Normalized forward direction of the mesh's model-space attachment frame.</summary>
    public Vec3 Direction
    {
        get {
            NativeMethods.Check(NativeMethods.oxide_mesh_get_direction(Handle, out Vec3 result));

            return result;
        }

        set {
            NativeMethods.Check(NativeMethods.oxide_mesh_set_direction(Handle, value));
        }
    }
    /// <summary>Normalized model-space reference axis perpendicular to the attachment direction.</summary>
    public Vec3 Reference
    {
        get {
            NativeMethods.Check(NativeMethods.oxide_mesh_get_reference(Handle, out Vec3 result));

            return result;
        }

        set {
            NativeMethods.Check(NativeMethods.oxide_mesh_set_reference(Handle, value));
        }
    }
    /// <summary>Normalized side axis computed from direction cross reference.</summary>
    public Vec3 Side
    {
        get {
            NativeMethods.Check(NativeMethods.oxide_mesh_get_side(Handle, out Vec3 result));

            return result;
        }
    }

    /// <summary>Builds an orthonormal model-space attachment frame from nonzero direction and reference vectors.</summary>
    public void SetOrientation(Vec3 direction, Vec3 reference)
    {
        NativeMethods.Check(NativeMethods.oxide_mesh_set_orientation(Handle, direction, reference));
    }

    /// <summary>Sets the pivot to the arithmetic mean of all model-space vertex positions.</summary>
    public void RecalculatePivot()
    {
        NativeMethods.Check(NativeMethods.oxide_mesh_recalculate_pivot(Handle));
    }

    /// <summary>Returns a copy of the mesh's vertices; submit edits with set_vertices.</summary>
    public unsafe Vertex[] GetVertices()
    {
        int count = VertexCount;

        var vertices = new Vertex[count];

        if (count == 0)
        {
            return vertices;
        }

        fixed(Vertex *pointer = vertices)
        {
            NativeMethods.Check(
                NativeMethods.oxide_mesh_copy_vertices(Handle, pointer, (nuint)count));
        }

        return vertices;
    }

    /// <summary>Returns a copy of the mesh's triangle indices; submit edits with set_indices.</summary>
    public unsafe uint[] GetIndices()
    {
        int count = IndexCount;

        var indices = new uint[count];

        if (count == 0)
        {
            return indices;
        }

        fixed(uint *pointer = indices)
        {
            NativeMethods.Check(
                NativeMethods.oxide_mesh_copy_indices(Handle, pointer, (nuint)count));
        }

        return indices;
    }

    /// <summary>Copies replacement vertex data into the mesh; the vertex count must remain unchanged.</summary>
    public unsafe void SetVertices(Vertex[] vertices)
    {
        ArgumentNullException.ThrowIfNull(vertices);

        fixed(Vertex *pointer = vertices)
        {
            NativeMethods.Check(
                NativeMethods.oxide_mesh_set_vertices(Handle, pointer, (nuint)vertices.Length));
        }
    }

    /// <summary>Copies replacement triangle indices into the mesh; the index count must remain unchanged.</summary>
    public unsafe void SetIndices(uint[] indices)
    {
        ArgumentNullException.ThrowIfNull(indices);

        fixed(uint *pointer = indices)
        {
            NativeMethods.Check(
                NativeMethods.oxide_mesh_set_indices(Handle, pointer, (nuint)indices.Length));
        }
    }


}
