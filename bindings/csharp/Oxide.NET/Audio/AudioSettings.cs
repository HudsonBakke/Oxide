using Oxide.Native;
namespace Oxide.Audio;

/// <summary>Shared live playback controls. Sources retain native settings independently of this wrapper.</summary>
/// <remarks>Changes affect all sources using this instance. Dispose releases this handle, without invalidating active sources.</remarks>
public sealed class AudioSettings : IDisposable
{
    private readonly AudioHandle handle;
    internal ulong Handle => handle.Value;
    internal bool IsDisposed => handle.IsClosed;
    internal AudioSettings(ulong value) { handle=new(value,ResourceKind.Settings); }
    /// <summary>Creates default settings without opening an audio device.</summary>
    public AudioSettings() { NativeMethods.Check(AudioNative.oxide_audio_settings_create(out ulong value)); handle=new(value,ResourceKind.Settings); }
    private SettingsValues Values { get { NativeMethods.Check(AudioNative.oxide_audio_settings_get(Handle,out SettingsValues value)); return value; } }
    /// <summary>Linear gain in [0,16], initially one. Changes affect active playback; invalid values throw without modifying settings.</summary>
    public float Volume { get => Values.Volume; set => NativeMethods.Check(AudioNative.oxide_audio_settings_set_volume(Handle,value)); }
    /// <summary>Playback-rate ratio in [1/16,16], initially one. Changes affect active playback; invalid values throw without modifying settings.</summary>
    public float Pitch { get => Values.Pitch; set => NativeMethods.Check(AudioNative.oxide_audio_settings_set_pitch(Handle,value)); }
    /// <summary>World distance where attenuation begins, initially one. Changes affect active playback; invalid values throw without modifying settings.</summary>
    public float ReferenceDistance { get => Values.ReferenceDistance; set => NativeMethods.Check(AudioNative.oxide_audio_settings_set_reference_distance(Handle,value)); }
    /// <summary>World cutoff distance, greater than ReferenceDistance; initially 50. Changes affect active playback; invalid values throw without modifying settings.</summary>
    public float MaxDistance { get => Values.MaxDistance; set => NativeMethods.Check(AudioNative.oxide_audio_settings_set_max_distance(Handle,value)); }
    /// <summary>Inverse-distance exponent in [0,16], initially one. Changes affect active playback; invalid values throw without modifying settings.</summary>
    public float Rolloff { get => Values.Rolloff; set => NativeMethods.Check(AudioNative.oxide_audio_settings_set_rolloff(Handle,value)); }
    /// <summary>Low-pass cutoff in Hz, [20,24000]; 24000 bypasses filtering. Changes affect active playback; invalid values throw without modifying settings.</summary>
    public float LowPassCutoff { get => Values.LowPassCutoff; set => NativeMethods.Check(AudioNative.oxide_audio_settings_set_low_pass_cutoff(Handle,value)); }
    /// <summary>Amplitude-modulation frequency in Hz, [0,100]; zero disables it. Changes affect active playback; invalid values throw without modifying settings.</summary>
    public float ModulationFrequency { get => Values.ModulationFrequency; set => NativeMethods.Check(AudioNative.oxide_audio_settings_set_modulation_frequency(Handle,value)); }
    /// <summary>Amplitude-modulation depth in [0,1]; zero disables it. Changes affect active playback; invalid values throw without modifying settings.</summary>
    public float ModulationDepth { get => Values.ModulationDepth; set => NativeMethods.Check(AudioNative.oxide_audio_settings_set_modulation_depth(Handle,value)); }
    /// <summary>Whether active playback wraps to the beginning, initially false.</summary>
    public bool Looping { get => Values.Looping!=0; set => NativeMethods.Check(AudioNative.oxide_audio_settings_set_looping(Handle,value?1u:0u)); }
    /// <summary>Changes both distance bounds atomically; requires 0 &lt; reference &lt; maximum.</summary>
    public void SetDistanceRange(float reference, float maximum) => NativeMethods.Check(AudioNative.oxide_audio_settings_set_distance_range(Handle,reference,maximum));
    /// <summary>Releases this handle; sources retain their shared settings.</summary>
    public void Dispose() => handle.Dispose();
}
