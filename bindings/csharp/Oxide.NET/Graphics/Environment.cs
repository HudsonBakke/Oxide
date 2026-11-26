using System.Runtime.InteropServices;

namespace Oxide.Graphics;

[StructLayout(LayoutKind.Sequential)]
public struct Environment
{
    public Color SkyColor;
    public Color HorizonColor;
    public Color GroundColor;
    public float Intensity;

    public Environment(
        Color skyColor,
        Color horizonColor,
        Color groundColor,
        float intensity = 1.0f
    )
    {
        if (float.IsNaN(intensity) || float.IsInfinity(intensity) || intensity < 0.0f)
            throw new ArgumentOutOfRangeException(nameof(intensity));

        SkyColor = skyColor;
        HorizonColor = horizonColor;
        GroundColor = groundColor;
        Intensity = intensity;
    }

    public static Environment Daylight =>
        new(
            new Color(0.28f, 0.48f, 0.90f, 1.0f),
            new Color(0.72f, 0.78f, 0.88f, 1.0f),
            new Color(0.16f, 0.13f, 0.10f, 1.0f),
            0.65f
        );
}
