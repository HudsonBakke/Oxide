using Oxide.Math;
using Oxide.Native;
namespace Oxide.Audio;

/// <summary>Independent audio viewpoint; initially at the origin facing -Z, with +Y up.</summary>
public sealed class AudioListener : IDisposable
{
    private readonly AudioHandle handle;
    private ulong Handle => handle.Value;
    /// <summary>Creates a listener without activating it or opening an output device.</summary>
    public AudioListener() { NativeMethods.Check(AudioNative.oxide_audio_listener_create(out ulong value)); handle=new(value,ResourceKind.Listener); }
    /// <summary>Selects this listener; later edits apply live. Selection retains native state after disposal.</summary>
    public void Activate() => NativeMethods.Check(AudioNative.oxide_audio_listener_activate(Handle));
    /// <summary>World-space position; changes affect active playback.</summary>
    public Vec3 Position { get { NativeMethods.Check(AudioNative.oxide_audio_listener_get_position(Handle,out Vec3 v)); return v; } set => NativeMethods.Check(AudioNative.oxide_audio_listener_set_position(Handle,value)); }
    /// <summary>Normalized forward direction.</summary>
    public Vec3 Direction { get { NativeMethods.Check(AudioNative.oxide_audio_listener_get_direction(Handle,out Vec3 v)); return v; } }
    /// <summary>Normalized up vector orthogonal to Direction.</summary>
    public Vec3 Up { get { NativeMethods.Check(AudioNative.oxide_audio_listener_get_up(Handle,out Vec3 v)); return v; } }
    /// <summary>Normalizes and orthogonalizes nonzero, nonparallel forward/up vectors.</summary>
    public void SetOrientation(Vec3 direction,Vec3 up) => NativeMethods.Check(AudioNative.oxide_audio_listener_set_orientation(Handle,direction,up));
    /// <summary>Releases this handle; an activated listener retains its last native state.</summary>
    public void Dispose() => handle.Dispose();
}
