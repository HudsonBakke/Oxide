namespace Oxide.App;

/// <summary>Initial window title and logical-pixel client size; defaults to Oxide at 1280 by 720.</summary>
public sealed class AppConfig
{
    /// <summary>Initial window title; defaults to Oxide.</summary>
    public string Title { get; set; } = "Oxide";
    /// <summary>Initial client width in logical pixels; defaults to 1280.</summary>
    public uint Width { get; set; } = 1280;
    /// <summary>Initial client height in logical pixels; defaults to 720.</summary>
    public uint Height { get; set; } = 720;
}
