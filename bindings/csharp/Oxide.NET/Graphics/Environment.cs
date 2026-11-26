namespace Oxide.Graphics;

public sealed class Environment
{
    public Color AmbientColor { get; }
    public float AmbientIntensity { get; }
    public Texture? EnvironmentTexture { get; }
    public float EnvironmentIntensity { get; }
    public Texture? BackgroundTexture { get; }
    public Color BackgroundColor { get; }

    public Environment(
        Color ambientColor,
        float ambientIntensity,
        Texture? environmentTexture,
        float environmentIntensity,
        Texture? backgroundTexture,
        Color backgroundColor
    )
    {
        if (!float.IsFinite(ambientIntensity) || ambientIntensity < 0.0f)
            throw new ArgumentOutOfRangeException(nameof(ambientIntensity));
        if (!float.IsFinite(environmentIntensity) || environmentIntensity < 0.0f)
            throw new ArgumentOutOfRangeException(nameof(environmentIntensity));

        AmbientColor = ambientColor;
        AmbientIntensity = ambientIntensity;
        EnvironmentTexture = environmentTexture;
        EnvironmentIntensity = environmentIntensity;
        BackgroundTexture = backgroundTexture;
        BackgroundColor = backgroundColor;
    }
}
