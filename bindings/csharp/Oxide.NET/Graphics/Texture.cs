using Oxide.Content;
using Oxide.Native;

namespace Oxide.Graphics;

/// <summary>Immutable 2D texture resource loaded through ContentManager or created procedurally.</summary>
public sealed class Texture
    : IContentAsset
{
    private readonly TextureSafeHandle
        _handle;

    internal Texture(
        NativeTextureHandle handle
    )
    {
        _handle =
            new TextureSafeHandle(
                handle
            );
    }

    internal NativeTextureHandle Handle
    {
        get
        {
            if (_handle.IsInvalid
                || _handle.IsClosed)
            {
                throw new ObjectDisposedException(
                    nameof(Texture)
                );
            }

            return _handle.NativeHandle;
        }
    }

    /// <summary>Texture width in pixels.</summary>
    public uint Width
    {
        get
        {
            NativeMethods.Check(
                NativeMethods
                    .oxide_texture_width(
                        Handle,
                        out uint result
                    )
            );

            return result;
        }
    }

    /// <summary>Texture height in pixels.</summary>
    public uint Height
    {
        get
        {
            NativeMethods.Check(
                NativeMethods
                    .oxide_texture_height(
                        Handle,
                        out uint result
                    )
            );

            return result;
        }
    }


    /// <summary>Creates or accesses Texture.</summary>
    public static unsafe Texture FromRgba(
        uint width,
        uint height,
        byte[] pixels
    )
    {
        ArgumentNullException.ThrowIfNull(pixels);

        fixed (byte* pointer = pixels)
        {
            NativeMethods.Check(
                NativeMethods.oxide_texture_create_rgba(
                    width,
                    height,
                    pointer,
                    (nuint)pixels.Length,
                    out NativeTextureHandle handle
                )
            );

            return new Texture(handle);
        }
    }

        /// <summary>Creates a one-pixel texture containing a solid color.</summary>
public static Texture Solid(
        Color color
    )
    {
        NativeMethods.Check(
            NativeMethods
                .oxide_texture_solid(
                    color,
                    out NativeTextureHandle handle
                )
        );

        return new Texture(
            handle
        );
    }
}