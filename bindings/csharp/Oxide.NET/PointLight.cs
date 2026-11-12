using System;
using System.Runtime.InteropServices;

namespace Oxide;

[StructLayout(LayoutKind.Sequential)]
public readonly struct PointLight
{
    public readonly Vec3 Position;
    public readonly Color Color;
    public readonly float Intensity;

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

        Position = position;
        Color = color;
        Intensity = intensity;
    }
}