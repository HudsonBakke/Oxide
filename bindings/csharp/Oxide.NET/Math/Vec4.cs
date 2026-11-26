using System;
using System.Runtime.InteropServices;

namespace Oxide.Math;

[StructLayout(LayoutKind.Sequential)]
/// <summary>Four-component floating-point vector.</summary>
public struct Vec4
    : IEquatable<Vec4>
{
    public float X;
    public float Y;
    public float Z;
    public float W;

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