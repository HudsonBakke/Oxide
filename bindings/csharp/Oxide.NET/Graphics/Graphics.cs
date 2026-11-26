using System;

using Oxide.Native;
using Oxide.Math;

namespace Oxide.Graphics;

/// <summary>Per-frame rendering interface valid only during the current Draw callback.</summary>
public sealed class Graphics
{
    private NativeGraphicsHandle
        _handle;

    private bool
        _valid;

    internal Graphics(
        NativeGraphicsHandle handle
    )
    {
        _handle = handle;
        _valid = true;
    }

    /// <summary>Gets the current backbuffer width or requests a new width.</summary>
    public uint PreferredBackbufferWidth
    {
        get => NativeMethods.oxide_app_window_width();

        set
        {
            if (value == 0)
            {
                throw new ArgumentOutOfRangeException(
                    nameof(value),
                    "Backbuffer width must be greater than zero."
                );
            }

            NativeMethods.Check(
                NativeMethods.oxide_app_set_preferred_backbuffer_size(
                    value,
                    NativeMethods.oxide_app_window_height()
                )
            );
        }
    }

    /// <summary>Gets the current backbuffer height or requests a new height.</summary>
    public uint PreferredBackbufferHeight
    {
        get => NativeMethods.oxide_app_window_height();

        set
        {
            if (value == 0)
            {
                throw new ArgumentOutOfRangeException(
                    nameof(value),
                    "Backbuffer height must be greater than zero."
                );
            }

            NativeMethods.Check(
                NativeMethods.oxide_app_set_preferred_backbuffer_size(
                    NativeMethods.oxide_app_window_width(),
                    value
                )
            );
        }
    }

    /// <summary>Rectangle covering the current backbuffer.</summary>
    public Rect FullRect
    {
        get
        {
            EnsureValid();

            NativeMethods.Check(
                NativeMethods
                    .oxide_graphics_full_rect(
                        _handle,
                        out Rect result
                    )
            );

            return result;
        }
    }

        /// <summary>Sets ambient lighting, IBL, and background rendering for the current scene.</summary>
public void SetEnvironment(
        Environment environment
    )
    {
        NativeMethods.Check(
            NativeMethods.oxide_graphics_set_environment_explicit(
                _handle,
                environment.AmbientColor,
                environment.AmbientIntensity,
                environment.EnvironmentTexture?.Handle ?? default,
                environment.EnvironmentTexture is not null,
                environment.EnvironmentIntensity,
                environment.BackgroundTexture?.Handle ?? default,
                environment.BackgroundTexture is not null,
                environment.BackgroundColor
            )
        );
    }

        /// <summary>Removes the current scene environment.</summary>
public void ClearEnvironment()
    {
        NativeMethods.Check(
            NativeMethods.oxide_graphics_clear_environment(
                _handle
            )
        );
    }

        /// <summary>Removes lights accumulated for the current frame.</summary>
public void ClearLights()
    {
        EnsureValid();

        NativeMethods.Check(
            NativeMethods
                .oxide_graphics_clear_lights(
                    _handle
                )
        );
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
                "Graphics is only valid during the current Draw callback."
            );
        }
    }

        /// <summary>Binds a camera to the specified destination rectangle for subsequent draws.</summary>
public void BindCamera(
        Camera camera,
        Rect destination
    )
    {
        ArgumentNullException.ThrowIfNull(
            camera
        );

        EnsureValid();

        NativeMethods.Check(
            NativeMethods
                .oxide_graphics_bind_camera(
                    _handle,
                    camera.Handle,
                    destination
                )
        );
    }

    /// <summary>Adds a light to the current frame's lighting state.</summary>
    public void AddLight(
        PointLight light
    )
    {
        EnsureValid();

        NativeMethods.Check(
            NativeMethods
                .oxide_graphics_add_point_light(
                    _handle,
                    light
                )
        );
    }

    /// <summary>Adds a light to the current frame's lighting state.</summary>
    public void AddLight(
        SpotLight light
    )
    {
        EnsureValid();

        NativeMethods.Check(
            NativeMethods
                .oxide_graphics_add_spot_light(
                    _handle,
                    light
                )
        );
    }

    /// <summary>Adds a light to the current frame's lighting state.</summary>
    public void AddLight(
        DirectionalLight light
    )
    {
        EnsureValid();

        NativeMethods.Check(
            NativeMethods
                .oxide_graphics_add_directional_light(
                    _handle,
                    light
                )
        );
    }

        /// <summary>Draws a mesh with the supplied material and transform.</summary>
public void DrawMesh(
        Mesh mesh,
        Material material,
        Transform transform
    )
    {
        ArgumentNullException.ThrowIfNull(
            mesh
        );

        ArgumentNullException.ThrowIfNull(
            material
        );

        EnsureValid();

        NativeMethods.Check(
            NativeMethods
                .oxide_graphics_draw_mesh(
                    _handle,
                    mesh.Handle,
                    material.Handle,
                    transform
                )
        );
    }
}