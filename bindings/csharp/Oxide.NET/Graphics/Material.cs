using Oxide.Content;
using Oxide.Native;

namespace Oxide.Graphics;

/// <summary>Physically based surface material controlling color, metallicity, roughness, emission, and normal mapping.</summary>
public sealed class Material
    : IContentAsset
{
    private readonly MaterialSafeHandle
        _handle;

    /// <summary>Creates or accesses Material.</summary>
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

    /// <summary>Base-color texture sampled by the material.</summary>
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



    /// <summary>Tangent-space normal map sampled by the material.</summary>
    public Texture NormalTexture
    {
        get
        {
            NativeMethods.Check(
                NativeMethods.oxide_material_normal_texture(
                    Handle,
                    out NativeTextureHandle handle
                )
            );
            return new Texture(handle);
        }
        set
        {
            ArgumentNullException.ThrowIfNull(value);
            NativeMethods.Check(
                NativeMethods.oxide_material_set_normal_texture(
                    Handle,
                    value.Handle
                )
            );
        }
    }

    /// <summary>Strength of normal-map perturbation.</summary>
    public float NormalStrength
    {
        get
        {
            NativeMethods.Check(
                NativeMethods.oxide_material_get_normal_strength(
                    Handle,
                    out float value
                )
            );
            return value;
        }
        set => NativeMethods.Check(
            NativeMethods.oxide_material_set_normal_strength(Handle, value)
        );
    }

    /// <summary>Color multiplier applied to the base texture.</summary>
    public Color BaseColor
    {
        get
        {
            NativeMethods.Check(NativeMethods.oxide_material_get_base_color(Handle, out Color value));
            return value;
        }
        set => NativeMethods.Check(NativeMethods.oxide_material_set_base_color(Handle, value));
    }

    /// <summary>Metallic factor in the range 0 to 1.</summary>
    public float Metallic
    {
        get
        {
            NativeMethods.Check(NativeMethods.oxide_material_get_metallic(Handle, out float value));
            return value;
        }
        set => NativeMethods.Check(NativeMethods.oxide_material_set_metallic(Handle, value));
    }

    /// <summary>Perceptual roughness factor in the range 0 to 1.</summary>
    public float Roughness
    {
        get
        {
            NativeMethods.Check(NativeMethods.oxide_material_get_roughness(Handle, out float value));
            return value;
        }
        set => NativeMethods.Check(NativeMethods.oxide_material_set_roughness(Handle, value));
    }

    /// <summary>Color emitted by the material.</summary>
    public Color EmissiveColor
    {
        get
        {
            NativeMethods.Check(NativeMethods.oxide_material_get_emissive_color(Handle, out Color value));
            return value;
        }
        set => NativeMethods.Check(NativeMethods.oxide_material_set_emissive_color(Handle, value));
    }

    /// <summary>Multiplier applied to emissive color.</summary>
    public float EmissiveStrength
    {
        get
        {
            NativeMethods.Check(NativeMethods.oxide_material_get_emissive_strength(Handle, out float value));
            return value;
        }
        set => NativeMethods.Check(NativeMethods.oxide_material_set_emissive_strength(Handle, value));
    }

    /// <summary>Creates a material backed by a solid-color texture.</summary>
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