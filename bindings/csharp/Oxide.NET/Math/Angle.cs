using System;
using System.Runtime.InteropServices;

namespace Oxide.Math;

[StructLayout(LayoutKind.Sequential)]
/// <summary>Angle value stored in radians with degree conversion helpers.</summary>
public readonly struct Angle
    : IEquatable<Angle>
{
    private readonly float _radians;

    private Angle(
        float radians
    )
    {
        _radians = radians;
    }

    public static Angle Zero =>
        new(0.0f);

    public static Angle Radians(
        float radians
    )
    {
        return new Angle(
            radians
        );
    }

    public static Angle Degrees(
        float degrees
    )
    {
        return new Angle(
            degrees
                * MathF.PI
                / 180.0f
        );
    }

    public float AsRadians =>
        _radians;

    public float AsDegrees =>
        _radians
            * 180.0f
            / MathF.PI;

    public bool Equals(
        Angle other
    )
    {
        return _radians
            == other._radians;
    }

    public override bool Equals(
        object? obj
    )
    {
        return obj is Angle other
            && Equals(other);
    }

    public override int GetHashCode()
    {
        return _radians
            .GetHashCode();
    }

    public static bool operator ==(
        Angle left,
        Angle right
    )
    {
        return left.Equals(
            right
        );
    }

    public static bool operator !=(
        Angle left,
        Angle right
    )
    {
        return !left.Equals(
            right
        );
    }
}