namespace Oxide.App;

/// <summary>Timing information supplied to each update callback.</summary>
public readonly struct GameTime
{
    public double DeltaSeconds { get; }

    public double TotalSeconds { get; }

    internal GameTime(
        double deltaSeconds,
        double totalSeconds
    )
    {
        DeltaSeconds =
            deltaSeconds;

        TotalSeconds =
            totalSeconds;
    }
}