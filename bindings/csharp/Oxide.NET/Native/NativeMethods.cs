using System;
using System.Runtime.InteropServices;

using Oxide.Graphics;
using Oxide.Math;

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

    [DllImport(
        "oxide",
        CallingConvention =
            CallingConvention.Cdecl
    )]
    internal static extern OxideStatus
        oxide_content_manager_create(
            out NativeContentManagerHandle result
        );

    [DllImport(
        "oxide",
        CallingConvention =
            CallingConvention.Cdecl
    )]
    internal static extern OxideStatus
        oxide_content_manager_destroy(
            NativeContentManagerHandle manager
        );

    [DllImport("oxide", CallingConvention = CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_content_load_mesh(
        NativeContentManagerHandle manager,
        IntPtr path,
        out NativeMeshHandle result
    );

    [DllImport("oxide", CallingConvention = CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_content_load_material(
        NativeContentManagerHandle manager,
        IntPtr path,
        out NativeMaterialHandle result
    );

    [DllImport("oxide", CallingConvention = CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_mesh_destroy(
        NativeMeshHandle mesh
    );

    [DllImport("oxide", CallingConvention = CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_material_destroy(
        NativeMaterialHandle material
    );

    [DllImport("oxide", CallingConvention = CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_graphics_draw_mesh(
        NativeGraphicsHandle graphics,
        NativeMeshHandle mesh,
        NativeMaterialHandle material,
        Transform transform
    );

    [DllImport("oxide", CallingConvention = CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_mesh_vertex_count(
        NativeMeshHandle mesh,
        out nuint result
    );

    [DllImport("oxide", CallingConvention = CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_mesh_index_count(
        NativeMeshHandle mesh,
        out nuint result
    );

    [DllImport("oxide", CallingConvention = CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_mesh_get_pivot(
        NativeMeshHandle mesh,
        out Vec3 result
    );

    [DllImport("oxide", CallingConvention = CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_mesh_set_pivot(
        NativeMeshHandle mesh,
        Vec3 pivot
    );

    [DllImport("oxide", CallingConvention = CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_mesh_recalculate_pivot(
        NativeMeshHandle mesh
    );

    [DllImport("oxide", CallingConvention = CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_mesh_get_direction(
        NativeMeshHandle mesh,
        out Vec3 result
    );

    [DllImport("oxide", CallingConvention = CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_mesh_get_reference(
        NativeMeshHandle mesh,
        out Vec3 result
    );

    [DllImport("oxide", CallingConvention = CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_mesh_get_side(
        NativeMeshHandle mesh,
        out Vec3 result
    );

    [DllImport("oxide", CallingConvention = CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_mesh_set_direction(
        NativeMeshHandle mesh,
        Vec3 direction
    );

    [DllImport("oxide", CallingConvention = CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_mesh_set_reference(
        NativeMeshHandle mesh,
        Vec3 reference
    );

    [DllImport("oxide", CallingConvention = CallingConvention.Cdecl)]
    internal static extern OxideStatus oxide_mesh_set_orientation(
        NativeMeshHandle mesh,
        Vec3 direction,
        Vec3 reference
    );

    [DllImport("oxide", CallingConvention = CallingConvention.Cdecl)]
    internal static extern unsafe OxideStatus oxide_mesh_copy_vertices(
        NativeMeshHandle mesh,
        Vertex* destination,
        nuint capacity
    );

    [DllImport("oxide", CallingConvention = CallingConvention.Cdecl)]
    internal static extern unsafe OxideStatus oxide_mesh_copy_indices(
        NativeMeshHandle mesh,
        uint* destination,
        nuint capacity
    );

    [DllImport("oxide", CallingConvention = CallingConvention.Cdecl)]
    internal static extern unsafe OxideStatus oxide_mesh_set_vertices(
        NativeMeshHandle mesh,
        Vertex* vertices,
        nuint count
    );

    [DllImport("oxide", CallingConvention = CallingConvention.Cdecl)]
    internal static extern unsafe OxideStatus oxide_mesh_set_indices(
        NativeMeshHandle mesh,
        uint* indices,
        nuint count
    );

    [DllImport(
        "oxide",
        CallingConvention =
            CallingConvention.Cdecl
    )]
    internal static extern unsafe OxideStatus
        oxide_mesh_create(
            Vertex* vertices,
            nuint vertexCount,

            uint* indices,
            nuint indexCount,

            out NativeMeshHandle result
        );
}

