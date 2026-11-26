using System;

using System.Runtime.InteropServices;

namespace Oxide.Graphics;

[StructLayout(LayoutKind.Sequential)]
/// <summary>Linear RGBA color with floating-point channels.</summary>
public readonly struct Color
    : IEquatable<Color>
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

    public static Color Rgb(
        float r,
        float g,
        float b
    )
    {
        return new Color(
            r,
            g,
            b,
            1.0f
        );
    }

    public static Color Rgba(
        float r,
        float g,
        float b,
        float a
    )
    {
        return new Color(
            r,
            g,
            b,
            a
        );
    }

    public static Color Black =>
        Rgb(0.0f, 0.0f, 0.0f);

    public static Color White =>
        Rgb(1.0f, 1.0f, 1.0f);

    public static Color Red =>
        Rgb(1.0f, 0.0f, 0.0f);

    public static Color Green =>
        Rgb(0.0f, 1.0f, 0.0f);

    public static Color Blue =>
        Rgb(0.0f, 0.0f, 1.0f);

    public static Color Yellow =>
        Rgb(1.0f, 1.0f, 0.0f);

    public bool Equals(
        Color other
    )
    {
        return R == other.R
            && G == other.G
            && B == other.B
            && A == other.A;
    }

    public override bool Equals(
        object? obj
    )
    {
        return obj
            is Color other
            && Equals(other);
    }

    public override int GetHashCode()
    {
        return HashCode.Combine(
            R,
            G,
            B,
            A
        );
    }

    public static bool operator ==(
        Color left,
        Color right
    ) =>
        left.Equals(right);

    public static bool operator !=(
        Color left,
        Color right
    ) =>
        !left.Equals(right);
}