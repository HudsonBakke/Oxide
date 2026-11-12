namespace Oxide;

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