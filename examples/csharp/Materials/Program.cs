using Oxide.App;
using Oxide.Graphics;
using Oxide.Math;

internal sealed class MaterialTest : Game
{
    private const int Steps = 6;
    private readonly Camera _camera = new();
    private readonly Mesh _sphere = MakeSphere(24, 16);
    private readonly List<Material> _materials = new();
    private readonly Material _normalMapped = Material.Solid(new Color(0.32f,0.42f,0.72f,1));
    private readonly Material _emissive = Material.Solid(new Color(0.08f,0.08f,0.08f,1));
    private readonly PointLight _light = new(new Vec3(-2.5f,5.5f,5), Color.White, 65);
    private readonly Oxide.Graphics.Environment _environment;
    private readonly DirectionalLight _fill = new(
        new Vec3(0.4f,-1,-0.3f),
        new Color(0.35f,0.42f,0.55f,1),
        0.18f
    );

    public MaterialTest()
    {
        _environment = new Oxide.Graphics.Environment(
            Color.White,
            0.08f,
            EnvironmentTexture(),
            0.65f,
            EnvironmentTexture(),
            new Color(0.10f,0.15f,0.25f,1)
        );
        _normalMapped.Roughness = 0.55f;
        _normalMapped.NormalTexture = ProceduralNormalMap();
        _normalMapped.NormalStrength = 1.0f;
        for (int roughness = 0; roughness < Steps; roughness++)
        for (int metallic = 0; metallic < Steps; metallic++)
        {
            Material material = Material.Solid(new Color(0.72f,0.24f,0.12f,1));
            material.Metallic = (float)metallic / (Steps - 1);
            material.Roughness = (float)roughness / (Steps - 1);
            _materials.Add(material);
        }

        _emissive.EmissiveColor = new Color(0.1f,0.65f,1.0f,1);
        _emissive.EmissiveStrength = 4.0f;
    }

    protected override void Initialize()
    {
        _camera.Position = new Vec3(0,0,15.5f);
    }

    protected override void Draw(Graphics graphics)
    {
        graphics.BindCamera(_camera, graphics.FullRect);
        graphics.SetEnvironment(_environment);
        graphics.AddLight(_light);
        graphics.AddLight(_fill);

        const float spacing = 2.15f;
        const float center = (Steps - 1) * 0.5f;

        for (int row = 0; row < Steps; row++)
        for (int column = 0; column < Steps; column++)
        {
            Transform transform = new() {
                Position = new Vec3(
                    (column - center) * spacing,
                    (center - row) * spacing,
                    0
                ),
                Scale = new Vec3(1.65f,1.65f,1.65f),
            };
            graphics.DrawMesh(
                _sphere,
                _materials[row * Steps + column],
                transform
            );
        }

        graphics.DrawMesh(
            _sphere,
            _normalMapped,
            new Transform {
                Position = new Vec3(-3,-7,0),
                Scale = new Vec3(1.8f,1.8f,1.8f),
            }
        );

        graphics.DrawMesh(
            _sphere,
            _emissive,
            new Transform {
                Position = new Vec3(3,-7,0),
                Scale = new Vec3(1.8f,1.8f,1.8f),
            }
        );
    }



    private static Texture EnvironmentTexture()
    {
        const int width = 256, height = 128;
        byte[] pixels = new byte[width * height * 4];
        for (int y = 0; y < height; y++)
        for (int x = 0; x < width; x++)
        {
            float v = (float)y / (height - 1);
            float u = (float)x / (width - 1);
            float horizon = MathF.Pow(1 - MathF.Abs(v - 0.5f) * 2, 3);
            float sun = MathF.Exp(-((u-0.72f)*(u-0.72f)+(v-0.32f)*(v-0.32f))/0.0015f);
            float r = v < 0.5f ? 0.12f+0.28f*horizon : 0.12f+0.20f*horizon;
            float g = v < 0.5f ? 0.28f+0.38f*horizon : 0.10f+0.18f*horizon;
            float b = v < 0.5f ? 0.62f+0.28f*horizon : 0.08f+0.18f*horizon;
            int i=(y*width+x)*4;
            pixels[i]=(byte)(Math.Clamp(r+sun*0.85f,0,1)*255);
            pixels[i+1]=(byte)(Math.Clamp(g+sun*0.72f,0,1)*255);
            pixels[i+2]=(byte)(Math.Clamp(b+sun*0.38f,0,1)*255);
            pixels[i+3]=255;
        }
        return Texture.FromRgba(width,height,pixels);
    }

    private static Texture ProceduralNormalMap()
    {
        const int size = 64;
        byte[] pixels = new byte[size * size * 4];

        for (int y = 0; y < size; y++)
        for (int x = 0; x < size; x++)
        {
            float fx = (float)x / size * MathF.Tau * 4;
            float fy = (float)y / size * MathF.Tau * 4;
            Vec3 normal = new Vec3(
                -MathF.Cos(fx) * 0.45f,
                -MathF.Cos(fy) * 0.45f,
                1
            ).Normalized();

            int i = (y * size + x) * 4;
            pixels[i] = (byte)((normal.X * 0.5f + 0.5f) * 255);
            pixels[i+1] = (byte)((normal.Y * 0.5f + 0.5f) * 255);
            pixels[i+2] = (byte)((normal.Z * 0.5f + 0.5f) * 255);
            pixels[i+3] = 255;
        }

        return Texture.FromRgba(size, size, pixels);
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
                Vec3 normal = new(
                    radius * MathF.Cos(theta),
                    y,
                    radius * MathF.Sin(theta)
                );
                vertices.Add(new Vertex(normal * 0.5f, normal, new Vec2(u,v)));
            }
        }

        uint stride = (uint)segments + 1;
        for (uint ring = 0; ring < rings; ring++)
        for (uint segment = 0; segment < segments; segment++)
        {
            uint a = ring * stride + segment;
            uint b = a + stride;
            indices.AddRange(new[] { a,b+1,a+1, a,b,b+1 });
        }

        return new Mesh(vertices.ToArray(), indices.ToArray());
    }
}

internal static class Program
{
    private static void Main()
    {
        using var game = new MaterialTest();
        game.Run(new AppConfig {
            Title = "Oxide Material Test",
            Width = 1280,
            Height = 900,
        });
    }
}
