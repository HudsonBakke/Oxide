using System;

using Oxide.Native;

namespace Oxide;

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
}