using System;
using System.Runtime.InteropServices;

namespace Oxide.Native;

internal sealed class TextureSafeHandle : SafeHandle
{
    internal TextureSafeHandle(NativeTextureHandle handle) : base(IntPtr.Zero, true)
    {
        SetHandle(new IntPtr(unchecked((long)handle.Value)));
    }

    public override bool IsInvalid => handle == IntPtr.Zero;

    internal NativeTextureHandle NativeHandle => new() {
        Value = unchecked((ulong)handle.ToInt64()),
    };

    protected override bool ReleaseHandle()
    {
        return NativeMethods.oxide_texture_destroy(NativeHandle) == OxideStatus.Ok;
    }
}