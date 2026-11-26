namespace Oxide.App;

/// <summary>Timing information supplied to each update callback.</summary>
public readonly struct GameTime
{
    /// <summary>Seconds elapsed since the previous update.</summary>
    public double DeltaSeconds { get; }

    /// <summary>Seconds elapsed since the game started.</summary>
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