using Oxide.Content;
using Oxide.Native;

namespace Oxide.Graphics;

/// <summary>Surface settings for base color, metallicity, roughness, emission, and tangent-space normal mapping.</summary>
/// <remarks>Owns a native handle released by a SafeHandle finalizer. Native failures raise OxideException.</remarks>
public sealed class Material : IContentAsset
{
    private readonly MaterialSafeHandle _handle;

    /// <summary>Creates a material using a base-color texture, metallic factor zero, roughness one, and no emission.</summary>
    public Material(Texture baseTexture)
    {
        ArgumentNullException.ThrowIfNull(baseTexture);

        NativeMethods.Check(NativeMethods.oxide_material_create(baseTexture.Handle,
                                                                out NativeMaterialHandle handle));

        _handle = new MaterialSafeHandle(handle);
    }

    internal Material(NativeMaterialHandle handle)
    {
        _handle = new MaterialSafeHandle(handle);
    }

    internal NativeMaterialHandle Handle
    {
        get {
            if (_handle.IsInvalid || _handle.IsClosed)
            {
                throw new ObjectDisposedException(nameof(Material));
            }

            return _handle.NativeHandle;
        }
    }
    /// <summary>Base-color texture sampled by the material.</summary>
    public Texture BaseTexture
    {
        get {
            NativeMethods.Check(
                NativeMethods.oxide_material_base_texture(Handle, out NativeTextureHandle handle));

            return new Texture(handle);
        }
    }
    /// <summary>Tangent-space normal map; RGB encodes a direction from -1 to 1.</summary>
    public Texture NormalTexture
    {
        get {
            NativeMethods.Check(NativeMethods.oxide_material_normal_texture(
                Handle, out NativeTextureHandle handle));
            return new Texture(handle);
        }
        set {
            ArgumentNullException.ThrowIfNull(value);
            NativeMethods.Check(NativeMethods.oxide_material_set_normal_texture(Handle,
                                                                                value.Handle));
        }
    }
    /// <summary>Finite multiplier applied to the normal map's XY components; zero disables perturbation.</summary>
    public float NormalStrength
    {
        get {
            NativeMethods.Check(NativeMethods.oxide_material_get_normal_strength(Handle,
                                                                                 out float value));
            return value;
        }
        set => NativeMethods.Check(NativeMethods.oxide_material_set_normal_strength(Handle, value));
    }
    /// <summary>RGBA multiplier applied to the base-color texture.</summary>
    public Color BaseColor
    {
        get {
            NativeMethods.Check(NativeMethods.oxide_material_get_base_color(Handle,
                                                                            out Color value));
            return value;
        }
        set => NativeMethods.Check(NativeMethods.oxide_material_set_base_color(Handle, value));
    }
    /// <summary>Metallic factor in [0, 1]: zero is dielectric and one is metallic.</summary>
    public float Metallic
    {
        get {
            NativeMethods.Check(NativeMethods.oxide_material_get_metallic(Handle, out float value));
            return value;
        }
        set => NativeMethods.Check(NativeMethods.oxide_material_set_metallic(Handle, value));
    }
    /// <summary>Perceptual roughness in [0, 1]: zero is smooth and one is fully rough.</summary>
    public float Roughness
    {
        get {
            NativeMethods.Check(NativeMethods.oxide_material_get_roughness(Handle,
                                                                           out float value));
            return value;
        }
        set => NativeMethods.Check(NativeMethods.oxide_material_set_roughness(Handle, value));
    }
    /// <summary>RGB color emitted by the surface, multiplied by emissive strength.</summary>
    public Color EmissiveColor
    {
        get {
            NativeMethods.Check(NativeMethods.oxide_material_get_emissive_color(Handle,
                                                                                out Color value));
            return value;
        }
        set => NativeMethods.Check(NativeMethods.oxide_material_set_emissive_color(Handle, value));
    }
    /// <summary>Finite, non-negative multiplier applied to the emissive color.</summary>
    public float EmissiveStrength
    {
        get {
            NativeMethods.Check(
                NativeMethods.oxide_material_get_emissive_strength(Handle, out float value));
            return value;
        }
        set =>
            NativeMethods.Check(NativeMethods.oxide_material_set_emissive_strength(Handle, value));
    }

    /// <summary>Creates an untextured-looking material from a solid-color base texture.</summary>
    public static Material Solid(Color color)
    {
        NativeMethods.Check(
            NativeMethods.oxide_material_solid(color, out NativeMaterialHandle handle));

        return new Material(handle);
    }
}
