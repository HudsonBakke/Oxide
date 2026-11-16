using Oxide.Native;
using Oxide.Graphics;
using System.Runtime.InteropServices;

namespace Oxide.Content;

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

    public T Load<T>(
        string path,
        string? subObject = null
    )
        where T : IContentAsset
    {
        ArgumentException.ThrowIfNullOrEmpty(
            path
        );

        if (string.IsNullOrEmpty(
            subObject
        ))
        {
            return LoadDefault<T>(
                path
            );
        }

        return LoadSubObject<T>(
            path,
            subObject
        );
    }

    private T LoadDefault<T>(
        string path
    )
        where T : IContentAsset
    {
        IntPtr nativePath =
            Marshal.StringToCoTaskMemUTF8(
                path
            );

        try
        {
            object result;

            if (typeof(T)
                == typeof(Mesh))
            {
                NativeMethods.Check(
                    NativeMethods
                        .oxide_content_load_mesh(
                            Handle,
                            nativePath,
                            out NativeMeshHandle handle
                        )
                );

                result =
                    new Mesh(handle);
            }
            else if (typeof(T)
                == typeof(Material))
            {
                NativeMethods.Check(
                    NativeMethods
                        .oxide_content_load_material(
                            Handle,
                            nativePath,
                            out NativeMaterialHandle handle
                        )
                );

                result =
                    new Material(handle);
            }
            else if (typeof(T)
                == typeof(Texture))
            {
                NativeMethods.Check(
                    NativeMethods
                        .oxide_content_load_texture(
                            Handle,
                            nativePath,
                            out NativeTextureHandle handle
                        )
                );

                result =
                    new Texture(handle);
            }
            else {
                throw new NotSupportedException(
                    $"{typeof(T).Name} is not a supported Oxide content type."
                );
            }

            return (T)result;
        }
        finally
        {
            Marshal.FreeCoTaskMem(
                nativePath
            );
        }
    }

    private T LoadSubObject<T>(
        string path,
        string subObject
    )
        where T : IContentAsset
    {
        IntPtr nativePath =
            Marshal.StringToCoTaskMemUTF8(
                path
            );

        IntPtr nativeSubObject =
            Marshal.StringToCoTaskMemUTF8(
                subObject
            );

        try
        {
            object result;

            if (typeof(T)
                == typeof(Mesh))
            {
                NativeMethods.Check(
                    NativeMethods
                        .oxide_content_load_mesh_sub(
                            Handle,
                            nativePath,
                            nativeSubObject,
                            out NativeMeshHandle handle
                        )
                );

                result =
                    new Mesh(handle);
            }
            else if (typeof(T)
                == typeof(Material))
            {
                NativeMethods.Check(
                    NativeMethods
                        .oxide_content_load_material_sub(
                            Handle,
                            nativePath,
                            nativeSubObject,
                            out NativeMaterialHandle handle
                        )
                );

                result =
                    new Material(handle);
            }
            else if (typeof(T)
                == typeof(Texture))
            {
                NativeMethods.Check(
                    NativeMethods
                        .oxide_content_load_texture_sub(
                            Handle,
                            nativePath,
                            nativeSubObject,
                            out NativeTextureHandle handle
                        )
                );

                result =
                    new Texture(handle);
            }
            else {
                throw new NotSupportedException(
                    $"{typeof(T).Name} is not a supported Oxide content type."
                );
            }

            return (T)result;
        }
        finally
        {
            Marshal.FreeCoTaskMem(
                nativeSubObject
            );

            Marshal.FreeCoTaskMem(
                nativePath
            );
        }
    }
}