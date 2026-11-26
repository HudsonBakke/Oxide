using Oxide.App;
using Oxide.Graphics;
using Oxide.Input;
using Oxide.Math;

internal sealed class FirstPersonGame : Game
{
    private readonly Camera _camera = new();
    private readonly Mesh _floor = MakeQuad();
    private readonly Mesh _cube = MakeCube();

    private readonly Material _green = Material.Solid(new Color(0.25f, 0.55f, 0.22f, 1.0f));
    private readonly Material _dark = Material.Solid(new Color(0.12f, 0.18f, 0.12f, 1.0f));
    private readonly Material _brown = Material.Solid(new Color(0.45f, 0.25f, 0.10f, 1.0f));
    private readonly Material _red = Material.Solid(new Color(0.65f, 0.16f, 0.10f, 1.0f));

    private readonly DirectionalLight _sun = new(
        new Vec3(-0.5f, -1.0f, -0.35f),
        Color.White,
        1.25f
    );

    private float _yaw;
    private float _pitch;

    protected override void Initialize()
    {
        _camera.Position = new Vec3(0.0f, 1.7f, 7.0f);
        SetWindowName("Oxide First Person Camera");
        SetCursorVisible(false);
        SetCursorGrabbed(true);
    }

    protected override void Update(GameTime time)
    {
        KeyboardState keyboard = Keyboard.GetState();
        MouseState mouse = Mouse.GetState();

        if (keyboard.IsKeyPressed(Keys.Escape))
        {
            Exit();
            return;
        }

        if (keyboard.IsKeyPressed(Keys.F11))
        {
            IsFullscreen = !IsFullscreen;
        }

        // Demonstrates that resizing is legal from Update().
        if (keyboard.IsKeyPressed(Keys.F2))
        {
            SetPreferredBackbufferSize(1920, 1080);
        }

        _yaw -= mouse.Delta.X * 0.0025f;
        _pitch = Math.Clamp(
            _pitch - mouse.Delta.Y * 0.0025f,
            -1.5f,
            1.5f
        );

        _camera.SetEuler(
            Angle.Radians(_pitch),
            Angle.Radians(_yaw),
            Angle.Zero
        );

        Quaternion rotation = Quaternion.FromEuler(
            Angle.Zero,
            Angle.Radians(_yaw),
            Angle.Zero
        );

        Vec3 forward = rotation.RotateVector(new Vec3(0.0f, 0.0f, -1.0f));
        Vec3 right = rotation.RotateVector(new Vec3(1.0f, 0.0f, 0.0f));

        Vec3 movement = new();
        if (keyboard.IsKeyDown(Keys.W)) movement += forward;
        if (keyboard.IsKeyDown(Keys.S)) movement -= forward;
        if (keyboard.IsKeyDown(Keys.D)) movement += right;
        if (keyboard.IsKeyDown(Keys.A)) movement -= right;

        if (movement.Length > 0.0f)
        {
            _camera.TranslateBy(
                movement.Normalized() * (5.0f * (float)time.DeltaSeconds)
            );
        }
    }

    protected override void Draw(Graphics graphics)
    {
        graphics.BindCamera(_camera, graphics.FullRect);
        graphics.AddLight(_sun);

        for (int x = -10; x <= 10; x++)
        {
            for (int z = -10; z <= 10; z++)
            {
                DrawBox(
                    graphics,
                    _floor,
                    ((x + z) & 1) == 0 ? _green : _dark,
                    new Vec3(x * 2.0f, 0.0f, z * 2.0f),
                    new Vec3(2.0f, 1.0f, 2.0f)
                );
            }
        }

        DrawBox(graphics, _cube, _brown, new Vec3(0, 1.5f, -6), new Vec3(0.22f, 3, 0.22f));
        DrawBox(graphics, _cube, _red, new Vec3(0, 2.8f, -6), new Vec3(1.4f, 1.3f, 0.55f));
        DrawBox(graphics, _cube, _brown, new Vec3(0, 3.9f, -6), new Vec3(0.8f, 0.8f, 0.8f));
        DrawBox(graphics, _cube, _brown, new Vec3(0, 3.0f, -6), new Vec3(4.0f, 0.18f, 0.18f));
        DrawBox(graphics, _cube, _dark, new Vec3(0, 4.45f, -6), new Vec3(1.7f, 0.15f, 1.2f));
        DrawBox(graphics, _cube, _dark, new Vec3(0, 4.75f, -6), new Vec3(0.9f, 0.55f, 0.8f));
    }

    private static void DrawBox(Graphics graphics, Mesh mesh, Material material, Vec3 position, Vec3 scale)
    {
        Transform transform = new() {
            Position = position,
            Scale = scale,
        };
        graphics.DrawMesh(mesh, material, transform);
    }

    private static Mesh MakeQuad()
    {
        return new Mesh(
            new[] {
                new Vertex(new Vec3(-0.5f,0,-0.5f), new Vec3(0,1,0), new Vec2(0,0)),
                new Vertex(new Vec3( 0.5f,0,-0.5f), new Vec3(0,1,0), new Vec2(1,0)),
                new Vertex(new Vec3( 0.5f,0, 0.5f), new Vec3(0,1,0), new Vec2(1,1)),
                new Vertex(new Vec3(-0.5f,0, 0.5f), new Vec3(0,1,0), new Vec2(0,1)),
            },
            new uint[] { 0,2,1, 0,3,2 }
        );
    }

    private static Mesh MakeCube()
    {
        var vertices = new List<Vertex>();
        var indices = new List<uint>();

        void Face(Vec3 n, params Vec3[] p)
        {
            uint b = (uint)vertices.Count;
            foreach (Vec3 v in p)
                vertices.Add(new Vertex(v * 0.5f, n, new Vec2()));
            indices.AddRange(new[] { b,b+2,b+1, b,b+3,b+2 });
        }

        Face(new Vec3(0,0,1), new Vec3(-1,-1,1),new Vec3(1,-1,1),new Vec3(1,1,1),new Vec3(-1,1,1));
        Face(new Vec3(0,0,-1), new Vec3(1,-1,-1),new Vec3(-1,-1,-1),new Vec3(-1,1,-1),new Vec3(1,1,-1));
        Face(new Vec3(1,0,0), new Vec3(1,-1,1),new Vec3(1,-1,-1),new Vec3(1,1,-1),new Vec3(1,1,1));
        Face(new Vec3(-1,0,0), new Vec3(-1,-1,-1),new Vec3(-1,-1,1),new Vec3(-1,1,1),new Vec3(-1,1,-1));
        Face(new Vec3(0,1,0), new Vec3(-1,1,1),new Vec3(1,1,1),new Vec3(1,1,-1),new Vec3(-1,1,-1));
        Face(new Vec3(0,-1,0), new Vec3(-1,-1,-1),new Vec3(1,-1,-1),new Vec3(1,-1,1),new Vec3(-1,-1,1));

        return new Mesh(vertices.ToArray(), indices.ToArray());
    }
}

internal static class Program
{
    private static void Main()
    {
        using var game = new FirstPersonGame();
        game.Run(new AppConfig {
            Title = "Oxide First Person Camera",
            Width = 1280,
            Height = 720,
        });
    }
}
