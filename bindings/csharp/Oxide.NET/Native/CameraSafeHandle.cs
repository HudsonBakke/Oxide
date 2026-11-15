using System;
using System.Runtime.InteropServices;

namespace Oxide.Native;

internal sealed class CameraSafeHandle
    : SafeHandle
{
    internal CameraSafeHandle(
        NativeCameraHandle handle
    )
        : base(
            IntPtr.Zero,
            true
        )
    {
        SetHandle(
            new IntPtr(
                unchecked(
                    (long)handle.Value
                )
            )
        );
    }

    private CameraSafeHandle()
        : base(
            IntPtr.Zero,
            true
        )
    {
    }

    public override bool IsInvalid =>
        handle == IntPtr.Zero;

    internal NativeCameraHandle
        NativeHandle =>
            new() {
                Value =
                    unchecked(
                        (ulong)handle.ToInt64()
                    ),
            };

    protected override bool ReleaseHandle()
    {
        OxideStatus status =
            NativeMethods
                .oxide_camera_destroy(
                    NativeHandle
                );

        return status
            == OxideStatus.Ok;
    }
}