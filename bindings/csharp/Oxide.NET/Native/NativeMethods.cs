using System;
using System.Runtime.InteropServices;

namespace Oxide.Native;

internal static class NativeMethods
{
    [DllImport(
        "oxide",
        CallingConvention =
            CallingConvention.Cdecl
    )]
    internal static extern OxideStatus
        oxide_run(
            NativeAppConfig config,
            NativeGameCallbacks callbacks
        );

    [DllImport(
        "oxide",
        CallingConvention =
            CallingConvention.Cdecl
    )]
    internal static extern IntPtr
        oxide_last_error_message();

    internal static void Check(
        OxideStatus status
    )
    {
        if (status == OxideStatus.Ok)
        {
            return;
        }

        IntPtr messagePtr =
            oxide_last_error_message();

        string message =
            messagePtr == IntPtr.Zero
                ? $"Oxide failed with status {status}."
                : Marshal.PtrToStringUTF8(
                    messagePtr
                )
                ?? $"Oxide failed with status {status}.";

        throw new OxideException(
            status,
            message
        );
    }

    [DllImport(
        "oxide",
        CallingConvention =
            CallingConvention.Cdecl
    )]
    internal static extern OxideStatus
        oxide_graphics_full_rect(
            NativeGraphicsHandle graphics,
            out Rect result
        );

    [DllImport(
        "oxide",
        CallingConvention =
            CallingConvention.Cdecl
    )]
    internal static extern OxideStatus
        oxide_graphics_clear_lights(
            NativeGraphicsHandle graphics
        );

    [DllImport("oxide", CallingConvention = CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_camera_create(
        out NativeCameraHandle result
    );

    [DllImport("oxide", CallingConvention = CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_camera_destroy(
        NativeCameraHandle camera
    );

    [DllImport("oxide", CallingConvention = CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_camera_get_position(
        NativeCameraHandle camera,
        out Vec3 result
    );

    [DllImport("oxide", CallingConvention = CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_camera_set_position(
        NativeCameraHandle camera,
        Vec3 position
    );

    [DllImport("oxide", CallingConvention = CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_camera_translate_by(
        NativeCameraHandle camera,
        Vec3 translation
    );

    [DllImport("oxide", CallingConvention = CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_camera_get_rotation(
        NativeCameraHandle camera,
        out Quaternion result
    );

    [DllImport("oxide", CallingConvention = CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_camera_set_rotation(
        NativeCameraHandle camera,
        Quaternion rotation
    );

    [DllImport("oxide", CallingConvention = CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_camera_set_euler(
        NativeCameraHandle camera,
        Angle pitch,
        Angle yaw,
        Angle roll
    );

    [DllImport("oxide", CallingConvention = CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_camera_get_field_of_view(
        NativeCameraHandle camera,
        out Angle result
    );

    [DllImport("oxide", CallingConvention = CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_camera_set_field_of_view(
        NativeCameraHandle camera,
        Angle fieldOfView
    );

    [DllImport("oxide", CallingConvention = CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_camera_set_clip_planes(
        NativeCameraHandle camera,
        float nearPlane,
        float farPlane
    );

    [DllImport("oxide", CallingConvention = CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_graphics_bind_camera(
        NativeGraphicsHandle graphics,
        NativeCameraHandle camera,
        Rect destination
    );

    [DllImport(
        "oxide",
        CallingConvention =
            CallingConvention.Cdecl
    )]
    internal static extern OxideStatus
        oxide_graphics_add_point_light(
            NativeGraphicsHandle graphics,
            PointLight light
        );

    [DllImport(
        "oxide",
        CallingConvention =
            CallingConvention.Cdecl
    )]
    internal static extern OxideStatus
        oxide_graphics_add_spot_light(
            NativeGraphicsHandle graphics,
            SpotLight light
        );

    [DllImport(
        "oxide",
        CallingConvention =
            CallingConvention.Cdecl
    )]
    internal static extern OxideStatus
        oxide_graphics_add_directional_light(
            NativeGraphicsHandle graphics,
            DirectionalLight light
        );
}

