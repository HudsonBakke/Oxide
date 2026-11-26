using System;
using System.Runtime.InteropServices;

namespace Oxide.Math;

[StructLayout(LayoutKind.Sequential)]
/// <summary>Angle stored in radians; construct from radians or degrees explicitly.</summary>
public readonly struct Angle : IEquatable<Angle>
{
    private readonly float _radians;

    private Angle(float radians)
    {
        _radians = radians;
    }

    /// <summary>An angle of zero radians.</summary>
    public static Angle Zero => new(0.0f);

    /// <summary>Creates an angle from a value measured in radians.</summary>
    public static Angle Radians(float radians)
    {
        return new Angle(radians);
    }

    /// <summary>Creates an angle from a value measured in degrees.</summary>
    public static Angle Degrees(float degrees)
    {
        return new Angle(degrees * MathF.PI / 180.0f);
    }

    /// <summary>The angle measured in radians.</summary>
    public float AsRadians => _radians;
    /// <summary>The angle measured in degrees.</summary>
    public float AsDegrees => _radians * 180.0f / MathF.PI;

    /// <summary>Tests exact component equality; no floating-point tolerance is applied.</summary>
    public bool Equals(Angle other)
    {
        return _radians == other._radians;
    }

    /// <summary>Tests exact component equality; no floating-point tolerance is applied.</summary>
    public override bool Equals(object? obj)
    {
        return obj is Angle other && Equals(other);
    }

    /// <summary>Returns a hash consistent with exact component equality.</summary>
    public override int GetHashCode()
    {
        return _radians.GetHashCode();
    }

    /// <summary>Tests exact component equality.</summary>
    public static bool operator ==(Angle left, Angle right)
    {
        return left.Equals(right);
    }

    /// <summary>Tests exact component inequality.</summary>
    public static bool operator !=(Angle left, Angle right)
    {
        return !left.Equals(right);
    }
}
