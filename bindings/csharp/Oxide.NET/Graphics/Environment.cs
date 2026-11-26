namespace Oxide.Graphics;
/// <summary>Independent ambient, image-based lighting, and equirectangular background settings.</summary>
public sealed class Environment
{
    /// <summary>RGB color of constant ambient lighting.</summary>
    public Color AmbientColor { get; }
    /// <summary>Finite, non-negative multiplier for constant ambient lighting.</summary>
    public float AmbientIntensity { get; }
    /// <summary>Optional equirectangular texture used for diffuse and specular image-based lighting.</summary>
    public Texture? EnvironmentTexture { get; }
    /// <summary>Finite, non-negative multiplier for image-based lighting.</summary>
    public float EnvironmentIntensity { get; }
    /// <summary>Optional equirectangular texture displayed behind geometry, independent of lighting.</summary>
    public Texture? BackgroundTexture { get; }
    /// <summary>Clear color used when no background texture is displayed.</summary>
    public Color BackgroundColor { get; }
    /// <summary>Creates explicit lighting and background settings; intensity multipliers must be finite and non-negative.</summary>
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
