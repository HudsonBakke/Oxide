using System;

using System.Runtime.InteropServices;

namespace Oxide.Graphics;

/// <summary>Linear RGBA color with floating-point channels.</summary>
[StructLayout(LayoutKind.Sequential)]
public readonly struct Color
    : IEquatable<Color>
{
    /// <summary>Red channel.</summary>
    public readonly float R;
    /// <summary>Green channel.</summary>
    public readonly float G;
    /// <summary>Blue channel.</summary>
    public readonly float B;
    /// <summary>Alpha channel.</summary>
    public readonly float A;

    /// <summary>Light color.</summary>
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

    /// <summary>Creates an opaque color from RGB channels.</summary>
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

    /// <summary>Creates a color from RGBA channels.</summary>
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

    /// <summary>Tests value equality.</summary>
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