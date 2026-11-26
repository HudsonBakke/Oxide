namespace Oxide.Graphics;

/// <summary>Explicit ambient, image-based-lighting, and background settings for a scene.</summary>
public sealed class Environment
{
    /// <summary>Color of the constant ambient-light contribution.</summary>
    public Color AmbientColor { get; }
    /// <summary>Non-negative multiplier for ambient lighting.</summary>
    public float AmbientIntensity { get; }
    /// <summary>Optional equirectangular texture used for image-based lighting.</summary>
    public Texture? EnvironmentTexture { get; }
    /// <summary>Non-negative multiplier for image-based lighting.</summary>
    public float EnvironmentIntensity { get; }
    /// <summary>Optional equirectangular texture rendered as the visible background.</summary>
    public Texture? BackgroundTexture { get; }
    /// <summary>Fallback clear color used when no background texture is supplied.</summary>
    public Color BackgroundColor { get; }

    /// <summary>Initializes or accesses Environment.</summary>
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
