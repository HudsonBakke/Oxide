using System.Runtime.InteropServices;

using Oxide.Math;

namespace Oxide.Graphics;

[StructLayout(LayoutKind.Sequential)]
/// <summary>Model-space vertex with position, normal, UV coordinates, and tangent XYZ plus handedness W.</summary>
public struct Vertex : IEquatable<Vertex>
{
    /// <summary>Vertex position in model space.</summary>
    public Vec3 Position;
    /// <summary>Model-space surface normal used for lighting.</summary>
    public Vec3 Normal;
    /// <summary>Texture UV coordinates; (0, 0) addresses the top-left corner.</summary>
    public Vec2 TexCoord;
    /// <summary>Model-space tangent XYZ and bitangent handedness W (normally +1 or -1).</summary>
    public Vec4 Tangent;

    /// <summary>Creates a vertex; an omitted tangent is generated when the mesh is constructed.</summary>
    public Vertex(Vec3 position, Vec3 normal, Vec2 texCoord)
    {
        Position = position;
        Normal = normal;
        TexCoord = texCoord;
        Tangent = new Vec4(0.0f, 0.0f, 0.0f, 1.0f);
    }

    /// <summary>Creates a vertex; an omitted tangent is generated when the mesh is constructed.</summary>
    public Vertex(Vec3 position, Vec3 normal, Vec2 texCoord, Vec4 tangent)
    {
        Position = position;
        Normal = normal;
        TexCoord = texCoord;
        Tangent = tangent;
    }

    /// <summary>Tests exact component equality; no floating-point tolerance is applied.</summary>
    public readonly bool Equals(Vertex other)
    {
        return Position == other.Position && Normal == other.Normal && TexCoord == other.TexCoord &&
               Tangent == other.Tangent;
    }

    /// <summary>Tests exact component equality; no floating-point tolerance is applied.</summary>
    public override readonly bool Equals(object? obj)
    {
        return obj is Vertex other && Equals(other);
    }

    /// <summary>Returns a hash consistent with exact component equality.</summary>
    public override readonly int GetHashCode()
    {
        return HashCode.Combine(Position, Normal, TexCoord, Tangent);
    }

    /// <summary>Tests exact component equality.</summary>
    public static bool operator ==(Vertex left, Vertex right) => left.Equals(right);

    /// <summary>Tests exact component inequality.</summary>
    public static bool operator !=(Vertex left, Vertex right) => !left.Equals(right);
}
