using Oxide.Math;
using Oxide.Native;
namespace Oxide.Audio;

/// <summary>State of the source's single playback.</summary>
public enum PlaybackState
{
    /// <summary>No clip is playing, or a nonlooping clip has completed.</summary>
    Stopped=0,
    /// <summary>Samples are advancing, including while outside audible range.</summary>
    Playing=1,
    /// <summary>Playback is held at its current cursor.</summary>
    Paused=2,
}
/// <summary>A positional emitter with one playback; create multiple sources to overlap sounds.</summary>
/// <remarks>Output opens lazily at construction. No geometry or occlusion is processed automatically.</remarks>
public sealed class AudioSource : IDisposable
{
    private readonly AudioHandle handle;
    private AudioSettings settings;
    private ulong Handle => handle.Value;
    /// <summary>Creates a source; omitted settings use defaults and omitted direction faces -Z. Throws if no output device is available.</summary>
    public AudioSource(Vec3 position,AudioSettings? settings=null,Vec3? direction=null)
    {
        this.settings=settings??new AudioSettings();
        NativeMethods.Check(AudioNative.oxide_audio_source_create(position,this.settings.Handle,direction??new Vec3(0,0,-1),out ulong value));
        handle=new(value,ResourceKind.Source);
    }
    /// <summary>Shared controls for active and future playback; property edits do not restart audio.</summary>
    public AudioSettings Settings
    {
        get { _=Handle; if(settings.IsDisposed) { NativeMethods.Check(AudioNative.oxide_audio_source_get_settings(Handle,out ulong value)); settings=new AudioSettings(value); } return settings; }
        set { ArgumentNullException.ThrowIfNull(value); NativeMethods.Check(AudioNative.oxide_audio_source_set_settings(Handle,value.Handle)); settings=value; }
    }
    /// <summary>World-space source position; changes apply live.</summary>
    public Vec3 Position { get { NativeMethods.Check(AudioNative.oxide_audio_source_get_position(Handle,out Vec3 v)); return v; } set => NativeMethods.Check(AudioNative.oxide_audio_source_set_position(Handle,value)); }
    /// <summary>Finite nonzero emission direction, normalized on assignment; changes apply live.</summary>
    public Vec3 Direction { get { NativeMethods.Check(AudioNative.oxide_audio_source_get_direction(Handle,out Vec3 v)); return v; } set => NativeMethods.Check(AudioNative.oxide_audio_source_set_direction(Handle,value)); }
    /// <summary>Current playback state; reports output failures through OxideException.</summary>
    public PlaybackState State { get { NativeMethods.Check(AudioNative.oxide_audio_source_get_state(Handle,out uint value)); return (PlaybackState)value; } }
    /// <summary>Replaces the current sound with omnidirectional, distance-attenuated playback.</summary>
    public void Play(Audio audio) { ArgumentNullException.ThrowIfNull(audio); NativeMethods.Check(AudioNative.oxide_audio_source_play(Handle,audio.Handle)); }
    /// <summary>Replaces the current sound with cone emission; angles are half-angles, and distance attenuation also applies.</summary>
    public void PlayDirectional(Audio audio,Angle innerHalfAngle,Angle outerHalfAngle,float outerVolume=0)
    { ArgumentNullException.ThrowIfNull(audio); NativeMethods.Check(AudioNative.oxide_audio_source_play_directional(Handle,audio.Handle,innerHalfAngle,outerHalfAngle,outerVolume)); }
    /// <summary>Changes the emission cone without restarting; requires 0 &lt;= inner &lt;= outer &lt;= 180 degrees and outerVolume in [0,1].</summary>
    public void SetCone(Angle innerHalfAngle,Angle outerHalfAngle,float outerVolume=0) => NativeMethods.Check(AudioNative.oxide_audio_source_set_cone(Handle,innerHalfAngle,outerHalfAngle,outerVolume));
    /// <summary>Pauses a playing sound at its cursor; otherwise does nothing.</summary>
    public void Pause() => NativeMethods.Check(AudioNative.oxide_audio_source_pause(Handle));
    /// <summary>Resumes a paused sound; otherwise does nothing.</summary>
    public void Resume() => NativeMethods.Check(AudioNative.oxide_audio_source_resume(Handle));
    /// <summary>Stops playback and releases its clip.</summary>
    public void Stop() => NativeMethods.Check(AudioNative.oxide_audio_source_stop(Handle));
    /// <summary>Stops this source and releases its native handle.</summary>
    public void Dispose() => handle.Dispose();
}
