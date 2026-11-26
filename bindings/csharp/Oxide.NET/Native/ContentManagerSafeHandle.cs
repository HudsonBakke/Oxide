using System;
using System.Runtime.InteropServices;

namespace Oxide.Native;

internal sealed class ContentManagerSafeHandle : SafeHandle
{
    internal ContentManagerSafeHandle(NativeContentManagerHandle handle) : base(IntPtr.Zero, true)
    {
        SetHandle(new IntPtr(unchecked((long)handle.Value)));
    }

    private ContentManagerSafeHandle() : base(IntPtr.Zero, true) {}

    public override bool IsInvalid => handle == IntPtr.Zero;

    internal NativeContentManagerHandle NativeHandle => new() {
        Value = unchecked((ulong)handle.ToInt64()),
    };

    protected override bool ReleaseHandle()
    {
        OxideStatus status = NativeMethods.oxide_content_manager_destroy(NativeHandle);

        return status == OxideStatus.Ok;
    }
}