using Oxide.Graphics;
using Oxide.Math;

internal static class Program
{
    private static void Main()
    {
        AudioChecks.Run();
        MonitorChecks.Run();
        SpriteChecks.Run();
        LifecycleChecks.Run();
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
    }
}
