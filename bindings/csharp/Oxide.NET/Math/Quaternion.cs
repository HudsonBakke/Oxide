using System;
using System.Runtime.InteropServices;

namespace Oxide.Math;

/// <summary>Rotation quaternion with XYZW components; normalize manually constructed values before rotating vectors.</summary>
[StructLayout(LayoutKind.Sequential)]
public readonly struct Quaternion : IEquatable<Quaternion>
{
    /// <summary>X component.</summary>
    public readonly float X;
    /// <summary>Y component.</summary>
    public readonly float Y;
    /// <summary>Z component.</summary>
    public readonly float Z;
    /// <summary>W component.</summary>
    public readonly float W;

    /// <summary>Creates a quaternion from XYZW components without normalizing it.</summary>
    public Quaternion(float x, float y, float z, float w)
    {
        X = x;
        Y = y;
        Z = z;
        W = w;
    }

    /// <summary>Rotation quaternion (0, 0, 0, 1), which leaves vectors unchanged.</summary>
    public static Quaternion Identity => new(0.0f, 0.0f, 0.0f, 1.0f);
    /// <summary>Euclidean magnitude of the components.</summary>
    public float Length => MathF.Sqrt(X * X + Y * Y + Z * Z + W * W);

    /// <summary>Returns a unit-length quaternion; a zero quaternion becomes the identity.</summary>
    public Quaternion Normalized()
    {
        float length = Length;

        if (length == 0.0f)
        {
            return Identity;
        }

        return new Quaternion(X / length, Y / length, Z / length, W / length);
    }

    /// <summary>Returns the quaternion with negated XYZ components; the inverse for a unit quaternion.</summary>
    public Quaternion Conjugate()
    {
        return new Quaternion(-X, -Y, -Z, W);
    }

    /// <summary>Rotates a vector by this quaternion; use a unit quaternion for a pure rotation.</summary>
    public Vec3 RotateVector(Vec3 vector)
    {
        var q = new Vec3(X, Y, Z);

        Vec3 t = q.Cross(vector) * 2.0f;

        return vector + t * W + q.Cross(t);
    }

    /// <summary>Creates a rotation about a nonzero axis, normalizing the axis before use.</summary>
    public static Quaternion FromAxisAngle(Vec3 axis, Angle angle)
    {
        axis = axis.Normalized();

        float halfAngle = angle.AsRadians * 0.5f;

        float sin = MathF.Sin(halfAngle);

        return new Quaternion(axis.X * sin, axis.Y * sin, axis.Z * sin, MathF.Cos(halfAngle));
    }

    /// <summary>Constructs yaw * pitch * roll about Y, X, and Z; the rightmost rotation acts first on a vector.</summary>
    public static Quaternion FromEuler(Angle pitch, Angle yaw, Angle roll)
    {
        Quaternion pitchRotation = FromAxisAngle(new Vec3(1.0f, 0.0f, 0.0f), pitch);

        Quaternion yawRotation = FromAxisAngle(new Vec3(0.0f, 1.0f, 0.0f), yaw);

        Quaternion rollRotation = FromAxisAngle(new Vec3(0.0f, 0.0f, 1.0f), roll);

        return (yawRotation * pitchRotation * rollRotation).Normalized();
    }

    /// <summary>Returns the shortest rotation aligning two nonzero directions; vector lengths are ignored.</summary>
    public static Quaternion FromTo(Vec3 from, Vec3 to)
    {
        from = from.Normalized();

        to = to.Normalized();

        float dot = from.Dot(to);

        if (dot > 1.0f - 1.0e-6f)
        {
            return Identity;
        }

        if (dot < -1.0f + 1.0e-6f)
        {
            Vec3 candidate;

            if (MathF.Abs(from.X) <= MathF.Abs(from.Y) && MathF.Abs(from.X) <= MathF.Abs(from.Z))
            {
                candidate = new Vec3(1.0f, 0.0f, 0.0f);
            }
            else if (MathF.Abs(from.Y) <= MathF.Abs(from.Z))
            {
                candidate = new Vec3(0.0f, 1.0f, 0.0f);
            }
            else
            {
                candidate = new Vec3(0.0f, 0.0f, 1.0f);
            }

            Vec3 axis = from.Cross(candidate).Normalized();

            return FromAxisAngle(axis, Angle.Radians(MathF.PI));
        }

        Vec3 axis2 = from.Cross(to);

        return new Quaternion(axis2.X, axis2.Y, axis2.Z, 1.0f + dot).Normalized();
    }

    /// <summary>Composes rotations; the right-hand quaternion acts first.</summary>
    public static Quaternion operator *(Quaternion left, Quaternion right)
    {
        return new Quaternion(
            left.W * right.X + left.X * right.W + left.Y * right.Z - left.Z * right.Y,

            left.W * right.Y - left.X * right.Z + left.Y * right.W + left.Z * right.X,

            left.W * right.Z + left.X * right.Y - left.Y * right.X + left.Z * right.W,

            left.W * right.W - left.X * right.X - left.Y * right.Y - left.Z * right.Z);
    }

    /// <summary>Tests exact component equality; no floating-point tolerance is applied.</summary>
    public bool Equals(Quaternion other)
    {
        return X == other.X && Y == other.Y && Z == other.Z && W == other.W;
    }

    /// <summary>Tests exact component equality; no floating-point tolerance is applied.</summary>
    public override bool Equals(object? obj)
    {
        return obj is Quaternion other && Equals(other);
    }

    /// <summary>Returns a hash consistent with exact component equality.</summary>
    public override int GetHashCode()
    {
        return HashCode.Combine(X, Y, Z, W);
    }

    /// <summary>Tests exact component equality.</summary>
    public static bool operator ==(Quaternion left, Quaternion right)
    {
        return left.Equals(right);
    }

    /// <summary>Tests exact component inequality.</summary>
    public static bool operator !=(Quaternion left, Quaternion right)
    {
        return !left.Equals(right);
    }
}
