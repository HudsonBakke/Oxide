using System;

using Oxide.Native;
using Oxide.Math;

namespace Oxide.Graphics;

/// <summary>Rendering interface supplied to the draw callback; bind a camera before drawing geometry.</summary>
/// <remarks>Valid only during Game.Draw; do not retain it for later frames.</remarks>
public sealed class Graphics
{
    private NativeGraphicsHandle _handle;

    private bool _valid;

    /// <summary>Finite non-negative linear exposure multiplier; defaults to one. Zero produces black.</summary>
    /// <remarks>Persists across frames. The last value set during Draw applies to the entire surface and all viewports.</remarks>
    public float Exposure
    {
        get
        {
            EnsureValid();
            NativeMethods.Check(NativeMethods.oxide_graphics_get_exposure(_handle, out float value));
            return value;
        }
        set
        {
            EnsureValid();
            NativeMethods.Check(NativeMethods.oxide_graphics_set_exposure(_handle, value));
        }
    }

    internal Graphics(NativeGraphicsHandle handle)
    {
        _handle = handle;
        _valid = true;
    }

    /// <summary>Current client width in physical pixels; assignment requests a resize.</summary>
    public uint PreferredBackbufferWidth
    {
        get => NativeMethods.oxide_app_window_width();

        set {
            if (value == 0)
            {
                throw new ArgumentOutOfRangeException(
                    nameof(value), "Backbuffer width must be greater than zero.");
            }

            NativeMethods.Check(NativeMethods.oxide_app_set_preferred_backbuffer_size(
                value, NativeMethods.oxide_app_window_height()));
        }
    }
    /// <summary>Current client height in physical pixels; assignment requests a resize.</summary>
    public uint PreferredBackbufferHeight
    {
        get => NativeMethods.oxide_app_window_height();

        set {
            if (value == 0)
            {
                throw new ArgumentOutOfRangeException(
                    nameof(value), "Backbuffer height must be greater than zero.");
            }

            NativeMethods.Check(NativeMethods.oxide_app_set_preferred_backbuffer_size(
                NativeMethods.oxide_app_window_width(), value));
        }
    }
    /// <summary>Rectangle covering the entire render surface in physical pixels.</summary>
    public Rect FullRect
    {
        get {
            EnsureValid();

            NativeMethods.Check(NativeMethods.oxide_graphics_full_rect(_handle, out Rect result));

            return result;
        }
    }

    /// <summary>Sets ambient lighting, image-based lighting, and the independent visible background.</summary>
    public void SetEnvironment(Environment environment)
    {
        NativeMethods.Check(NativeMethods.oxide_graphics_set_environment_explicit(
            _handle, environment.AmbientColor, environment.AmbientIntensity,
            environment.EnvironmentTexture?.Handle ?? default,
            environment.EnvironmentTexture is not null, environment.EnvironmentIntensity,
            environment.BackgroundTexture?.Handle ?? default,
            environment.BackgroundTexture is not null, environment.BackgroundColor));
    }

    /// <summary>Removes environment lighting and its background settings.</summary>
    public void ClearEnvironment()
    {
        NativeMethods.Check(NativeMethods.oxide_graphics_clear_environment(_handle));
    }

    /// <summary>Removes lights added so far in the current frame.</summary>
    public void ClearLights()
    {
        EnsureValid();

        NativeMethods.Check(NativeMethods.oxide_graphics_clear_lights(_handle));
    }

    internal void Invalidate()
    {
        _valid = false;
        _handle = default;
    }

    private void EnsureValid()
    {
        if (!_valid)
        {
            throw new InvalidOperationException(
                "Graphics is only valid during the current Draw callback.");
        }
    }

    /// <summary>Binds a world-space camera surface, automatically displayed in screen-bound cameras.</summary>
    /// <remarks>Default image size is 512 pixels on the longest edge. The normal selects the front; feed cameras omit monitor surfaces. Bind visible surfaces each frame, at most 16 distinct rectangles. Equal rectangles share a feed and the last binding wins.</remarks>
    public void BindCamera(Camera camera, Rect3D destination)
    {
        ArgumentNullException.ThrowIfNull(camera);
        EnsureValid();
        NativeMethods.Check(NativeMethods.oxide_graphics_bind_camera_3d(_handle, camera.Handle, destination));
    }

    /// <summary>Binds a world-space camera surface with image dimensions 1..2048, or both zero for automatic size.</summary>
    /// <remarks>Camera aspect follows the world rectangle. One rectangle cannot use different resolutions in the same frame.</remarks>
    public void BindCamera(Camera camera, Rect3D destination, uint width, uint height)
    {
        ArgumentNullException.ThrowIfNull(camera);
        EnsureValid();
        NativeMethods.Check(NativeMethods.oxide_graphics_bind_camera_3d_with_resolution(_handle, camera.Handle, destination, width, height));
    }

    /// <summary>Binds a camera and a positive-size pixel viewport for subsequent draw calls.</summary>
    public void BindCamera(Camera camera, Rect destination)
    {
        ArgumentNullException.ThrowIfNull(camera);

        EnsureValid();

        NativeMethods.Check(
            NativeMethods.oxide_graphics_bind_camera(_handle, camera.Handle, destination));
    }

    /// <summary>Adds a point, spot, or directional light for subsequent draws; lights reset at frame start.</summary>
    public void AddLight(PointLight light)
    {
        EnsureValid();

        NativeMethods.Check(NativeMethods.oxide_graphics_add_point_light(_handle, light));
    }

    /// <summary>Adds a point, spot, or directional light for subsequent draws; lights reset at frame start.</summary>
    public void AddLight(SpotLight light)
    {
        EnsureValid();

        NativeMethods.Check(NativeMethods.oxide_graphics_add_spot_light(_handle, light));
    }

    /// <summary>Adds a point, spot, or directional light for subsequent draws; lights reset at frame start.</summary>
    public void AddLight(DirectionalLight light)
    {
        EnsureValid();

        NativeMethods.Check(NativeMethods.oxide_graphics_add_directional_light(_handle, light));
    }

    /// <summary>Adds any supported light through the shared light interface, including its shadow state.</summary>
    public void AddLight(ILightSource light)
    {
        ArgumentNullException.ThrowIfNull(light);
        switch (light)
        {
            case PointLight point: AddLight(point); break;
            case SpotLight spot: AddLight(spot); break;
            case DirectionalLight directional: AddLight(directional); break;
            default: throw new ArgumentException("Unsupported light implementation.", nameof(light));
        }
    }

    /// <summary>Submits a mesh with transparency from zero (opaque) to one (invisible and no shadow).</summary>
    /// <remarks>Requires a finite value in [0,1]. Visible opacity is one minus transparency; the same value controls shadow transmission. Captures draw state now for rendering at frame end. Up to eight transparent depths are retained per light ray. Assets may be disposed after submission. More than eight active lights of any type, 32 shadow projections per frame, or exhausted GPU descriptor capacity produces an exception; earlier queued draws remain valid.</remarks>
    public void DrawMesh(Mesh mesh, Material material, Transform transform, float transparency)
    {
        ArgumentNullException.ThrowIfNull(mesh);
        ArgumentNullException.ThrowIfNull(material);
        EnsureValid();
        NativeMethods.Check(NativeMethods.oxide_graphics_draw_mesh_with_transparency(
            _handle, mesh.Handle, material.Handle, transform, transparency));
    }

    /// <summary>Submits a mesh with opaque rendering and shadows; captures draw state now for rendering at frame end.</summary>
    public void DrawMesh(Mesh mesh, Material material, Transform transform)
    {
        ArgumentNullException.ThrowIfNull(mesh);

        ArgumentNullException.ThrowIfNull(material);

        EnsureValid();

        NativeMethods.Check(NativeMethods.oxide_graphics_draw_mesh(_handle, mesh.Handle,
                                                                   material.Handle, transform));
    }
}
