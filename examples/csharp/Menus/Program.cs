using Oxide.App;
using Oxide.Content;
using Oxide.Graphics;
using Oxide.Input;
using Oxide.Math;

internal sealed class Menus : Game
{
    private readonly Texture white = Texture.Solid(Color.White);
    private readonly Texture atlas = Texture.FromRgba(2, 2, [255,70,70,255, 70,255,110,255, 70,140,255,255, 255,210,70,255]);
    private readonly Mesh mesh = new ContentManager().Load<Mesh>("assets/blender_cube.glb");
    private readonly Material material = Material.Solid(new Color(0.1f,0.4f,0.8f));
    private readonly Camera camera = new() { Position = new Vec3(-1.5f,0,5) };
    private Transform transform = new();
    private readonly Oxide.Graphics.Environment environment = new(Color.White,0.2f,null,0,null,new Color(0.02f,0.035f,0.065f));
    private readonly DirectionalLight light = new(new Vec3(-0.5f,-1,-0.5f),Color.White,2);
    private int selected, exposure = 1;
    private bool paused, flipped, clipped = true;
    private float angle;
    private static Rect Button(int i) => new(40,130+i*68,260,52);
    private void Activate()
    {
        if (selected == 0) paused = !paused;
        else if (selected == 1) exposure = (exposure + 1) % 3;
        else Exit();
    }
    protected override void Update(GameTime time)
    {
        var keys = Keyboard.GetState(); var mouse = Mouse.GetState(); var p = mouse.Position;
        if (keys.IsKeyPressed(Keys.Up)) selected = (selected + 2) % 3;
        if (keys.IsKeyPressed(Keys.Down)) selected = (selected + 1) % 3;
        for (int i = 0; i < 3; i++)
        {
            var r = Button(i);
            if (p.X >= r.X && p.X < r.X+r.Width && p.Y >= r.Y && p.Y < r.Y+r.Height)
            {
                if (mouse.Delta.Length > 0 || mouse.IsButtonPressed(MouseButtons.Left)) selected = i;
                if (mouse.IsButtonPressed(MouseButtons.Left)) Activate();
            }
        }
        if (keys.IsKeyPressed(Keys.Enter)) Activate();
        if (keys.IsKeyPressed(Keys.F)) flipped = !flipped;
        if (keys.IsKeyPressed(Keys.C)) clipped = !clipped;
        if (keys.IsKeyPressed(Keys.Escape)) Exit();
        if (!paused) { angle += (float)time.DeltaSeconds; transform.RotateYawBy(Angle.Radians((float)time.DeltaSeconds)); }
    }
    private void Panel(Graphics g, Rect r, Color color) => g.Draw2D(white,r,new Draw2DOptions {Tint=color});
    protected override void Draw(Graphics g)
    {
        g.Exposure = new float[] {0,1,4}[exposure]; g.SetEnvironment(environment); g.AddLight(light);
        // 2D submitted before 3D still appears above the completed scene.
        Panel(g,new Rect(24,24,292,348),new Color(0.025f,0.04f,0.08f,0.94f));
        g.BindCamera(camera,g.FullRect); g.DrawMesh(mesh,material,transform);
        g.DrawText("OXIDE",new Vec2(40,38),new TextOptions {FontSize=36});
        g.DrawText("2D + 3D",new Vec2(42,85),new Color(0.45f,0.75f,1));
        string[] labels = [paused ? "Resume" : "Pause", $"Exposure: {new int[] {0,1,4}[exposure]}", "Quit"];
        for (int i=0;i<3;i++)
        {
            var r=Button(i); Panel(g,r,i==selected ? new Color(0.08f,0.3f,0.6f) : new Color(0.06f,0.09f,0.15f));
            var size=g.MeasureText(labels[i]); g.DrawText(labels[i],new Vec2(r.X+(r.Width-size.X)*0.5f,r.Y+(r.Height-size.Y)*0.5f));
        }
        var preview=new Rect(360,48,160,160);
        g.Draw2D(atlas,preview,new Draw2DOptions {Rotation=Angle.Radians(angle*0.4f),Origin=new Vec2(80,80),FlipHorizontally=flipped,
            ClipRectangle=clipped ? preview : null,Tint=new Color(1,1,1,0.8f)});
        g.Draw2D(atlas,new Rect(360,232,32,32),new Draw2DOptions {SourceRectangle=new Rect(1,0,1,1)});
        g.DrawText("Mouse or Up/Down + Enter\nF: flip   C: clip   Escape: quit\nUI ignores scene exposure",new Vec2(40,395),new TextOptions {FontSize=16});
    }
}
internal static class Program
{
    private static void Main() {using var game=new Menus();game.Run(new AppConfig {Title="Oxide Menus",Width=900,Height=600});}
}
