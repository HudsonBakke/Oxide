namespace Oxide.App;

/// <summary>Frame timing in seconds; delta time varies with actual frame duration.</summary>
public readonly struct GameTime
{
    /// <summary>Active simulation step, capped at 0.1 seconds; zero on the first update after a pause.</summary>
    public double DeltaSeconds { get; }
    /// <summary>Sum of delivered simulation steps, excluding initialization, paused time, and capped stall excess.</summary>
    public double TotalSeconds { get; }

    internal GameTime(double deltaSeconds, double totalSeconds)
    {
        DeltaSeconds = deltaSeconds;

        TotalSeconds = totalSeconds;
    }
}
