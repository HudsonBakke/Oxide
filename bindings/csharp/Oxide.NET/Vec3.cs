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
}