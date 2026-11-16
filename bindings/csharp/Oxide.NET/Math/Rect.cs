using System.Runtime.InteropServices;

namespace Oxide.Math;

[StructLayout(LayoutKind.Sequential)]
public readonly struct Rect
    : IEquatable<Rect>
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