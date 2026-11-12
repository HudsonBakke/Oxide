using System.Runtime.InteropServices;

namespace Oxide;

[StructLayout(LayoutKind.Sequential)]
public readonly struct Color
{
    public readonly float R;
    public readonly float G;
    public readonly float B;
    public readonly float A;

    public Color(
        float r,
        float g,
        float b,
        float a = 1.0f
    )
    {
        R = r;
        G = g;
        B = b;
        A = a;
    }

    public static Color Black =>
        new(0.0f, 0.0f, 0.0f);

    public static Color White =>
        new(1.0f, 1.0f, 1.0f);

    public static Color Red =>
        new(1.0f, 0.0f, 0.0f);

    public static Color Green =>
        new(0.0f, 1.0f, 0.0f);

    public static Color Blue =>
        new(0.0f, 0.0f, 1.0f);

    public static Color Yellow =>
        new(1.0f, 1.0f, 0.0f);
}