using System.Runtime.InteropServices;
using Oxide.Graphics;
using Oxide.Math;
internal static class SpriteChecks
{
    [StructLayout(LayoutKind.Sequential)]
    private struct Options { public Color Tint; public Rect Source; public Angle Rotation; public Vec2 Origin; public Rect Clip; public uint Flags; }
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    private static extern int oxide_draw_2d_options_default(out Options options);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    private static extern int oxide_graphics_draw_2d_with_options(IntPtr graphics, ulong texture, Rect target, Options options);
    internal static void Run()
    {
        if (Marshal.SizeOf<Options>()!=64 || Marshal.OffsetOf<Options>(nameof(Options.Flags)).ToInt32()!=60) throw new Exception("2D ABI mismatch");
        var native=typeof(Draw2DOptions).Assembly.GetType("Oxide.Graphics.NativeDraw2DOptions")!;
        if (Marshal.SizeOf(native)!=64) throw new Exception("C# sprite layout mismatch");
        if (oxide_draw_2d_options_default(out var options)!=0 || options.Tint.A!=1 || options.Tint.R!=1 || options.Flags!=0) throw new Exception("Sprite defaults mismatch");
        options.Flags=16;
        if (oxide_graphics_draw_2d_with_options(IntPtr.Zero,0,default,options)!=1) throw new Exception("Unknown sprite flags accepted");
        var first=new Draw2DOptions();var second=new Draw2DOptions();first.Tint=new Color(1,0,0,0.5f);
        if (second.Tint.A!=1 || second.SourceRectangle is not null || new TextOptions().FontFamily!="Segoe UI") throw new Exception("Sprite defaults shared");
        if (typeof(Graphics).GetMethod("Draw2D",[typeof(Texture),typeof(Rect)]) is null || typeof(Graphics).GetMethod("DrawText",[typeof(string),typeof(Vec2),typeof(Color)]) is null) throw new Exception("2D overload missing");
        Console.WriteLine("2D bindings checks passed");
    }
}
