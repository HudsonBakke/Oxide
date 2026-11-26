using System;
using System.Runtime.InteropServices;

namespace Oxide.Math;

[StructLayout(LayoutKind.Sequential)]
/// <summary>Quaternion rotation for three-dimensional orientation.</summary>
public readonly struct Quaternion
    : IEquatable<Quaternion>
{
    public readonly float X;
    public readonly float Y;
    public readonly float Z;
    public readonly float W;

    public Quaternion(
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

    public static Quaternion Identity =>
        new(
            0.0f,
            0.0f,
            0.0f,
            1.0f
        );

    public float Length =>
        MathF.Sqrt(
            X * X
            + Y * Y
            + Z * Z
            + W * W
        );

    public Quaternion Normalized()
    {
        float length =
            Length;

        if (length == 0.0f)
        {
            return Identity;
        }

        return new Quaternion(
            X / length,
            Y / length,
            Z / length,
            W / length
        );
    }

    public Quaternion Conjugate()
    {
        return new Quaternion(
            -X,
            -Y,
            -Z,
            W
        );
    }

    public Vec3 RotateVector(
        Vec3 vector
    )
    {
        var q =
            new Vec3(
                X,
                Y,
                Z
            );

        Vec3 t =
            q.Cross(vector)
            * 2.0f;

        return vector
            + t * W
            + q.Cross(t);
    }

    public static Quaternion FromAxisAngle(
        Vec3 axis,
        Angle angle
    )
    {
        axis =
            axis.Normalized();

        float halfAngle =
            angle.AsRadians
            * 0.5f;

        float sin =
            MathF.Sin(
                halfAngle
            );

        return new Quaternion(
            axis.X * sin,
            axis.Y * sin,
            axis.Z * sin,
            MathF.Cos(
                halfAngle
            )
        );
    }

    public static Quaternion FromEuler(
        Angle pitch,
        Angle yaw,
        Angle roll
    )
    {
        Quaternion pitchRotation =
            FromAxisAngle(
                new Vec3(
                    1.0f,
                    0.0f,
                    0.0f
                ),
                pitch
            );

        Quaternion yawRotation =
            FromAxisAngle(
                new Vec3(
                    0.0f,
                    1.0f,
                    0.0f
                ),
                yaw
            );

        Quaternion rollRotation =
            FromAxisAngle(
                new Vec3(
                    0.0f,
                    0.0f,
                    1.0f
                ),
                roll
            );

        return (
            yawRotation
            * pitchRotation
            * rollRotation
        ).Normalized();
    }

    public static Quaternion FromTo(
        Vec3 from,
        Vec3 to
    )
    {
        from =
            from.Normalized();

        to =
            to.Normalized();

        float dot =
            from.Dot(to);

        if (dot
            > 1.0f - 1.0e-6f)
        {
            return Identity;
        }

        if (dot
            < -1.0f + 1.0e-6f)
        {
            Vec3 candidate;

            if (MathF.Abs(from.X)
                    <= MathF.Abs(from.Y)
                && MathF.Abs(from.X)
                    <= MathF.Abs(from.Z))
            {
                candidate =
                    new Vec3(
                        1.0f,
                        0.0f,
                        0.0f
                    );
            }
            else if (
                MathF.Abs(from.Y)
                    <= MathF.Abs(from.Z)
            )
            {
                candidate =
                    new Vec3(
                        0.0f,
                        1.0f,
                        0.0f
                    );
            }
            else
            {
                candidate =
                    new Vec3(
                        0.0f,
                        0.0f,
                        1.0f
                    );
            }

            Vec3 axis =
                from
                    .Cross(candidate)
                    .Normalized();

            return FromAxisAngle(
                axis,
                Angle.Radians(
                    MathF.PI
                )
            );
        }

        Vec3 axis2 =
            from.Cross(to);

        return new Quaternion(
            axis2.X,
            axis2.Y,
            axis2.Z,
            1.0f + dot
        ).Normalized();
    }

    public static Quaternion operator *(
        Quaternion left,
        Quaternion right
    )
    {
        return new Quaternion(
            left.W * right.X
                + left.X * right.W
                + left.Y * right.Z
                - left.Z * right.Y,

            left.W * right.Y
                - left.X * right.Z
                + left.Y * right.W
                + left.Z * right.X,

            left.W * right.Z
                + left.X * right.Y
                - left.Y * right.X
                + left.Z * right.W,

            left.W * right.W
                - left.X * right.X
                - left.Y * right.Y
                - left.Z * right.Z
        );
    }

    public bool Equals(
        Quaternion other
    )
    {
        return X == other.X
            && Y == other.Y
            && Z == other.Z
            && W == other.W;
    }

    public override bool Equals(
        object? obj
    )
    {
        return obj
            is Quaternion other
            && Equals(other);
    }

    public override int GetHashCode()
    {
        return HashCode.Combine(
            X,
            Y,
            Z,
            W
        );
    }

    public static bool operator ==(
        Quaternion left,
        Quaternion right
    )
    {
        return left.Equals(
            right
        );
    }

    public static bool operator !=(
        Quaternion left,
        Quaternion right
    )
    {
        return !left.Equals(
            right
        );
    }
}