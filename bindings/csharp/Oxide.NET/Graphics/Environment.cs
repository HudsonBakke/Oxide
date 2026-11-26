using System.Runtime.InteropServices;

namespace Oxide.Graphics;

[StructLayout(LayoutKind.Sequential)]
public struct Environment
{
    public Color AmbientColor;
    public float AmbientIntensity;

    public Color SkyColor;
    public Color HorizonColor;
    public Color GroundColor;
    public float EnvironmentIntensity;

    public Color BackgroundColor;

    public Environment(
        Color ambientColor,
        float ambientIntensity,
        Color skyColor,
        Color horizonColor,
        Color groundColor,
        float environmentIntensity,
        Color backgroundColor
    )
    {
        if (!float.IsFinite(ambientIntensity) || ambientIntensity < 0.0f)
            throw new ArgumentOutOfRangeException(nameof(ambientIntensity));

        if (!float.IsFinite(environmentIntensity) || environmentIntensity < 0.0f)
            throw new ArgumentOutOfRangeException(nameof(environmentIntensity));

        AmbientColor = ambientColor;
        AmbientIntensity = ambientIntensity;
        SkyColor = skyColor;
        HorizonColor = horizonColor;
        GroundColor = groundColor;
        EnvironmentIntensity = environmentIntensity;
        BackgroundColor = backgroundColor;
    }
}
