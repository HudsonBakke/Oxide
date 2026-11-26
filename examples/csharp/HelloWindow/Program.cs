using System;

using Oxide.App;
using Oxide.Content;
using Oxide.Graphics;
using Oxide.Math;
using Oxide.Collision;

internal sealed class MyGame : Game
{
    private bool? _previousCollision;

    private readonly Camera _camera = new();

    private DirectionalLight _light = new(new Vec3(1.0f, -1.0f, -1.0f), Color.White, 1.0f);

    private readonly ContentManager _content = new();

    private Mesh? _parentMesh;
    private Mesh? _childMesh;

    private Material? _material;

    private Transform _parentTransform = new();

    private Transform _childTransform = new();

    protected override void Initialize()
    {
        _camera.Position = new Vec3(0.0f, 0.0f, 5.0f);

        Console.WriteLine($"Camera clips: " + $"{_camera.NearPlane} - " + $"{_camera.FarPlane}");

        _parentMesh = _content.Load<Mesh>("../../../assets/blender_cube.glb");

        _childMesh = _content.Load<Mesh>("../../../assets/blender_cube.glb");

        _material = _content.Load<Material>("../../../assets/textured_material_cube.glb");

        _parentTransform.Position = new Vec3(0.0f, 0.0f, 0.0f);

        _childTransform = new Transform();
    }

    protected override void Update(GameTime time)
    {
        float x = MathF.Sin((float)time.TotalSeconds) * 3.0f;

        _childTransform.Position = new Vec3(x, 0.0f, 0.0f);

        bool collision = Collision.Collides(_parentMesh!, _parentTransform,

                                            _childMesh!, _childTransform);

        if (_previousCollision != collision)
        {
            Console.WriteLine(collision ? "Cubes are colliding" : "Cubes are separated");

            _previousCollision = collision;
        }
    }

    protected override void Draw(Graphics graphics)
    {
        graphics.BindCamera(_camera, graphics.FullRect);

        graphics.AddLight(_light);

        graphics.DrawMesh(_parentMesh!, _material!, _parentTransform);

        graphics.DrawMesh(_childMesh!, _material!, _childTransform);
    }
}

internal static class Program
{
    private static void Main()
    {
        using var game = new MyGame();

        game.Run(new AppConfig {
            Title = "Oxide.NET Graphics Test",

            Width = 1280,

            Height = 720,
        });
    }
}