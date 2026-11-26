using Oxide.App;
using Oxide.Graphics;
using Oxide.Input;
using Oxide.Math;

internal sealed class FirstPersonGame : Game
{
    private readonly Camera _camera = new();
    private readonly Mesh _floor = MakeQuad();
    private readonly Mesh _cube = MakeCube();
    private readonly Mesh _sphere = MakeSphere(16, 10);

    private readonly Material _green = Material.Solid(new Color(0.25f, 0.55f, 0.22f, 1.0f));
    private readonly Material _dark = Material.Solid(new Color(0.12f, 0.18f, 0.12f, 1.0f));
    private readonly Material _brown = Material.Solid(new Color(0.45f, 0.25f, 0.10f, 1.0f));
    private readonly Material _red = Material.Solid(new Color(0.65f, 0.16f, 0.10f, 1.0f));

    private readonly DirectionalLight[] _directionalLights = {
        new(new Vec3(0,-1,0), new Color(1.0f,0.75f,0.65f,1), 0.30f),
        new(new Vec3(0,1,0), new Color(0.45f,0.55f,1.0f,1), 0.18f),
        new(new Vec3(-1,0,0), new Color(1.0f,0.35f,0.30f,1), 0.22f),
        new(new Vec3(1,0,0), new Color(0.30f,1.0f,0.45f,1), 0.22f),
    };

    private readonly PointLight _sun = new(
        new Vec3(4, 4, -5),
        new Color(1.0f, 0.72f, 0.15f, 1),
        2.5f
    );

    private SpotLight _flashlight = new(
        new Vec3(-4, 1.2f, -3),
        new Vec3(0.8f, -0.15f, -1),
        Angle.Degrees(32),
        Color.White,
        5.0f
    );

    private readonly Material _yellow = Material.Solid(new Color(1.0f,0.75f,0.10f,1));
    private readonly Material _gray = Material.Solid(new Color(0.22f,0.24f,0.28f,1));

    private float _yaw;
    private float _pitch;
    private bool _captured;

    protected override void Initialize()
    {
        _camera.Position = new Vec3(0.0f, 1.7f, 7.0f);
        SetWindowName("Oxide First Person Camera");
    }

    protected override void Update(GameTime time)
    {
        KeyboardState keyboard = Keyboard.GetState();
        MouseState mouse = Mouse.GetState();

        if (mouse.IsButtonPressed(MouseButtons.Left) && !_captured)
        {
            _captured = true;
            SetCursorVisible(false);
            SetCursorGrabbed(true);
        }

        if (keyboard.IsKeyPressed(Keys.Escape))
        {
            if (_captured)
            {
                _captured = false;
                SetCursorGrabbed(false);
                SetCursorVisible(true);
            }
            else
            {
                Exit();
            }

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

        if (_captured)
        {
            _yaw -= mouse.Delta.X * 0.0025f;
            _pitch = Math.Clamp(
                _pitch - mouse.Delta.Y * 0.0025f,
                -1.5f,
                1.5f
            );
        }

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
        if (keyboard.IsKeyDown(Keys.Space)) movement += new Vec3(0,1,0);
        if (keyboard.IsKeyDown(Keys.LeftShift) || keyboard.IsKeyDown(Keys.RightShift)) movement -= new Vec3(0,1,0);

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
        foreach (DirectionalLight light in _directionalLights)
            graphics.AddLight(light);
        graphics.AddLight(_sun);
        Quaternion cameraRotation = _camera.Rotation;
        Vec3 cameraPosition = _camera.Position;
        Vec3 forward3d = cameraRotation.RotateVector(new Vec3(0,0,-1));
        Vec3 right3d = cameraRotation.RotateVector(new Vec3(1,0,0));
        Vec3 down3d = cameraRotation.RotateVector(new Vec3(0,-1,0));

        _flashlight.Position = cameraPosition + forward3d * 0.7f + right3d * 0.35f + down3d * 0.25f;
        _flashlight.Direction = forward3d;
        graphics.AddLight(_flashlight);

        for (int x = -5; x <= 5; x++)
        {
            for (int z = -5; z <= 5; z++)
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

        DrawBox(graphics, _sphere, _yellow, new Vec3(4,4,-5), new Vec3(1.1f,1.1f,1.1f));
        DrawOrientedBox(graphics, _cube, _gray,
            cameraPosition + forward3d * 0.9f + right3d * 0.35f + down3d * 0.25f,
            new Vec3(0.22f,0.22f,0.8f), cameraRotation);
        DrawOrientedBox(graphics, _cube, _yellow,
            cameraPosition + forward3d * 1.3f + right3d * 0.35f + down3d * 0.25f,
            new Vec3(0.28f,0.28f,0.12f), cameraRotation);

        DrawBox(graphics, _cube, _brown, new Vec3(0, 1.5f, -6), new Vec3(0.22f, 3, 0.22f));
        DrawBox(graphics, _cube, _red, new Vec3(0, 2.8f, -6), new Vec3(1.4f, 1.3f, 0.55f));
        DrawBox(graphics, _cube, _brown, new Vec3(0, 3.9f, -6), new Vec3(0.8f, 0.8f, 0.8f));
        DrawBox(graphics, _cube, _brown, new Vec3(0, 3.0f, -6), new Vec3(4.0f, 0.18f, 0.18f));
        DrawBox(graphics, _cube, _dark, new Vec3(0, 4.45f, -6), new Vec3(1.7f, 0.15f, 1.2f));
        DrawBox(graphics, _cube, _dark, new Vec3(0, 4.75f, -6), new Vec3(0.9f, 0.55f, 0.8f));
    }

    private static void DrawOrientedBox(Graphics graphics, Mesh mesh, Material material, Vec3 position, Vec3 scale, Quaternion rotation)
    {
        Transform transform = new() { Position = position, Scale = scale, Rotation = rotation };
        graphics.DrawMesh(mesh, material, transform);
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
            new uint[] { 0,1,2, 0,2,3 }
        );
    }


    private static Mesh MakeSphere(int segments, int rings)
    {
        var vertices = new List<Vertex>();
        var indices = new List<uint>();

        for (int ring = 0; ring <= rings; ring++)
        {
            float v = (float)ring / rings;
            float phi = v * MathF.PI;
            float y = MathF.Cos(phi);
            float radius = MathF.Sin(phi);

            for (int segment = 0; segment <= segments; segment++)
            {
                float u = (float)segment / segments;
                float theta = u * MathF.Tau;
                Vec3 normal = new(radius * MathF.Cos(theta), y, radius * MathF.Sin(theta));
                vertices.Add(new Vertex(normal * 0.5f, normal, new Vec2(u, v)));
            }
        }

        uint stride = (uint)segments + 1;
        for (uint ring = 0; ring < rings; ring++)
        for (uint segment = 0; segment < segments; segment++)
        {
            uint a = ring * stride + segment;
            uint b = a + stride;
            indices.AddRange(new[] { a,a+1,b+1, a,b+1,b });
        }

        return new Mesh(vertices.ToArray(), indices.ToArray());
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
