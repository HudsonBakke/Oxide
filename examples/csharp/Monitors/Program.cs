using Oxide.App;
using Oxide.Content;
using Oxide.Graphics;
using Oxide.Input;
using Oxide.Math;

internal sealed class Monitors : Game
{
    private readonly Mesh mesh=new ContentManager().Load<Mesh>("assets/blender_cube.glb");
    private readonly Material[] materials=[Material.Solid(Color.Red),Material.Solid(Color.Green),Material.Solid(Color.Blue),
        Material.Solid(new Color(0.35f,0.38f,0.45f,1)),Material.Solid(new Color(0.015f,0.02f,0.03f,1)),Material.Solid(new Color(1,0.75f,0.05f,1))];
    private readonly Transform[] room=[
        new() {Position=new Vec3(10.5f,0.6f,0),Scale=new Vec3(0.55f,0.55f,0.55f)},
        new() {Position=new Vec3(12,0.8f,-0.5f),Scale=new Vec3(0.5f,0.8f,0.5f)},
        new() {Position=new Vec3(13.5f,0.5f,0.4f),Scale=new Vec3(0.5f,0.5f,0.5f)},
        new() {Position=new Vec3(12,-0.1f,0),Scale=new Vec3(3.5f,0.1f,3)}];
    private readonly Transform[] frames=[new() {Position=new Vec3(-1.25f,0.6f,-0.12f),Scale=new Vec3(1.18f,0.88f,0.1f)},new() {Position=new Vec3(1.25f,0.6f,-0.12f),Scale=new Vec3(1.18f,0.88f,0.1f)}];
    private Transform occluder=new() {Position=new Vec3(0,0.2f,1),Scale=new Vec3(0.35f,0.35f,0.35f)};
    private readonly Camera screen=new();
    private readonly Camera[] feeds=[new(),new()];
    private readonly Oxide.Graphics.Environment environment=new(Color.White,0.2f,null,0,null,new Color(0.035f,0.05f,0.08f,1));
    private readonly DirectionalLight light=new(new Vec3(-0.4f,-1,-0.5f),Color.White,2.5f);
    private bool feedsFirst=true, front=true;
    private float transparency=0.5f;
    public Monitors()
    {
        Aim(screen,new Vec3(0,1.4f,6.5f),new Vec3(0,0.5f,0));
        Aim(feeds[0],new Vec3(12,2.4f,6),new Vec3(12,0.5f,0));
        Aim(feeds[1],new Vec3(15,4,4.5f),new Vec3(12,0.5f,0));
    }
    private static void Aim(Camera camera,Vec3 position,Vec3 target)
    { camera.Position=position;camera.Rotation=Quaternion.FromTo(new Vec3(0,0,-1),target-position); }
    private Rect3D Monitor(float x) => new(new Vec3(x-1.1f,1.4f,0),new Vec3(x+1.1f,1.4f,0),new Vec3(x+1.1f,-0.2f,0),new Vec3(x-1.1f,-0.2f,0),new Vec3(0,0,front?1:-1));
    protected override void Update(GameTime time)
    {
        float t=(float)time.TotalSeconds;
        room[0].Rotation=Quaternion.FromAxisAngle(new Vec3(0,1,0),Angle.Radians(t));
        occluder.Position=new Vec3(MathF.Sin(t*0.7f)*2,0.3f,1.1f);
        Aim(feeds[0],new Vec3(12+MathF.Sin(t)*1.3f,2.4f,6),new Vec3(12,0.5f,0));
        var keys=Keyboard.GetState();
        if(keys.IsKeyPressed(Keys.Space)) feedsFirst=!feedsFirst;
        if(keys.IsKeyPressed(Keys.T)) transparency=transparency<1?transparency+0.5f:0;
        if(keys.IsKeyPressed(Keys.B)) front=!front;
        if(keys.IsKeyPressed(Keys.Escape)) Exit();
        SetWindowName($"Oxide Monitors | Space order: {(feedsFirst?"feeds first":"screen first")} | T transparency: {transparency} | B front: {front}");
    }
    private void DrawRoom(Graphics graphics)
    { for(int i=0;i<4;i++) graphics.DrawMesh(mesh,materials[i],room[i]); }
    private void DrawFeeds(Graphics graphics)
    { graphics.BindCamera(feeds[0],Monitor(-1.25f));DrawRoom(graphics);graphics.BindCamera(feeds[1],Monitor(1.25f));DrawRoom(graphics); }
    private void DrawScreen(Graphics graphics)
    {
        graphics.BindCamera(screen,graphics.FullRect);DrawRoom(graphics);
        foreach(var frame in frames) graphics.DrawMesh(mesh,materials[4],frame);
        graphics.DrawMesh(mesh,materials[5],occluder,transparency);
    }
    protected override void Draw(Graphics graphics)
    {
        graphics.SetEnvironment(environment);graphics.ClearLights();graphics.AddLight(light);
        if(feedsFirst) {DrawFeeds(graphics);DrawScreen(graphics);} else {DrawScreen(graphics);DrawFeeds(graphics);}
    }
}
internal static class Program
{
    private static void Main() {using var game=new Monitors();game.Run(new AppConfig {Title="Oxide Monitors"});}
}
