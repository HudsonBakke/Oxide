using System.Runtime.InteropServices;

using Oxide.Graphics;
using Oxide.Native;

namespace Oxide.Content;

/// <summary>Loads source assets and reuses disposable compiled caches, rebuilding them when source dependencies change.</summary>
/// <remarks>Owns a native handle released by a SafeHandle finalizer. Native failures raise OxideException.</remarks>
public sealed class ContentManager
{
    private readonly ContentManagerSafeHandle _handle;
    /// <summary>Directory containing disposable compiled asset caches; relative paths use the working directory.</summary>
    public unsafe string CacheDirectory
    {
        get {
            NativeMethods.Check(NativeMethods.oxide_content_manager_cache_directory_length(
                Handle, out nuint length));

            int byteCount = checked((int)length);

            // Include space for the terminating null.
            var buffer = new byte[byteCount + 1];

            fixed(byte *pointer = buffer)
            {
                NativeMethods.Check(NativeMethods.oxide_content_manager_copy_cache_directory(
                    Handle, pointer, (nuint)buffer.Length));
            }

            return System.Text.Encoding.UTF8.GetString(buffer, 0, byteCount);
        }
    }

    /// <summary>Creates a loader using the default target/oxide-content cache, or a supplied cache directory.</summary>
    public ContentManager()
    {
        NativeMethods.Check(
            NativeMethods.oxide_content_manager_create(out NativeContentManagerHandle handle));

        _handle = new ContentManagerSafeHandle(handle);
    }

    /// <summary>Creates a loader using the default target/oxide-content cache, or a supplied cache directory.</summary>
    public ContentManager(string cacheDirectory)
    {
        ArgumentException.ThrowIfNullOrEmpty(cacheDirectory);

        IntPtr nativePath = Marshal.StringToCoTaskMemUTF8(cacheDirectory);

        try
        {
            NativeMethods.Check(NativeMethods.oxide_content_manager_create_with_cache_directory(
                nativePath, out NativeContentManagerHandle handle));

            _handle = new ContentManagerSafeHandle(handle);
        }
        finally
        {
            Marshal.FreeCoTaskMem(nativePath);
        }
    }

    internal NativeContentManagerHandle Handle
    {
        get {
            if (_handle.IsInvalid || _handle.IsClosed)
            {
                throw new ObjectDisposedException(nameof(ContentManager));
            }

            return _handle.NativeHandle;
        }
    }
    /// <summary>Loads a Mesh, Material, or Texture from a source path, optionally selecting a named glTF subobject.</summary>
    public T Load<T>(string path, string? subObject = null)
        where T : IContentAsset
    {
        ArgumentException.ThrowIfNullOrEmpty(path);

        if (string.IsNullOrEmpty(subObject))
        {
            return LoadDefault<T>(path);
        }

        return LoadSubObject<T>(path, subObject);
    }

    private T LoadDefault<T>(string path)
        where T : IContentAsset
    {
        IntPtr nativePath = Marshal.StringToCoTaskMemUTF8(path);

        try
        {
            object result;

            if (typeof(T) == typeof(Mesh))
            {
                NativeMethods.Check(NativeMethods.oxide_content_load_mesh(
                    Handle, nativePath, out NativeMeshHandle handle));

                result = new Mesh(handle);
            }
            else if (typeof(T) == typeof(Material))
            {
                NativeMethods.Check(NativeMethods.oxide_content_load_material(
                    Handle, nativePath, out NativeMaterialHandle handle));

                result = new Material(handle);
            }
            else if (typeof(T) == typeof(Texture))
            {
                NativeMethods.Check(NativeMethods.oxide_content_load_texture(
                    Handle, nativePath, out NativeTextureHandle handle));

                result = new Texture(handle);
            }
            else
            {
                throw new NotSupportedException(
                    $"{typeof(T).Name} is not a supported Oxide content type.");
            }

            return (T)result;
        }
        finally
        {
            Marshal.FreeCoTaskMem(nativePath);
        }
    }

    private T LoadSubObject<T>(string path, string subObject)
        where T : IContentAsset
    {
        IntPtr nativePath = Marshal.StringToCoTaskMemUTF8(path);

        IntPtr nativeSubObject = Marshal.StringToCoTaskMemUTF8(subObject);

        try
        {
            object result;

            if (typeof(T) == typeof(Mesh))
            {
                NativeMethods.Check(NativeMethods.oxide_content_load_mesh_sub(
                    Handle, nativePath, nativeSubObject, out NativeMeshHandle handle));

                result = new Mesh(handle);
            }
            else if (typeof(T) == typeof(Material))
            {
                NativeMethods.Check(NativeMethods.oxide_content_load_material_sub(
                    Handle, nativePath, nativeSubObject, out NativeMaterialHandle handle));

                result = new Material(handle);
            }
            else if (typeof(T) == typeof(Texture))
            {
                NativeMethods.Check(NativeMethods.oxide_content_load_texture_sub(
                    Handle, nativePath, nativeSubObject, out NativeTextureHandle handle));

                result = new Texture(handle);
            }
            else
            {
                throw new NotSupportedException(
                    $"{typeof(T).Name} is not a supported Oxide content type.");
            }

            return (T)result;
        }
        finally
        {
            Marshal.FreeCoTaskMem(nativeSubObject);

            Marshal.FreeCoTaskMem(nativePath);
        }
    }
}
