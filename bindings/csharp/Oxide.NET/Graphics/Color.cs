using System;

using System.Runtime.InteropServices;

namespace Oxide.Graphics;
[StructLayout(LayoutKind.Sequential)]
/// <summary>Floating-point RGBA color; values are stored without clamping.</summary>
public readonly struct Color
    : IEquatable<Color>
{
    /// <summary>Red component.</summary>
    public readonly float R;
    /// <summary>Green component.</summary>
    public readonly float G;
    /// <summary>Blue component.</summary>
    public readonly float B;
    /// <summary>Alpha component: zero is transparent and one is opaque.</summary>
    public readonly float A;
    /// <summary>Creates a color from RGBA components without clamping.</summary>
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
    /// <summary>Creates an RGB color with alpha set to one.</summary>
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
    /// <summary>Creates a color from red, green, blue, and alpha components without clamping.</summary>
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
    /// <summary>Opaque black.</summary>
    public static Color Black =>
        Rgb(0.0f, 0.0f, 0.0f);
    /// <summary>Opaque white.</summary>
    public static Color White =>
        Rgb(1.0f, 1.0f, 1.0f);
    /// <summary>Opaque red.</summary>
    public static Color Red =>
        Rgb(1.0f, 0.0f, 0.0f);
    /// <summary>Opaque green.</summary>
    public static Color Green =>
        Rgb(0.0f, 1.0f, 0.0f);
    /// <summary>Opaque blue.</summary>
    public static Color Blue =>
        Rgb(0.0f, 0.0f, 1.0f);
    /// <summary>Opaque yellow.</summary>
    public static Color Yellow =>
        Rgb(1.0f, 1.0f, 0.0f);
    /// <summary>Tests exact component equality; no floating-point tolerance is applied.</summary>
    public bool Equals(
        Color other
    )
    {
        return R == other.R
            && G == other.G
            && B == other.B
            && A == other.A;
    }

    /// <summary>Tests exact component equality; no floating-point tolerance is applied.</summary>
    public override bool Equals(
        object? obj
    )
    {
        return obj
            is Color other
            && Equals(other);
    }

    /// <summary>Returns a hash consistent with exact component equality.</summary>
    public override int GetHashCode()
    {
        return HashCode.Combine(
            R,
            G,
            B,
            A
        );
    }

    /// <summary>Tests exact component equality.</summary>
    public static bool operator ==(
        Color left,
        Color right
    ) =>
        left.Equals(right);

    /// <summary>Tests exact component inequality.</summary>
    public static bool operator !=(
        Color left,
        Color right
    ) =>
        !left.Equals(right);
}
