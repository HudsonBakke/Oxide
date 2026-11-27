using Oxide.Content;
using Oxide.Native;
namespace Oxide.Audio;

/// <summary>Immutable decoded mono sound, loaded from WAV/MP3 or normalized samples.</summary>
/// <remarks>Content loading averages multichannel input to mono. Active sources retain samples after this wrapper is disposed.</remarks>
public sealed class Audio : IContentAsset, IDisposable
{
    private readonly AudioHandle handle;
    internal ulong Handle => handle.Value;
    internal Audio(ulong value) { handle=new(value,ResourceKind.Clip); }
    /// <summary>Copies nonempty finite mono samples in [-1,1], at 8000..192000 Hz; maximum 64 million samples.</summary>
    public static Audio FromSamples(float[] samples, uint sampleRate=48000)
    {
        ArgumentNullException.ThrowIfNull(samples);
        NativeMethods.Check(AudioNative.oxide_audio_create_samples(samples,(nuint)samples.Length,sampleRate,out ulong value));
        return new(value);
    }
    /// <summary>Decoded mono samples per second.</summary>
    public uint SampleRate { get { NativeMethods.Check(AudioNative.oxide_audio_metadata(Handle,out uint rate,out _,out _)); return rate; } }
    /// <summary>Number of decoded mono samples.</summary>
    public ulong SampleCount { get { NativeMethods.Check(AudioNative.oxide_audio_metadata(Handle,out _,out ulong count,out _)); return count; } }
    /// <summary>Clip duration before pitch changes or looping.</summary>
    public TimeSpan Duration { get { NativeMethods.Check(AudioNative.oxide_audio_metadata(Handle,out _,out _,out double seconds)); return TimeSpan.FromSeconds(seconds); } }
    /// <summary>Releases this asset handle; active sources retain their samples.</summary>
    public void Dispose() => handle.Dispose();
}
