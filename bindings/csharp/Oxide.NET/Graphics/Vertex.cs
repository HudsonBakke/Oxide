using System.Runtime.InteropServices;

using Oxide.Math;

namespace Oxide.Graphics;

[StructLayout(LayoutKind.Sequential)]
public struct Vertex
    : IEquatable<Vertex>
{
    public Vec3 Position;
    public Vec3 Normal;
    public Vec2 TexCoord;

    public Vertex(
        Vec3 position,
        Vec3 normal,
        Vec2 texCoord
    )
    {
        Position = position;
        Normal = normal;
        TexCoord = texCoord;
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
                == other.TexCoord;
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
            TexCoord
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