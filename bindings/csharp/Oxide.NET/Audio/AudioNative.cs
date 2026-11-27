using System.Runtime.InteropServices;
using Oxide.Math;
using Oxide.Native;
namespace Oxide.Audio;

[StructLayout(LayoutKind.Sequential)]
internal struct SettingsValues
{
    internal float Volume;
    internal float Pitch;
    internal float ReferenceDistance;
    internal float MaxDistance;
    internal float Rolloff;
    internal float LowPassCutoff;
    internal float ModulationFrequency;
    internal float ModulationDepth;
    internal uint Looping;
}
internal enum ResourceKind { Clip, Settings, Source, Listener }
internal sealed class AudioHandle : SafeHandle
{
    private readonly ResourceKind kind;
    internal AudioHandle(ulong value, ResourceKind kind) : base(IntPtr.Zero,true) { this.kind=kind; SetHandle(unchecked((IntPtr)(long)value)); }
    public override bool IsInvalid => handle==IntPtr.Zero;
    internal ulong Value { get { ObjectDisposedException.ThrowIf(IsClosed || IsInvalid,this); return unchecked((ulong)handle.ToInt64()); } }
    protected override bool ReleaseHandle() => (kind switch {
        ResourceKind.Clip => AudioNative.oxide_audio_destroy(unchecked((ulong)handle.ToInt64())),
        ResourceKind.Settings => AudioNative.oxide_audio_settings_destroy(unchecked((ulong)handle.ToInt64())),
        ResourceKind.Source => AudioNative.oxide_audio_source_destroy(unchecked((ulong)handle.ToInt64())),
        _ => AudioNative.oxide_audio_listener_destroy(unchecked((ulong)handle.ToInt64())),
    }) == OxideStatus.Ok;
}
internal static class AudioNative
{
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_destroy(ulong handle);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_settings_destroy(ulong handle);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_source_destroy(ulong handle);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_listener_destroy(ulong handle);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_create_samples(float[] samples, nuint count, uint rate, out ulong result);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_metadata(ulong handle, out uint rate, out ulong count, out double duration);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_settings_create(out ulong result);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_settings_get(ulong handle, out SettingsValues result);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_settings_set(ulong handle, SettingsValues values);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_settings_set_distance_range(ulong handle, float @reference, float maximum);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_settings_set_looping(ulong handle, uint looping);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_settings_set_volume(ulong handle, float value);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_settings_set_pitch(ulong handle, float value);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_settings_set_reference_distance(ulong handle, float value);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_settings_set_max_distance(ulong handle, float value);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_settings_set_rolloff(ulong handle, float value);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_settings_set_low_pass_cutoff(ulong handle, float value);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_settings_set_modulation_frequency(ulong handle, float value);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_settings_set_modulation_depth(ulong handle, float value);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_source_create(Vec3 position, ulong settings_handle, Vec3 direction, out ulong result);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_source_play(ulong handle, ulong clip);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_source_play_directional(ulong handle, ulong clip, Angle inner, Angle outer, float outer_volume);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_source_set_cone(ulong handle, Angle inner, Angle outer, float outer_volume);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_source_set_settings(ulong handle, ulong settings_handle);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_source_get_settings(ulong handle, out ulong result);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_source_get_state(ulong handle, out uint result);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_source_pause(ulong handle);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_source_resume(ulong handle);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_source_stop(ulong handle);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_listener_create(out ulong result);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_listener_activate(ulong handle);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_listener_set_orientation(ulong handle, Vec3 forward, Vec3 up);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_source_get_position(ulong handle, out Vec3 result);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_source_set_position(ulong handle, Vec3 value);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_source_get_direction(ulong handle, out Vec3 result);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_source_set_direction(ulong handle, Vec3 value);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_listener_get_position(ulong handle, out Vec3 result);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_listener_set_position(ulong handle, Vec3 value);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_listener_get_direction(ulong handle, out Vec3 result);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_audio_listener_get_up(ulong handle, out Vec3 result);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_content_load_audio(NativeContentManagerHandle manager, IntPtr path, out ulong result);
    [DllImport("oxide", CallingConvention=CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_content_load_audio_sub(NativeContentManagerHandle manager, IntPtr path, IntPtr subobject, out ulong result);
}
