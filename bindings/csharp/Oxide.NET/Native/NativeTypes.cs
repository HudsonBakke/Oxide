using System;
using System.Runtime.InteropServices;

namespace Oxide.Native;

internal enum OxideStatus
{
    Ok = 0,
    InvalidArgument = 1,
    InvalidOperation = 2,
    ContentError = 3,
    GraphicsError = 4,
    InternalError = 5,
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
internal struct NativeAppConfig
{
    public IntPtr Title;

    public uint Width;
    public uint Height;
}

[UnmanagedFunctionPointer(
    CallingConvention.Cdecl
)]
internal delegate void
    NativeInitializeCallback(
        IntPtr userData
    );

[UnmanagedFunctionPointer(
    CallingConvention.Cdecl
)]
internal delegate void
    NativeUpdateCallback(
        IntPtr userData,
        NativeGameTime time
    );

[UnmanagedFunctionPointer(
    CallingConvention.Cdecl
)]
internal delegate void
    NativeDrawCallback(
        IntPtr userData,
        NativeGraphicsHandle graphics
    );

[StructLayout(LayoutKind.Sequential)]
internal struct NativeGameCallbacks
{
    public IntPtr UserData;

    public NativeInitializeCallback Initialize;
    public NativeUpdateCallback Update;
    public NativeDrawCallback Draw;
}

