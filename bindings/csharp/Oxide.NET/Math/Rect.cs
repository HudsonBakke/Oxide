using System.Runtime.InteropServices;

namespace Oxide.Math;

/// <summary>Rectangle expressed by position and size.</summary>
[StructLayout(LayoutKind.Sequential)]
public readonly struct Rect
    : IEquatable<Rect>
{
    /// <summary>Creates or accesses X.</summary>
    public readonly float X;
    /// <summary>Creates or accesses Y.</summary>
    public readonly float Y;
    /// <summary>Creates or accesses Width.</summary>
    public readonly float Width;
    /// <summary>Creates or accesses Height.</summary>
    public readonly float Height;

    /// <summary>Creates or accesses Rect.</summary>
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

    /// <summary>Width divided by height.</summary>
    public float AspectRatio =>
        Width / Height;

    /// <summary>Tests value equality.</summary>
    public bool Equals(
        Rect other
    )
    {
        return X == other.X
            && Y == other.Y
            && Width == other.Width
            && Height == other.Height;
    }

    public override bool Equals(
        object? obj
    )
    {
        return obj is Rect other
            && Equals(other);
    }

    public override int GetHashCode()
    {
        return HashCode.Combine(
            X,
            Y,
            Width,
            Height
        );
    }

    public static bool operator ==(
        Rect left,
        Rect right
    ) =>
        left.Equals(right);

    public static bool operator !=(
        Rect left,
        Rect right
    ) =>
        !left.Equals(right);
}