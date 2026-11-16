using System;
using System.Runtime.InteropServices;

using Oxide.Math;

namespace Oxide.Graphics;

[StructLayout(LayoutKind.Sequential)]
public struct PointLight
{
    private Vec3 _position;
    private Color _color;
    private float _intensity;

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

    public Vec3 Position
    {
        readonly get => _position;
        set => _position = value;
    }

    public Color Color
    {
        readonly get => _color;
        set => _color = value;
    }

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