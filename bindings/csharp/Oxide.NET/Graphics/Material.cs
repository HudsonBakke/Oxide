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


    public Color BaseColor
    {
        get
        {
            NativeMethods.Check(NativeMethods.oxide_material_get_base_color(Handle, out Color value));
            return value;
        }
        set => NativeMethods.Check(NativeMethods.oxide_material_set_base_color(Handle, value));
    }

    public float Metallic
    {
        get
        {
            NativeMethods.Check(NativeMethods.oxide_material_get_metallic(Handle, out float value));
            return value;
        }
        set => NativeMethods.Check(NativeMethods.oxide_material_set_metallic(Handle, value));
    }

    public float Roughness
    {
        get
        {
            NativeMethods.Check(NativeMethods.oxide_material_get_roughness(Handle, out float value));
            return value;
        }
        set => NativeMethods.Check(NativeMethods.oxide_material_set_roughness(Handle, value));
    }

    public Color EmissiveColor
    {
        get
        {
            NativeMethods.Check(NativeMethods.oxide_material_get_emissive_color(Handle, out Color value));
            return value;
        }
        set => NativeMethods.Check(NativeMethods.oxide_material_set_emissive_color(Handle, value));
    }

    public float EmissiveStrength
    {
        get
        {
            NativeMethods.Check(NativeMethods.oxide_material_get_emissive_strength(Handle, out float value));
            return value;
        }
        set => NativeMethods.Check(NativeMethods.oxide_material_set_emissive_strength(Handle, value));
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