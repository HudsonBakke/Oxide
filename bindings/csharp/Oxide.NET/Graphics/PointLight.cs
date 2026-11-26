using System;
using System.Runtime.InteropServices;

using Oxide.Math;

namespace Oxide.Graphics;

[StructLayout(LayoutKind.Sequential)]
/// <summary>Omnidirectional light emitted from a point in world space.</summary>
public struct PointLight
{
    private Vec3 _position;
    private Color _color;
    private float _intensity;

    /// <summary>Creates or accesses PointLight.</summary>
    public PointLight(
        Vec3 position,
        Color color,
        float intensity
    )
    {
        if (intensity < 0.0f)
        {
            throw new ArgumentOutOfRangeException(
                nameof(intensity),
                "Light intensity cannot be negative."
            );
        }

        _position = position;
        _color = color;
        _intensity = intensity;
    }

    /// <summary>Light position in world space.</summary>
    public Vec3 Position
    {
        readonly get => _position;
        set => _position = value;
    }

    /// <summary>Light color.</summary>
    public Color Color
    {
        readonly get => _color;
        set => _color = value;
    }

    /// <summary>Non-negative light intensity.</summary>
    public float Intensity
    {
        readonly get => _intensity;

        set
        {
            if (value < 0.0f)
            {
                throw new ArgumentOutOfRangeException(
                    nameof(value),
                    "Light intensity cannot be negative."
                );
            }

            _intensity = value;
        }
    }
}