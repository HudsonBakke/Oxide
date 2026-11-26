using System;

using Oxide.Native;
using Oxide.Math;

namespace Oxide.Graphics;

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
                environment.BackgroundColor
            )
        );
    }

    public void ClearEnvironment()
    {
        NativeMethods.Check(
            NativeMethods.oxide_graphics_clear_environment(
                _handle
            )
        );
    }

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