using System;
using System.Runtime.InteropServices;

namespace Oxide.Math;
[StructLayout(LayoutKind.Sequential)]
/// <summary>Three-component floating-point vector for positions, directions, and scales.</summary>
public struct Vec3
    : IEquatable<Vec3>
{
    /// <summary>X component.</summary>
    public float X;
    /// <summary>Y component.</summary>
    public float Y;
    /// <summary>Z component.</summary>
    public float Z;
    /// <summary>Creates a vector from its X, Y, and Z components.</summary>
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
    /// <summary>Euclidean magnitude of the components.</summary>
    public readonly float Length =>
        MathF.Sqrt(
            X * X
            + Y * Y
            + Z * Z
        );
    /// <summary>Returns a unit-length copy; a zero-length vector remains zero.</summary>
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
    /// <summary>Normalizes the vector in place; leaves a zero-length vector unchanged.</summary>
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
    /// <summary>Returns the scalar dot product with the other vector.</summary>
    public readonly float Dot(
        Vec3 other
    )
    {
        return X * other.X
            + Y * other.Y
            + Z * other.Z;
    }
    /// <summary>Returns the right-handed cross product with the other vector.</summary>
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

    /// <summary>Adds corresponding vector components.</summary>
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

    /// <summary>Subtracts vector components or negates a vector.</summary>
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

    /// <summary>Multiplies each component by a scalar.</summary>
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

    /// <summary>Multiplies each component by a scalar.</summary>
    public static Vec3 operator *(
        float scalar,
        Vec3 vector
    )
    {
        return vector * scalar;
    }

    /// <summary>Divides each component by a nonzero scalar.</summary>
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

    /// <summary>Subtracts vector components or negates a vector.</summary>
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
    /// <summary>Tests exact component equality; no floating-point tolerance is applied.</summary>
    public readonly bool Equals(
        Vec3 other
    )
    {
        return X == other.X
            && Y == other.Y
            && Z == other.Z;
    }

    /// <summary>Tests exact component equality; no floating-point tolerance is applied.</summary>
    public override readonly bool Equals(
        object? obj
    )
    {
        return obj is Vec3 other
            && Equals(other);
    }

    /// <summary>Returns a hash consistent with exact component equality.</summary>
    public override readonly int GetHashCode()
    {
        return HashCode.Combine(
            X,
            Y,
            Z
        );
    }

    /// <summary>Tests exact component equality.</summary>
    public static bool operator ==(
        Vec3 left,
        Vec3 right
    )
    {
        return left.Equals(
            right
        );
    }

    /// <summary>Tests exact component inequality.</summary>
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
