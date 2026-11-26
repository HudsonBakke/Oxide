using System.Runtime.InteropServices;

namespace Oxide.Math;

/// <summary>Pixel rectangle with top-left X/Y coordinates and width/height extents.</summary>
[StructLayout(LayoutKind.Sequential)]
public readonly struct Rect : IEquatable<Rect>
{
    /// <summary>Left edge in physical pixels.</summary>
    public readonly float X;
    /// <summary>Top edge in physical pixels.</summary>
    public readonly float Y;
    /// <summary>Horizontal extent in physical pixels.</summary>
    public readonly float Width;
    /// <summary>Vertical extent in physical pixels.</summary>
    public readonly float Height;

    /// <summary>Creates a pixel rectangle from top-left coordinates and width/height extents.</summary>
    public Rect(float x, float y, float width, float height)
    {
        X = x;
        Y = y;
        Width = width;
        Height = height;
    }

    /// <summary>Width divided by height; height must be nonzero for a finite result.</summary>
    public float AspectRatio => Width / Height;

    /// <summary>Tests exact component equality; no floating-point tolerance is applied.</summary>
    public bool Equals(Rect other)
    {
        return X == other.X && Y == other.Y && Width == other.Width && Height == other.Height;
    }

    /// <summary>Tests exact component equality; no floating-point tolerance is applied.</summary>
    public override bool Equals(object? obj)
    {
        return obj is Rect other && Equals(other);
    }

    /// <summary>Returns a hash consistent with exact component equality.</summary>
    public override int GetHashCode()
    {
        return HashCode.Combine(X, Y, Width, Height);
    }

    /// <summary>Tests exact component equality.</summary>
    public static bool operator ==(Rect left, Rect right) => left.Equals(right);

    /// <summary>Tests exact component inequality.</summary>
    public static bool operator !=(Rect left, Rect right) => !left.Equals(right);
}
