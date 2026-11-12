using System;
using System.Runtime.InteropServices;

namespace Oxide;

[StructLayout(LayoutKind.Sequential)]
public readonly struct SpotLight
{
    public readonly Vec3 Position;
    public readonly Vec3 Direction;
    public readonly Angle Angle;

    public readonly Color Color;
    public readonly float Intensity;

    public SpotLight(
        Vec3 position,
        Vec3 direction,
        Angle angle,
        Color color,
        float intensity
    )
    {
        float length =
            MathF.Sqrt(
                direction.X * direction.X
                + direction.Y * direction.Y
                + direction.Z * direction.Z
            );

        if (length <= 1.0e-6f)
        {
            throw new ArgumentException(
                "Spot light direction cannot be zero.",
                nameof(direction)
            );
        }

        if (angle.AsRadians <= 0.0f
            || angle.AsRadians >= MathF.PI)
        {
            throw new ArgumentOutOfRangeException(
                nameof(angle),
                "Spot light angle must be between 0 and pi radians."
            );
        }

        if (intensity < 0.0f)
        {
            throw new ArgumentOutOfRangeException(
                nameof(intensity),
                "Light intensity cannot be negative."
            );
        }

        Position = position;

        Direction =
            new Vec3(
                direction.X / length,
                direction.Y / length,
                direction.Z / length
            );

        Angle = angle;
        Color = color;
        Intensity = intensity;
    }
}