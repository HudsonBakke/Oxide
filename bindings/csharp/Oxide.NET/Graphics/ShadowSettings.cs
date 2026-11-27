using System.Runtime.InteropServices;

using Oxide.Math;

namespace Oxide.Graphics;

/// <summary>Optional shadow quality, directional coverage, and local-light clipping.</summary>
/// <remarks>Use new ShadowSettings() for defaults; default(ShadowSettings) contains invalid zero coverage.</remarks>
[StructLayout(LayoutKind.Sequential)]
public struct ShadowSettings
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
    /// <summary>Positive near clipping distance for point and spot shadows, in world units; default 0.1.</summary>
    public float NearPlane;
    /// <summary>Far clipping distance for point and spot shadows, greater than NearPlane; default 60.</summary>
    public float FarPlane;

    /// <summary>Creates settings with origin center, 30 by 30 by 60 coverage, and a 1024-pixel map.</summary>
    public ShadowSettings()
    {
        Center = new Vec3(0.0f, 0.0f, 0.0f);
        Size = 30.0f;
        Depth = 60.0f;
        Resolution = 1024;
        DepthBias = 0.0001f;
        NormalBias = 0.01f;
        NearPlane = 0.1f;
        FarPlane = 60.0f;
    }
    internal readonly void Validate()
    {
        if (!float.IsFinite(Center.X) || !float.IsFinite(Center.Y) || !float.IsFinite(Center.Z) ||
            !float.IsFinite(Size) || Size <= 0 || !float.IsFinite(Depth) || Depth <= 0 ||
            Resolution < 128 || Resolution > 4096 || (Resolution & (Resolution - 1)) != 0 ||
            !float.IsFinite(DepthBias) || DepthBias < 0 || !float.IsFinite(NormalBias) || NormalBias < 0 ||
            !float.IsFinite(NearPlane) || NearPlane <= 0 || !float.IsFinite(FarPlane) || FarPlane <= NearPlane)
            throw new System.ArgumentException("Invalid shadow coverage, resolution, bias, or clipping distances.");
    }

}
