using Oxide.Content;
using Oxide.Native;

namespace Oxide.Graphics;

public sealed class Material
    : IContentAsset
{
    private readonly MaterialSafeHandle
        _handle;

    public Material(
        Texture baseTexture
    )
    {
        ArgumentNullException.ThrowIfNull(
            baseTexture
        );

        NativeMethods.Check(
            NativeMethods
                .oxide_material_create(
                    baseTexture.Handle,
                    out NativeMaterialHandle handle
                )
        );

        _handle =
            new MaterialSafeHandle(
                handle
            );
    }

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

    public Texture BaseTexture
    {
        get
        {
            NativeMethods.Check(
                NativeMethods
                    .oxide_material_base_texture(
                        Handle,
                        out NativeTextureHandle handle
                    )
            );

            return new Texture(
                handle
            );
        }
    }

    public static Material Solid(
        Color color
    )
    {
        NativeMethods.Check(
            NativeMethods
                .oxide_material_solid(
                    color,
                    out NativeMaterialHandle handle
                )
        );

        return new Material(
            handle
        );
    }
}