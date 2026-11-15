using Oxide.Native;
using System.Runtime.InteropServices;

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

    public Mesh LoadMesh(
        string path
    )
    {
        ArgumentException.ThrowIfNullOrEmpty(
            path
        );

        IntPtr nativePath =
            Marshal.StringToCoTaskMemUTF8(
                path
            );

        try
        {
            NativeMethods.Check(
                NativeMethods
                    .oxide_content_load_mesh(
                        Handle,
                        nativePath,
                        out NativeMeshHandle handle
                    )
            );

            return new Mesh(
                handle
            );
        }
        finally
        {
            Marshal.FreeCoTaskMem(
                nativePath
            );
        }
    }

    public Material LoadMaterial(
        string path
    )
    {
        ArgumentException.ThrowIfNullOrEmpty(
            path
        );

        IntPtr nativePath =
            Marshal.StringToCoTaskMemUTF8(
                path
            );

        try
        {
            NativeMethods.Check(
                NativeMethods
                    .oxide_content_load_material(
                        Handle,
                        nativePath,
                        out NativeMaterialHandle handle
                    )
            );

            return new Material(
                handle
            );
        }
        finally
        {
            Marshal.FreeCoTaskMem(
                nativePath
            );
        }
    }
}