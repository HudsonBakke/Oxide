using System;
using System.Runtime.InteropServices;

namespace Oxide;

[StructLayout(LayoutKind.Sequential)]
public readonly struct Quaternion
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

    public static Quaternion FromAxisAngle(
        Vec3 axis,
        Angle angle
    )
    {
        float axisLength =
            MathF.Sqrt(
                axis.X * axis.X
                + axis.Y * axis.Y
                + axis.Z * axis.Z
            );

        if (axisLength == 0.0f)
        {
            return Identity;
        }

        float halfAngle =
            angle.AsRadians * 0.5f;

        float sin =
            MathF.Sin(
                halfAngle
            );

        float cos =
            MathF.Cos(
                halfAngle
            );

        return new Quaternion(
            axis.X / axisLength * sin,
            axis.Y / axisLength * sin,
            axis.Z / axisLength * sin,
            cos
        );
    }

    public static Quaternion FromEuler(
        Angle pitch,
        Angle yaw,
        Angle roll
    )
    {
        Quaternion yawRotation =
            FromAxisAngle(
                new Vec3(
                    0.0f,
                    1.0f,
                    0.0f
                ),
                yaw
            );

        Quaternion pitchRotation =
            FromAxisAngle(
                new Vec3(
                    1.0f,
                    0.0f,
                    0.0f
                ),
                pitch
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
            rollRotation
            * pitchRotation
            * yawRotation
        ).Normalized();
    }

    public static Quaternion operator *(
        Quaternion a,
        Quaternion b
    )
    {
        return new Quaternion(
            a.W * b.X
                + a.X * b.W
                + a.Y * b.Z
                - a.Z * b.Y,

            a.W * b.Y
                - a.X * b.Z
                + a.Y * b.W
                + a.Z * b.X,

            a.W * b.Z
                + a.X * b.Y
                - a.Y * b.X
                + a.Z * b.W,

            a.W * b.W
                - a.X * b.X
                - a.Y * b.Y
                - a.Z * b.Z
        );
    }
}