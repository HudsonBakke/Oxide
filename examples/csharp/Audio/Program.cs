using Oxide.Audio;
using Oxide.Content;
using Oxide.Math;
using Clip = Oxide.Audio.Audio;

using Clip audio = args.Length>0 ? new ContentManager().Load<Clip>(args[0])
    : Clip.FromSamples(Enumerable.Range(0,48000).Select(i=>(float)System.Math.Sin(System.Math.Tau*440*i/48000)*0.15f).ToArray());
using AudioListener listener=new(); listener.Activate();
using AudioSettings settings=new() { Looping=true };settings.SetDistanceRange(1,20);
using AudioSource source=new(new Vec3(0,0,-2),settings,new Vec3(0,0,1));source.Play(audio);
Console.WriteLine("Headphones: moving source. First 5 seconds omni, then a rotating cone. Live low-pass and modulation follow.");
for(int frame=0;frame<200;frame++)
{
    float t=frame*0.05f;source.Position=new Vec3(MathF.Sin(t)*3,0,-2);source.Direction=new Vec3(MathF.Sin(t*2),0,MathF.Cos(t*2));
    if(frame==60) {settings.LowPassCutoff=400;settings.Pitch=0.75f;Console.WriteLine("Low-pass 400 Hz, pitch 0.75");}
    if(frame==100) source.PlayDirectional(audio,Angle.Degrees(15),Angle.Degrees(60),0.05f);
    if(frame==130) {settings.LowPassCutoff=24000;settings.ModulationFrequency=6;settings.ModulationDepth=0.8f;Console.WriteLine("Modulation 6 Hz");}
    _=source.State;Thread.Sleep(50);
}
source.Pause();Thread.Sleep(250);source.Resume();Thread.Sleep(250);source.Stop();
