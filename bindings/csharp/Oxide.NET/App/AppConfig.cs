namespace Oxide.App;

/// <summary>Configures the application window used when a game starts.</summary>
public sealed class AppConfig
{
    /// <summary>Initial window title.</summary>
    public string Title { get; set; } =
        "Oxide";

    /// <summary>Initial window width in pixels.</summary>
    public uint Width { get; set; } =
        1280;

    /// <summary>Initial window height in pixels.</summary>
    public uint Height { get; set; } =
        720;
}