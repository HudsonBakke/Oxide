using System;
using System.Runtime.InteropServices;

using Oxide.Math;

namespace Oxide.Graphics;

[StructLayout(LayoutKind.Sequential)]
/// <summary>Infinitely distant light whose rays share one direction.</summary>
public struct DirectionalLight
{
    private Vec3 _direction;
    private Color _color;
    private float _intensity;

    /// <summary>Creates or accesses DirectionalLight.</summary>
    public DirectionalLight(
        Vec3 direction,
        Color color,
        float intensity
    )
    {
        if (direction.Length
            <= 1.0e-6f)
        {
            throw new ArgumentException(
                "Directional light direction cannot be zero.",
                nameof(direction)
            );
        }

        if (intensity < 0.0f)
        {
            throw new ArgumentOutOfRangeException(
                nameof(intensity),
                "Light intensity cannot be negative."
            );
        }

        _direction =
            direction.Normalized();

        _color = color;
        _intensity = intensity;
    }

    /// <summary>Normalized direction in which the light travels.</summary>
    public Vec3 Direction
    {
        readonly get =>
            _direction;

        set
        {
            if (value.Length
                <= 1.0e-6f)
            {
                throw new ArgumentException(
                    "Directional light direction cannot be zero.",
                    nameof(value)
                );
            }

            _direction =
                value.Normalized();
        }
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