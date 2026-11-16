using Oxide.App;
using Oxide.Content;
using Oxide.Graphics;
using Oxide.Math;

internal sealed class MyGame
    : Game
{
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
    private readonly ContentManager
        _content =
            new(
                "test-csharp-cache"
            );

    private Mesh? _mesh;
    private Mesh? _testMesh;
    private Material? _material;

    private Transform _transform =
        new();

    protected override void Initialize()
    {
        Console.WriteLine(
            $"Content cache: " +
            $"{_content.CacheDirectory}"
        );

        _camera.Position =
            new Vec3(
                0.0f,
                0.0f,
                5.0f
            );

        _mesh =
            _content.Load<Mesh>(
                "../../../assets/textured_material_cube.glb"
            );

        _material =
            _content.Load<Material>(
                "../../../assets/textured_material_cube.glb"
            );

        Texture brick =
            _content.Load<Texture>(
                "../../../assets/brick.jpg"
            );

        Console.WriteLine(
            $"Brick texture: " +
            $"{brick.Width}x{brick.Height}"
        );

        _testMesh =
            new Mesh(
                [
                    new(
                        new Vec3(
                            -1.0f,
                            -1.0f,
                            0.0f
                        ),
                        new Vec3(
                            0.0f,
                            0.0f,
                            1.0f
                        ),
                        new Vec2(
                            0.0f,
                            1.0f
                        )
                    ),

                    new(
                        new Vec3(
                            0.0f,
                            1.0f,
                            0.0f
                        ),
                        new Vec3(
                            0.0f,
                            0.0f,
                            1.0f
                        ),
                        new Vec2(
                            0.5f,
                            0.0f
                        )
                    ),

                    new(
                        new Vec3(
                            1.0f,
                            -1.0f,
                            0.0f
                        ),
                        new Vec3(
                            0.0f,
                            0.0f,
                            1.0f
                        ),
                        new Vec2(
                            1.0f,
                            1.0f
                        )
                    ),
                ],

                [
                    0,
                    1,
                    2,
                ]
            );

        _camera.Position =
            new Vec3(
                0.0f,
                0.0f,
                5.0f
            );
    }

    protected override void Update(
        GameTime time
    )
    {
        float delta =
            (float)time.DeltaSeconds;

        _transform.RotateYawBy(
            Angle.Radians(
                0.8f * delta
            )
        );

        _transform.RotatePitchBy(
            Angle.Radians(
                0.35f * delta
            )
        );
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

        graphics.DrawMesh(
            _mesh!,
            _material!,
            _transform
        );

        graphics.DrawMesh(
            _testMesh!,
            _material!,
            _transform
        );
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