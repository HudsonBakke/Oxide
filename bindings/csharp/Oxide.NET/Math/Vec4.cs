using System;
using System.Runtime.InteropServices;

namespace Oxide.Math;
[StructLayout(LayoutKind.Sequential)]
/// <summary>Four-component floating-point vector; W distinguishes homogeneous positions from directions.</summary>
public struct Vec4
    : IEquatable<Vec4>
{
    /// <summary>X component.</summary>
    public float X;
    /// <summary>Y component.</summary>
    public float Y;
    /// <summary>Z component.</summary>
    public float Z;
    /// <summary>W component.</summary>
    public float W;
    /// <summary>Creates a vector from its X, Y, Z, and W components.</summary>
    public Vec4(
        float x,
        float y,
        float z,
        float w
    )
    {
        X = x;
        Y = y;
        Z = z;
        W = w;
    }
    /// <summary>Creates homogeneous coordinates from XYZ with W = 1.</summary>
    public static Vec4 FromPosition(
        Vec3 position
    )
    {
        return new Vec4(
            position.X,
            position.Y,
            position.Z,
            1.0f
        );
    }
    /// <summary>Creates homogeneous direction coordinates from XYZ with W = 0.</summary>
    public static Vec4 FromDirection(
        Vec3 direction
    )
    {
        return new Vec4(
            direction.X,
            direction.Y,
            direction.Z,
            0.0f
        );
    }
    /// <summary>Tests exact component equality; no floating-point tolerance is applied.</summary>
    public readonly bool Equals(
        Vec4 other
    )
    {
        return X == other.X
            && Y == other.Y
            && Z == other.Z
            && W == other.W;
    }

    /// <summary>Tests exact component equality; no floating-point tolerance is applied.</summary>
    public override readonly bool Equals(
        object? obj
    )
    {
        return obj is Vec4 other
            && Equals(other);
    }

    /// <summary>Returns a hash consistent with exact component equality.</summary>
    public override readonly int GetHashCode()
    {
        return HashCode.Combine(
            X,
            Y,
            Z,
            W
        );
    }

    /// <summary>Tests exact component equality.</summary>
    public static bool operator ==(
        Vec4 left,
        Vec4 right
    )
    {
        return left.Equals(
            right
        );
    }

    /// <summary>Tests exact component inequality.</summary>
    public static bool operator !=(
        Vec4 left,
        Vec4 right
    )
    {
        return !left.Equals(
            right
        );
    }

    /// <summary>Adds corresponding vector components.</summary>
    public static Vec4 operator +(
        Vec4 left,
        Vec4 right
    )
    {
        return new Vec4(
            left.X + right.X,
            left.Y + right.Y,
            left.Z + right.Z,
            left.W + right.W
        );
    }

    /// <summary>Subtracts vector components or negates a vector.</summary>
    public static Vec4 operator -(
        Vec4 left,
        Vec4 right
    )
    {
        return new Vec4(
            left.X - right.X,
            left.Y - right.Y,
            left.Z - right.Z,
            left.W - right.W
        );
    }

    /// <summary>Multiplies each component by a scalar.</summary>
    public static Vec4 operator *(
        Vec4 vector,
        float scalar
    )
    {
        return new Vec4(
            vector.X * scalar,
            vector.Y * scalar,
            vector.Z * scalar,
            vector.W * scalar
        );
    }

    /// <summary>Divides each component by a nonzero scalar.</summary>
    public static Vec4 operator /(
        Vec4 vector,
        float scalar
    )
    {
        return new Vec4(
            vector.X / scalar,
            vector.Y / scalar,
            vector.Z / scalar,
            vector.W / scalar
        );
    }
}
