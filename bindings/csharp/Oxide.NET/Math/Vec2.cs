using System;
using System.Runtime.InteropServices;

namespace Oxide.Math;

/// <summary>Two-component floating-point vector.</summary>
[StructLayout(LayoutKind.Sequential)]
public struct Vec2
    : IEquatable<Vec2>
{
    /// <summary>X component.</summary>
    public float X;
    /// <summary>Y component.</summary>
    public float Y;

    /// <summary>Creates or accesses Vec2.</summary>
    public Vec2(
        float x,
        float y
    )
    {
        X = x;
        Y = y;
    }

    /// <summary>Vector magnitude.</summary>
    public readonly float Length =>
        MathF.Sqrt(
            X * X
            + Y * Y
        );

    /// <summary>Returns a normalized copy; zero vectors remain zero.</summary>
    public readonly Vec2 Normalized()
    {
        float length =
            Length;

        if (length == 0.0f)
        {
            return default;
        }

        return new Vec2(
            X / length,
            Y / length
        );
    }

    /// <summary>Normalizes this vector in place; zero vectors remain zero.</summary>
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
    }

    /// <summary>Returns the dot product with another vector.</summary>
    public readonly float Dot(
        Vec2 other
    )
    {
        return X * other.X
            + Y * other.Y;
    }

    public static Vec2 operator +(
        Vec2 left,
        Vec2 right
    )
    {
        return new Vec2(
            left.X + right.X,
            left.Y + right.Y
        );
    }

    public static Vec2 operator -(
        Vec2 left,
        Vec2 right
    )
    {
        return new Vec2(
            left.X - right.X,
            left.Y - right.Y
        );
    }

    public static Vec2 operator *(
        Vec2 vector,
        float scalar
    )
    {
        return new Vec2(
            vector.X * scalar,
            vector.Y * scalar
        );
    }

    public static Vec2 operator *(
        float scalar,
        Vec2 vector
    )
    {
        return vector * scalar;
    }

    public static Vec2 operator /(
        Vec2 vector,
        float scalar
    )
    {
        return new Vec2(
            vector.X / scalar,
            vector.Y / scalar
        );
    }

    public static Vec2 operator -(
        Vec2 vector
    )
    {
        return new Vec2(
            -vector.X,
            -vector.Y
        );
    }

    /// <summary>Tests value equality.</summary>
    public readonly bool Equals(
        Vec2 other
    )
    {
        return X == other.X
            && Y == other.Y;
    }

    public override readonly bool Equals(
        object? obj
    )
    {
        return obj is Vec2 other
            && Equals(other);
    }

    public override readonly int GetHashCode()
    {
        return HashCode.Combine(
            X,
            Y
        );
    }

    public static bool operator ==(
        Vec2 left,
        Vec2 right
    )
    {
        return left.Equals(
            right
        );
    }

    public static bool operator !=(
        Vec2 left,
        Vec2 right
    )
    {
        return !left.Equals(
            right
        );
    }
}