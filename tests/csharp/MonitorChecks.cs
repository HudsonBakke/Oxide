using System.Runtime.InteropServices;
using Oxide.Math;
using Oxide.Graphics;
internal static class MonitorChecks
{
    internal static void Run()
    {
        var rect=new Rect3D(new Vec3(-2,1,0),new Vec3(2,1,0),new Vec3(2,-1,0),new Vec3(-2,-1,0),new Vec3(0,0,4));
        if(Marshal.SizeOf<Rect3D>()!=60 || Marshal.OffsetOf<Rect3D>(nameof(Rect3D.Normal)).ToInt32()!=48) throw new Exception("Rect3D native layout mismatch");
        if(rect.Width!=4 || rect.Height!=2 || rect.AspectRatio!=2 || rect.Normal.Z!=1) throw new Exception("Rect3D geometry mismatch");
        bool threw=false;
        try { _=new Rect3D(rect.TopLeft,rect.TopRight,rect.BottomRight,rect.BottomLeft,new Vec3(1,0,0)); }
        catch(Oxide.OxideException) { threw=true; }
        if(!threw) throw new Exception("Invalid Rect3D was accepted");
        if(typeof(Graphics).GetMethod("BindCamera",[typeof(Camera),typeof(Rect3D)]) is null ||
           typeof(Graphics).GetMethod("BindCamera",[typeof(Camera),typeof(Rect3D),typeof(uint),typeof(uint)]) is null ||
           typeof(Graphics).GetMethod("BindCamera",[typeof(Camera),typeof(Rect)]) is null) throw new Exception("Camera overload missing");
        Console.WriteLine("Rect3D bindings checks passed");
    }
}
