namespace Oxide.App;

/// <summary>Frame timing in seconds; delta time varies with actual frame duration.</summary>
public readonly struct GameTime
{
    /// <summary>Seconds elapsed since the previous frame's update.</summary>
    public double DeltaSeconds { get; }
    /// <summary>Seconds elapsed since the application clock started.</summary>
    public double TotalSeconds { get; }

    internal GameTime(double deltaSeconds, double totalSeconds)
    {
        DeltaSeconds = deltaSeconds;

        TotalSeconds = totalSeconds;
    }
}
