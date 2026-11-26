using System;
using System.Runtime.InteropServices;

namespace Oxide.Math;

[StructLayout(LayoutKind.Sequential)]
/// <summary>Four-component floating-point vector.</summary>
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

    /// <summary>Creates or accesses Vec4.</summary>
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

    /// <summary>Creates homogeneous coordinates for a position with W = 1.</summary>
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

    /// <summary>Creates homogeneous coordinates for a direction with W = 0.</summary>
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

    /// <summary>Tests value equality.</summary>
    public readonly bool Equals(
        Vec4 other
    )
    {
        return X == other.X
            && Y == other.Y
            && Z == other.Z
            && W == other.W;
    }

    public override readonly bool Equals(
        object? obj
    )
    {
        return obj is Vec4 other
            && Equals(other);
    }

    public override readonly int GetHashCode()
    {
        return HashCode.Combine(
            X,
            Y,
            Z,
            W
        );
    }

    public static bool operator ==(
        Vec4 left,
        Vec4 right
    )
    {
        return left.Equals(
            right
        );
    }

    public static bool operator !=(
        Vec4 left,
        Vec4 right
    )
    {
        return !left.Equals(
            right
        );
    }

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