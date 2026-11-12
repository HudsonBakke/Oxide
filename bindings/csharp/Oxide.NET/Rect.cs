using System.Runtime.InteropServices;

namespace Oxide;

[StructLayout(LayoutKind.Sequential)]
public readonly struct Rect
{
    public readonly float X;
    public readonly float Y;
    public readonly float Width;
    public readonly float Height;

    public Rect(
        float x,
        float y,
        float width,
        float height
    )
    {
        X = x;
        Y = y;
        Width = width;
        Height = height;
    }

    public float AspectRatio =>
        Width / Height;
}