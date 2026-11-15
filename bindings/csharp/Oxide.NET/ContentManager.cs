using Oxide.Native;

namespace Oxide;

public sealed class ContentManager
{
    private readonly
        ContentManagerSafeHandle
        _handle;

    public ContentManager()
    {
        NativeMethods.Check(
            NativeMethods
                .oxide_content_manager_create(
                    out NativeContentManagerHandle handle
                )
        );

        _handle =
            new ContentManagerSafeHandle(
                handle
            );
    }

    internal NativeContentManagerHandle Handle
    {
        get
        {
            if (_handle.IsInvalid
                || _handle.IsClosed)
            {
                throw new ObjectDisposedException(
                    nameof(ContentManager)
                );
            }

            return _handle.NativeHandle;
        }
    }
}