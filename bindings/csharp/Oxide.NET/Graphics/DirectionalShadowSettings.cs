using System.Runtime.InteropServices;

using Oxide.Math;

namespace Oxide.Graphics;

/// <summary>Orthographic directional shadow volume and receiver bias.</summary>
/// <remarks>Use new DirectionalShadowSettings() for defaults; default(DirectionalShadowSettings) contains invalid zero coverage.</remarks>
[StructLayout(LayoutKind.Sequential)]
public struct DirectionalShadowSettings
{
    /// <summary>Finite world-space center of the volume; defaults to the origin.</summary>
    public Vec3 Center;
    /// <summary>Finite positive full square width and height in world units; defaults to 30.</summary>
    public float Size;
    /// <summary>Finite positive full depth along light rays, centered on Center; defaults to 60.</summary>
    public float Depth;
    /// <summary>Power-of-two square map size from 128 through 4096 pixels; defaults to 1024.</summary>
    public uint Resolution;
    /// <summary>Finite non-negative comparison offset in normalized depth units; defaults to 0.0001.</summary>
    public float DepthBias;
    /// <summary>Finite non-negative receiver offset along the geometric normal in world units; defaults to 0.01.</summary>
    public float NormalBias;

    /// <summary>Creates settings with origin center, 30 by 30 by 60 coverage, and a 1024-pixel map.</summary>
    public DirectionalShadowSettings()
    {
        Center = new Vec3(0.0f, 0.0f, 0.0f);
        Size = 30.0f;
        Depth = 60.0f;
        Resolution = 1024;
        DepthBias = 0.0001f;
        NormalBias = 0.01f;
    }
    /// <summary>Converts legacy directional settings to common shadow settings with default local-light clipping.</summary>
    public static implicit operator ShadowSettings(DirectionalShadowSettings settings) => new ShadowSettings {
        Center = settings.Center, Size = settings.Size, Depth = settings.Depth,
        Resolution = settings.Resolution, DepthBias = settings.DepthBias, NormalBias = settings.NormalBias,
    };

}
