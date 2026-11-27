using Oxide.App;
using Oxide.Content;
using Oxide.Graphics;
using Oxide.Input;
using Oxide.Math;

internal sealed class Shadows : Game
{
    private readonly Camera _camera = new();
    private readonly Mesh _mesh = Cube();
    private readonly Material[] _materials = [
        Material.Solid(new Color(0.55f, 0.58f, 0.62f, 1)),
        Material.Solid(new Color(0.8f, 0.18f, 0.06f, 1)),
        Material.Solid(new Color(0.08f, 0.35f, 0.8f, 1)),
        Material.Solid(new Color(0.2f, 0.8f, 0.2f, 1)),
        Material.Solid(new Color(0.15f, 0.7f, 0.8f, 1)),
        Material.Solid(new Color(0.15f, 0.7f, 0.8f, 1)),
        Material.Solid(new Color(0.95f, 0.75f, 0.05f, 1)),
    ];
    private readonly Transform[] _transforms = [
        new() { Position = new Vec3(0, -0.1f, 0), Scale = new Vec3(16, 0.2f, 16) },
        new() { Position = new Vec3(0, 1.5f, 0), Scale = new Vec3(2, 2, 2) },
        new() { Position = new Vec3(3, 1.5f, -2.5f), Scale = new Vec3(1.5f, 3, 1.5f) },
        new() { Position = new Vec3(-3.5f, 1, -3), Scale = new Vec3(1.5f, 2, 1.5f) },
        new() { Position = new Vec3(-3, 3, 2), Scale = new Vec3(2, 0.1f, 2) },
        new() { Position = new Vec3(-2.5f, 4, 2.35f), Scale = new Vec3(2, 0.1f, 2) },
        new() { Position = new Vec3(-1.2f, 1, -1.8f), Scale = new Vec3(0.8f, 0.8f, 0.8f) },
    ];
    private readonly ILightSource[] _lights = [
        new DirectionalLight(new Vec3(-0.5f, -1, -0.35f), Color.White, 3, castsShadows: true),
        new PointLight(new Vec3(2, 7, 4), Color.White, 100, castsShadows: true),
        new SpotLight(new Vec3(0, 8, 4), new Vec3(0, -1, -0.5f), Angle.Degrees(45), Color.White, 100, castsShadows: true),
    ];
    private int _lightMode;
    private readonly Oxide.Graphics.Environment _environment =
        new(Color.White, 0.12f, null, 0, null, new Color(0.04f, 0.06f, 0.09f, 1));
    private float _transparency = 0.5f;

    public Shadows()
    {
        Vec3 position = new(9, 8, 13);
        _camera.Position = position;
        _camera.Rotation = Quaternion.FromTo(new Vec3(0, 0, -1), new Vec3(0, 0.5f, 0) - position);
    }

    protected override void Update(GameTime time)
    {
        float t = (float)time.TotalSeconds;
        _transforms[1].Position = new Vec3(MathF.Sin(t) * 2.5f, 1.5f + MathF.Cos(t) * 0.4f, 0);
        _transforms[1].Rotation = Quaternion.FromAxisAngle(new Vec3(0, 1, 0), Angle.Radians(t * 0.5f));
        if (Keyboard.GetState().IsKeyPressed(Keys.Space)) _lights[_lightMode].CastsShadows = !_lights[_lightMode].CastsShadows;
        if (Keyboard.GetState().IsKeyPressed(Keys.T)) _transparency = _transparency < 1 ? _transparency + 0.5f : 0;
        if (Keyboard.GetState().IsKeyPressed(Keys.L)) _lightMode = (_lightMode + 1) % _lights.Length;
        SetWindowName($"Oxide Shadows | {(_lights[_lightMode].CastsShadows ? "on" : "off")} | box transparency {_transparency} | Space shadows, T transparency, L light {_lightMode}");
    }

    protected override void Draw(Graphics graphics)
    {
        ILightSource light = _lights[_lightMode];
        graphics.BindCamera(_camera, graphics.FullRect);
        graphics.SetEnvironment(_environment);
        graphics.AddLight(light);
        float[] transmission = [0, _transparency, 0, 0.75f, 0.5f, 0.5f, 0];
        for (int i = 0; i < _transforms.Length; i++)
        {
            if (transmission[i] == 0) graphics.DrawMesh(_mesh, _materials[i], _transforms[i]);
            else graphics.DrawMesh(_mesh, _materials[i], _transforms[i], transmission[i]);
        }
    }

    private static Mesh Cube()
    {
        var vertices = new List<Vertex>();
        var indices = new List<uint>();
        (Vec3 normal, Vec3 right, Vec3 up)[] faces = [
            (new(0, 0, 1), new(1, 0, 0), new(0, 1, 0)),
            (new(0, 0, -1), new(-1, 0, 0), new(0, 1, 0)),
            (new(1, 0, 0), new(0, 0, -1), new(0, 1, 0)),
            (new(-1, 0, 0), new(0, 0, 1), new(0, 1, 0)),
            (new(0, 1, 0), new(1, 0, 0), new(0, 0, -1)),
            (new(0, -1, 0), new(1, 0, 0), new(0, 0, 1)),
        ];
        foreach (var (normal, right, up) in faces)
        {
            uint start = (uint)vertices.Count;
            foreach (var (x, y) in new (float, float)[] { (-1, -1), (1, -1), (1, 1), (-1, 1) })
                vertices.Add(new Vertex((normal + right * x + up * y) * 0.5f,
                                        normal, new Vec2((x + 1) * 0.5f, (y + 1) * 0.5f)));
            indices.AddRange([start, start + 2, start + 1, start, start + 3, start + 2]);
        }
        return new Mesh(vertices.ToArray(), indices.ToArray());
    }
}

internal static class Program
{
    private static void Main(string[] args)
    {
        if (args.Length == 1 && args[0] == "--verify-lights")
        {
            if (System.Runtime.InteropServices.Marshal.SizeOf<ShadowSettings>() != 40 ||
                System.Runtime.InteropServices.Marshal.SizeOf<PointLight>() != 76 ||
                System.Runtime.InteropServices.Marshal.SizeOf<DirectionalLight>() != 76 ||
                System.Runtime.InteropServices.Marshal.SizeOf<SpotLight>() != 92)
                throw new InvalidOperationException("Native light layout mismatch");
            ILightSource[] lights = [
                new PointLight(new Vec3(0, 2, 0), Color.White, 1),
                new DirectionalLight(new Vec3(0, -1, 0), Color.White, 1),
                new SpotLight(new Vec3(0, 2, 0), new Vec3(0, -1, 0), Angle.Degrees(30), Color.White, 1),
            ];
            foreach (ILightSource light in lights)
            {
                if (light.CastsShadows || light.ShadowSettings.Resolution != 1024) throw new InvalidOperationException("Light defaults mismatch");
                light.CastsShadows = true;
                light.ShadowSettings = new ShadowSettings { Resolution = 256 };
                if (!light.CastsShadows || light.ShadowSettings.Resolution != 256) throw new InvalidOperationException("Light interface state mismatch");
            }
            PointLight zero = default;
            zero.CastsShadows = true;
            if (zero.ShadowSettings.Resolution != 1024) throw new InvalidOperationException("Default value shadow settings missing");
            return;
        }
        using var game = new Shadows();
        game.Run(new AppConfig { Title = "Oxide Shadows", Width = 1280, Height = 800 });
    }
}
