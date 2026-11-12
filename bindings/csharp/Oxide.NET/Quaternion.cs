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
}