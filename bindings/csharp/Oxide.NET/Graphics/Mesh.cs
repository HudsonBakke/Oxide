using Oxide.Native;

namespace Oxide.Graphics;

public sealed class Mesh
{
    private readonly MeshSafeHandle
        _handle;

    internal Mesh(
        NativeMeshHandle handle
    )
    {
        _handle =
            new MeshSafeHandle(
                handle
            );
    }

    internal NativeMeshHandle Handle
    {
        get
        {
            if (_handle.IsInvalid
                || _handle.IsClosed)
            {
                throw new ObjectDisposedException(
                    nameof(Mesh)
                );
            }

            return _handle.NativeHandle;
        }
    }
}