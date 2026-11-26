namespace Oxide.App;

/// <summary>Configures the application window used when a game starts.</summary>
public sealed class AppConfig
{
    public string Title { get; set; } =
        "Oxide";

    public uint Width { get; set; } =
        1280;

    public uint Height { get; set; } =
        720;
}