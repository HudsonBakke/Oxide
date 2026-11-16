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

    private Mesh? _parentMesh;
    private Mesh? _childMesh;
    private Material? _material;

    private Transform _parentTransform =
        new();

    private Transform _childTransform =
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

        _parentMesh =
            _content.Load<Mesh>(
                "../../../assets/blender_cube.glb"
            );

        _childMesh =
            _content.Load<Mesh>(
                "../../../assets/blender_cube.glb"
            );

        _material = _content.Load<Material>
            ("../../../assets/textured_material_cube.glb");

        _parentTransform.Position =
            new Vec3(
                -1.0f,
                0.0f,
                0.0f
            );

        _childTransform =
            Transform.FromAttachment(
                _parentMesh,
                _parentTransform,

                new Vec3(
                    1.0f,
                    0.0f,
                    0.0f
                ),

                _childMesh,

                new Vec3(
                    -1.0f,
                    0.0f,
                    0.0f
                ),

                new Vec3(
                    1.0f,
                    0.0f,
                    0.0f
                ),

                Angle.Degrees(45.0f),

                new Vec3(
                    1.0f,
                    1.0f,
                    1.0f
                )
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
            _parentMesh!,
            _material!,
            _parentTransform
        );

        graphics.DrawMesh(
            _childMesh!,
            _material!,
            _childTransform
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