using System.Runtime.InteropServices;

namespace Oxide;

[StructLayout(LayoutKind.Sequential)]
public readonly struct Vec3
{
    public readonly float X;
    public readonly float Y;
    public readonly float Z;

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

    public static Vec3 operator +(
        Vec3 a,
        Vec3 b
    )
    {
        return new Vec3(
            a.X + b.X,
            a.Y + b.Y,
            a.Z + b.Z
        );
    }

    public static Vec3 operator -(
        Vec3 a,
        Vec3 b
    )
    {
        return new Vec3(
            a.X - b.X,
            a.Y - b.Y,
            a.Z - b.Z
        );
    }

    public static Vec3 operator *(
        Vec3 value,
        float scalar
    )
    {
        return new Vec3(
            value.X * scalar,
            value.Y * scalar,
            value.Z * scalar
        );
    }
}