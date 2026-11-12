using Oxide;

internal sealed class MyGame
    : Game,
      IDisposable
{
    private bool _printedUpdate;
    private bool _printedDraw;
    private readonly Camera _camera =
        new();
    private readonly DirectionalLight _light =
        new(
            new Vec3(
                1.0f,
                -1.0f,
                -1.0f
            ),
            Color.White,
            1.0f
        );

    protected override void Initialize()
    {
        _camera.Position =
            new Vec3(
                0.0f,
                0.0f,
                5.0f
            );

        Console.WriteLine(
            "C# Initialize!"
        );
    }

    protected override void Update(
        GameTime time
    )
    {
        if (_printedUpdate)
        {
            return;
        }

        Console.WriteLine(
            $"C# Update! " +
            $"delta = {time.DeltaSeconds:F6}, " +
            $"total = {time.TotalSeconds:F6}"
        );

        _printedUpdate = true;
    }

    protected override void Draw(
        Graphics graphics
    )
    {
        graphics.BindCamera(
            _camera,
            graphics.FullRect
        );

        graphics.AddLight(
            _light
        );

        if (_printedDraw)
        {
            return;
        }

        Console.WriteLine(
            "C# camera and light successfully bound!"
        );

        _printedDraw = true;
    }

    public void Dispose()
    {
        _camera.Dispose();
    }
}

internal static class Program
{
    private static void Main()
    {
        using var game =
            new MyGame();

        game.Run(
            new AppConfig {
                Title =
                    "Oxide.NET Camera Test",

                Width =
                    1280,

                Height =
                    720,
            }
        );
    }
}