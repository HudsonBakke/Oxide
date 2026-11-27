using System;
using System.Runtime.InteropServices;

using Oxide.Math;

namespace Oxide.Native;

internal enum OxideStatus
{
    Ok = 0,
    InvalidArgument = 1,
    InvalidOperation = 2,
    ContentError = 3,
    GraphicsError = 4,
    InternalError = 5,
    /// <summary>Audio output initialization or playback failed.</summary>
    AudioError = 6,
}

[StructLayout(LayoutKind.Sequential)]
internal struct NativeMeshHandle
{
    public ulong Value;
}

[StructLayout(LayoutKind.Sequential)]
internal struct NativeMaterialHandle
{
    public ulong Value;
}

[StructLayout(LayoutKind.Sequential)]
internal struct NativeTextureHandle
{
    public ulong Value;
}

[StructLayout(LayoutKind.Sequential)]
internal struct NativeGameTime
{
    public double DeltaSeconds;
    public double TotalSeconds;
}

[StructLayout(LayoutKind.Sequential)]
internal struct NativeGraphicsHandle
{
    public IntPtr Ptr;
}

[StructLayout(LayoutKind.Sequential)]
internal struct NativeCameraHandle
{
    public ulong Value;
}

[StructLayout(LayoutKind.Sequential)]
internal struct NativeContentManagerHandle
{
    public ulong Value;
}

[StructLayout(LayoutKind.Sequential)]
internal struct NativeAppConfig
{
    public IntPtr Title;

    public uint Width;
    public uint Height;
}

[UnmanagedFunctionPointer(CallingConvention.Cdecl)]
internal delegate void NativeInitializeCallback(IntPtr userData);

[UnmanagedFunctionPointer(CallingConvention.Cdecl)]
internal delegate void NativeUpdateCallback(IntPtr userData, NativeGameTime time);

[UnmanagedFunctionPointer(CallingConvention.Cdecl)]
internal delegate void NativeDrawCallback(IntPtr userData, NativeGraphicsHandle graphics);

[StructLayout(LayoutKind.Sequential)]
internal struct NativeGameCallbacks
{
    public IntPtr UserData;

    public NativeInitializeCallback Initialize;
    public NativeUpdateCallback Update;
    public NativeDrawCallback Draw;
}

[StructLayout(LayoutKind.Sequential)]
internal struct NativeKeyboardState
{
    public ulong Current0;
    public ulong Current1;
    public ulong Previous0;
    public ulong Previous1;
}

[StructLayout(LayoutKind.Sequential)]
internal struct NativeMouseState
{
    public ulong CurrentButtons;
    public ulong PreviousButtons;
    public Vec2 Position;
    public Vec2 Delta;
    public Vec2 ScrollDelta;
}
