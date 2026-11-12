using System;
using System.Runtime.InteropServices;

namespace Oxide;

[StructLayout(LayoutKind.Sequential)]
public readonly struct Angle
{
    private readonly float _radians;

    private Angle(
        float radians
    )
    {
        _radians = radians;
    }

    public static Angle Radians(
        float radians
    ) =>
        new(radians);

    public static Angle Degrees(
        float degrees
    ) =>
        new(
            degrees
                * MathF.PI
                / 180.0f
        );

    public float AsRadians =>
        _radians;

    public float AsDegrees =>
        _radians
        * 180.0f
        / MathF.PI;

    public static Angle Zero =>
        new(0.0f);
}