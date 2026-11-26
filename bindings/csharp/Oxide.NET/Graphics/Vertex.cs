using System.Runtime.InteropServices;

using Oxide.Math;

namespace Oxide.Graphics;

[StructLayout(LayoutKind.Sequential)]
/// <summary>Vertex data containing position, normal, texture coordinates, and tangent.</summary>
public struct Vertex
    : IEquatable<Vertex>
{
    public Vec3 Position;
    public Vec3 Normal;
    public Vec2 TexCoord;
    public Vec4 Tangent;

    public Vertex(
        Vec3 position,
        Vec3 normal,
        Vec2 texCoord
    )
    {
        Position = position;
        Normal = normal;
        TexCoord = texCoord;
        Tangent = new Vec4(0.0f, 0.0f, 0.0f, 1.0f);
    }

    public Vertex(Vec3 position, Vec3 normal, Vec2 texCoord, Vec4 tangent)
    {
        Position = position;
        Normal = normal;
        TexCoord = texCoord;
        Tangent = tangent;
    }

    public readonly bool Equals(
        Vertex other
    )
    {
        return Position
                == other.Position
            && Normal
                == other.Normal
            && TexCoord
                == other.TexCoord
            && Tangent
                == other.Tangent;
    }

    public override readonly bool Equals(
        object? obj
    )
    {
        return obj
            is Vertex other
            && Equals(other);
    }

    public override readonly int GetHashCode()
    {
        return HashCode.Combine(
            Position,
            Normal,
            TexCoord,
            Tangent
        );
    }

    public static bool operator ==(
        Vertex left,
        Vertex right
    ) =>
        left.Equals(right);

    public static bool operator !=(
        Vertex left,
        Vertex right
    ) =>
        !left.Equals(right);
}