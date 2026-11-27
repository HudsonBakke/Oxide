namespace Oxide.Graphics;

/// <summary>Shared color, intensity, and shadow configuration for point, spot, and directional lights.</summary>
public interface ILightSource
{
    /// <summary>RGB light color; alpha is unused.</summary>
    Color Color { get; set; }
    /// <summary>Non-negative light intensity.</summary>
    float Intensity { get; set; }
    /// <summary>Enables shadows with automatic default settings.</summary>
    bool CastsShadows { get; set; }
    /// <summary>Optional shadow quality and coverage overrides.</summary>
    ShadowSettings ShadowSettings { get; set; }
}
