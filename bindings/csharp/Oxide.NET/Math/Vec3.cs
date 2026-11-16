using System;
using System.Runtime.InteropServices;

namespace Oxide.Math;

[StructLayout(LayoutKind.Sequential)]
public struct Vec3
    : IEquatable<Vec3>
{
    public float X;
    public float Y;
    public float Z;

    public Vec3(
        float x,
        float y,
        float z
    )
    {
        X = x;
        Y = y;
        Z = z;
    }

    public readonly float Length =>
        MathF.Sqrt(
            X * X
            + Y * Y
            + Z * Z
        );

    public readonly Vec3 Normalized()
    {
        float length =
            Length;

        if (length == 0.0f)
        {
            return default;
        }

        return new Vec3(
            X / length,
            Y / length,
            Z / length
        );
    }

    public void Normalize()
    {
        float length =
            Length;

        if (length == 0.0f)
        {
            return;
        }

        X /= length;
        Y /= length;
        Z /= length;
    }

    public readonly float Dot(
        Vec3 other
    )
    {
        return X * other.X
            + Y * other.Y
            + Z * other.Z;
    }

    public readonly Vec3 Cross(
        Vec3 other
    )
    {
        return new Vec3(
            Y * other.Z
                - Z * other.Y,

            Z * other.X
                - X * other.Z,

            X * other.Y
                - Y * other.X
        );
    }

    public static Vec3 operator +(
        Vec3 left,
        Vec3 right
    )
    {
        return new Vec3(
            left.X + right.X,
            left.Y + right.Y,
            left.Z + right.Z
        );
    }

    public static Vec3 operator -(
        Vec3 left,
        Vec3 right
    )
    {
        return new Vec3(
            left.X - right.X,
            left.Y - right.Y,
            left.Z - right.Z
        );
    }

    public static Vec3 operator *(
        Vec3 vector,
        float scalar
    )
    {
        return new Vec3(
            vector.X * scalar,
            vector.Y * scalar,
            vector.Z * scalar
        );
    }

    public static Vec3 operator *(
        float scalar,
        Vec3 vector
    )
    {
        return vector * scalar;
    }

    public static Vec3 operator /(
        Vec3 vector,
        float scalar
    )
    {
        return new Vec3(
            vector.X / scalar,
            vector.Y / scalar,
            vector.Z / scalar
        );
    }

    public static Vec3 operator -(
        Vec3 vector
    )
    {
        return new Vec3(
            -vector.X,
            -vector.Y,
            -vector.Z
        );
    }

    public readonly bool Equals(
        Vec3 other
    )
    {
        return X == other.X
            && Y == other.Y
            && Z == other.Z;
    }

    public override readonly bool Equals(
        object? obj
    )
    {
        return obj is Vec3 other
            && Equals(other);
    }

    public override readonly int GetHashCode()
    {
        return HashCode.Combine(
            X,
            Y,
            Z
        );
    }

    public static bool operator ==(
        Vec3 left,
        Vec3 right
    )
    {
        return left.Equals(
            right
        );
    }

    public static bool operator !=(
        Vec3 left,
        Vec3 right
    )
    {
        return !left.Equals(
            right
        );
    }
}