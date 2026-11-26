using System;
using System.Runtime.InteropServices;

using Oxide.Math;

namespace Oxide.Graphics;

/// <summary>Omnidirectional point light with inverse-square distance attenuation.</summary>
[StructLayout(LayoutKind.Sequential)]
public struct PointLight
{
    private Vec3 _position;
    private Color _color;
    private float _intensity;

    /// <summary>Creates a point light; intensity must not be negative.</summary>
    public PointLight(Vec3 position, Color color, float intensity)
    {
        if (intensity < 0.0f)
        {
            throw new ArgumentOutOfRangeException(nameof(intensity),
                                                  "Light intensity cannot be negative.");
        }

        _position = position;
        _color = color;
        _intensity = intensity;
    }

    /// <summary>Position in world space.</summary>
    public Vec3 Position
    {
        readonly get => _position;
        set => _position = value;
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
