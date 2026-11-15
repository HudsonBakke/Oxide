using Oxide.Native;

namespace Oxide.Graphics;

public sealed class Material
{
    private readonly MaterialSafeHandle
        _handle;

    internal Material(
        NativeMaterialHandle handle
    )
    {
        _handle =
            new MaterialSafeHandle(
                handle
            );
    }

    internal NativeMaterialHandle Handle
    {
        get
        {
            if (_handle.IsInvalid
                || _handle.IsClosed)
            {
                throw new ObjectDisposedException(
                    nameof(Material)
                );
            }

            return _handle.NativeHandle;
        }
    }
}