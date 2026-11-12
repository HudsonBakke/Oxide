using System;
using System.Runtime.InteropServices;

namespace Oxide;

[StructLayout(LayoutKind.Sequential)]
public readonly struct DirectionalLight
{
    public readonly Vec3 Direction;
    public readonly Color Color;
    public readonly float Intensity;

    public DirectionalLight(
        Vec3 direction,
        Color color,
        float intensity
    )
    {
        if (Length(direction)
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

        Direction =
            Normalize(direction);

        Color = color;
        Intensity = intensity;
    }

    private static float Length(
        Vec3 value
    )
    {
        return MathF.Sqrt(
            value.X * value.X
            + value.Y * value.Y
            + value.Z * value.Z
        );
    }

    private static Vec3 Normalize(
        Vec3 value
    )
    {
        float length =
            Length(value);

        return new Vec3(
            value.X / length,
            value.Y / length,
            value.Z / length
        );
    }
}