using System;
using System.Runtime.InteropServices;

using Oxide.Math;

namespace Oxide.Graphics;

/// <summary>Point light restricted to a directional cone, with inverse-square distance attenuation.</summary>
[StructLayout(LayoutKind.Sequential)]
public struct SpotLight
{
    private Vec3 _position;
    private Vec3 _direction;
    private Angle _angle;
    private Color _color;
    private float _intensity;

    /// <summary>Creates a cone light; requires a nonzero direction, non-negative intensity, and half-angle in (0, pi) radians.</summary>
    public SpotLight(Vec3 position, Vec3 direction, Angle angle, Color color, float intensity)
    {
        if (direction.Length <= 1.0e-6f)
        {
            throw new ArgumentException("Spot light direction cannot be zero.", nameof(direction));
        }

        if (angle.AsRadians <= 0.0f || angle.AsRadians >= MathF.PI)
        {
            throw new ArgumentOutOfRangeException(
                nameof(angle), "Spot light angle must be between 0 and pi radians.");
        }

        if (intensity < 0.0f)
        {
            throw new ArgumentOutOfRangeException(nameof(intensity),
                                                  "Light intensity cannot be negative.");
        }

        _position = position;

        _direction = direction.Normalized();

        _angle = angle;
        _color = color;
        _intensity = intensity;
    }

    /// <summary>Position in world space.</summary>
    public Vec3 Position
    {
        readonly get => _position;

        set => _position = value;
    }
    /// <summary>Normalized world-space direction in which light rays travel.</summary>
    public Vec3 Direction
    {
        readonly get => _direction;

        set
        {
            if (value.Length <= 1.0e-6f)
            {
                throw new ArgumentException("Spot light direction cannot be zero.", nameof(value));
            }

            _direction = value.Normalized();
        }
    }
    /// <summary>Spotlight cone half-angle measured from its direction; use an angle strictly between zero and pi radians.</summary>
    public Angle Angle
    {
        readonly get => _angle;

        set
        {
            if (value.AsRadians <= 0.0f || value.AsRadians >= MathF.PI)
            {
                throw new ArgumentOutOfRangeException(
                    nameof(value), "Spot light angle must be between 0 and pi radians.");
            }

            _angle = value;
        }
    }
    /// <summary>RGB light color; alpha is not used for lighting.</summary>
    public Color Color
    {
        readonly get => _color;

        set => _color = value;
    }
    /// <summary>Non-negative light intensity; point and spot lights attenuate with squared distance.</summary>
    public float Intensity
    {
        readonly get => _intensity;

        set
        {
            if (value < 0.0f)
            {
                throw new ArgumentOutOfRangeException(nameof(value),
                                                      "Light intensity cannot be negative.");
            }

            _intensity = value;
        }
    }
}
