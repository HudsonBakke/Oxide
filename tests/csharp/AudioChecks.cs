using Oxide.Audio;
using Oxide.Math;
using Clip=Oxide.Audio.Audio;

internal static class AudioChecks
{
    internal static void Run()
    {
        using var settings=new AudioSettings { Volume=0.5f,Looping=true };
        settings.SetDistanceRange(10,20);
        if(settings.Volume!=0.5f || !settings.Looping || settings.ReferenceDistance!=10) throw new Exception("Audio settings mismatch");
        foreach(float value in new[] {float.NaN,float.PositiveInfinity,-1f,17f})
        {
            bool threw=false;try { settings.Volume=value; } catch(Oxide.OxideException) { threw=true; }
            if(!threw || settings.Volume!=0.5f) throw new Exception("Invalid audio setting changed state");
        }
        settings.LowPassCutoff=400;settings.ModulationDepth=0.8f;settings.ModulationFrequency=6;
        using var clip=Clip.FromSamples([0,0.5f,-1],24000);
        if(clip.SampleRate!=24000 || clip.SampleCount!=3 || System.Math.Abs(clip.Duration.TotalSeconds-3d/24000)>1e-6) throw new Exception("Audio metadata mismatch");
        using var listener=new AudioListener();listener.Position=new Vec3(1,2,3);listener.SetOrientation(new Vec3(1,0,0),new Vec3(0,2,0));listener.Activate();
        if(listener.Position.X!=1 || listener.Direction.X!=1 || listener.Up.Y!=1) throw new Exception("Audio listener mismatch");
        clip.Dispose();bool disposed=false;try { _=clip.Duration; } catch(ObjectDisposedException) { disposed=true; }
        if(!disposed) throw new Exception("Closed audio handle remained usable");
        string directory=Path.Combine(Path.GetTempPath(),"oxide-csharp-audio-"+Guid.NewGuid());
        Directory.CreateDirectory(directory);
        try
        {
            string path=Path.Combine(directory,"tone-音声.wav");
            using(var stream=File.Create(path))
            using(var writer=new BinaryWriter(stream))
            {
                writer.Write(System.Text.Encoding.ASCII.GetBytes("RIFF"));writer.Write(36+200);
                writer.Write(System.Text.Encoding.ASCII.GetBytes("WAVEfmt "));writer.Write(16);writer.Write((short)1);writer.Write((short)1);
                writer.Write(44100);writer.Write(88200);writer.Write((short)2);writer.Write((short)16);
                writer.Write(System.Text.Encoding.ASCII.GetBytes("data"));writer.Write(200);
                for(int i=0;i<100;i++) writer.Write((short)8192);
            }
            var content=new Oxide.Content.ContentManager(Path.Combine(directory,"cache"));
            using var loaded=content.Load<Clip>(path);
            if(loaded.SampleRate!=44100 || loaded.SampleCount!=100) throw new Exception("C# audio content mismatch");
            string fixture=new string(File.ReadAllText("tests/fixtures/tone_mp3.hex").Where(c=>!char.IsWhiteSpace(c)).ToArray());
            string mp3=Path.Combine(directory,"tone.mp3");File.WriteAllBytes(mp3,Convert.FromHexString(fixture));
            using var decoded=content.Load<Clip>(mp3);
            if(decoded.Duration.TotalSeconds<0.15) throw new Exception("C# MP3 content mismatch");
        }
        finally { Directory.Delete(directory,true); }
        Console.WriteLine("Audio bindings checks passed");
    }
}
