using Oxide.Content;
using Oxide.Native;

namespace Oxide.Graphics;

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