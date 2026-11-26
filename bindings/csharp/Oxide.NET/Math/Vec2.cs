using System;
using System.Runtime.InteropServices;

namespace Oxide.Math;

[StructLayout(LayoutKind.Sequential)]
/// <summary>Two-component floating-point vector with X and Y coordinates.</summary>
public struct Vec2 : IEquatable<Vec2>
{
    /// <summary>X component.</summary>
    public float X;
    /// <summary>Y component.</summary>
    public float Y;

    /// <summary>Creates a vector from its X and Y components.</summary>
    public Vec2(float x, float y)
    {
        X = x;
        Y = y;
    }

    /// <summary>Euclidean magnitude of the components.</summary>
    public readonly float Length => MathF.Sqrt(X * X + Y * Y);

    /// <summary>Returns a unit-length copy; a zero-length vector remains zero.</summary>
    public readonly Vec2 Normalized()
    {
        float length = Length;

        if (length == 0.0f)
        {
            return default;
        }

        return new Vec2(X / length, Y / length);
    }

    /// <summary>Normalizes the vector in place; leaves a zero-length vector unchanged.</summary>
    public void Normalize()
    {
        float length = Length;

        if (length == 0.0f)
        {
            return;
        }

        X /= length;
        Y /= length;
    }

    /// <summary>Returns the scalar dot product with the other vector.</summary>
    public readonly float Dot(Vec2 other)
    {
        return X * other.X + Y * other.Y;
    }

    /// <summary>Adds corresponding vector components.</summary>
    public static Vec2 operator +(Vec2 left, Vec2 right)
    {
        return new Vec2(left.X + right.X, left.Y + right.Y);
    }

    /// <summary>Subtracts vector components or negates a vector.</summary>
    public static Vec2 operator -(Vec2 left, Vec2 right)
    {
        return new Vec2(left.X - right.X, left.Y - right.Y);
    }

    /// <summary>Multiplies each component by a scalar.</summary>
    public static Vec2 operator *(Vec2 vector, float scalar)
    {
        return new Vec2(vector.X * scalar, vector.Y * scalar);
    }

    /// <summary>Multiplies each component by a scalar.</summary>
    public static Vec2 operator *(float scalar, Vec2 vector)
    {
        return vector * scalar;
    }

    /// <summary>Divides each component by a nonzero scalar.</summary>
    public static Vec2 operator /(Vec2 vector, float scalar)
    {
        return new Vec2(vector.X / scalar, vector.Y / scalar);
    }

    /// <summary>Subtracts vector components or negates a vector.</summary>
    public static Vec2 operator -(Vec2 vector)
    {
        return new Vec2(-vector.X, -vector.Y);
    }

    /// <summary>Tests exact component equality; no floating-point tolerance is applied.</summary>
    public readonly bool Equals(Vec2 other)
    {
        return X == other.X && Y == other.Y;
    }

    /// <summary>Tests exact component equality; no floating-point tolerance is applied.</summary>
    public override readonly bool Equals(object? obj)
    {
        return obj is Vec2 other && Equals(other);
    }

    /// <summary>Returns a hash consistent with exact component equality.</summary>
    public override readonly int GetHashCode()
    {
        return HashCode.Combine(X, Y);
    }

    /// <summary>Tests exact component equality.</summary>
    public static bool operator ==(Vec2 left, Vec2 right)
    {
        return left.Equals(right);
    }

    /// <summary>Tests exact component inequality.</summary>
    public static bool operator !=(Vec2 left, Vec2 right)
    {
        return !left.Equals(right);
    }
}
