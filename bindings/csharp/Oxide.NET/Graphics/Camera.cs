using Oxide.Native;
using Oxide.Math;

namespace Oxide.Graphics;

public sealed class Camera
{
    private readonly CameraSafeHandle
        _handle;

    public Camera()
    {
        NativeMethods.Check(
            NativeMethods
                .oxide_camera_create(
                    out NativeCameraHandle handle
                )
        );

        _handle =
            new CameraSafeHandle(
                handle
            );
    }

    internal NativeCameraHandle Handle
    {
        get
        {
            if (_handle.IsInvalid
                || _handle.IsClosed)
            {
                throw new ObjectDisposedException(
                    nameof(Camera)
                );
            }

            return _handle.NativeHandle;
        }
    }

    public Vec3 Position
    {
        get
        {
            NativeMethods.Check(
                NativeMethods
                    .oxide_camera_get_position(
                        Handle,
                        out Vec3 result
                    )
            );

            return result;
        }

        set
        {
            NativeMethods.Check(
                NativeMethods
                    .oxide_camera_set_position(
                        Handle,
                        value
                    )
            );
        }
    }

    public Quaternion Rotation
    {
        get
        {
            NativeMethods.Check(
                NativeMethods
                    .oxide_camera_get_rotation(
                        Handle,
                        out Quaternion result
                    )
            );

            return result;
        }

        set
        {
            NativeMethods.Check(
                NativeMethods
                    .oxide_camera_set_rotation(
                        Handle,
                        value
                    )
            );
        }
    }

    public Angle FieldOfView
    {
        get
        {
            NativeMethods.Check(
                NativeMethods
                    .oxide_camera_get_field_of_view(
                        Handle,
                        out Angle result
                    )
            );

            return result;
        }

        set
        {
            NativeMethods.Check(
                NativeMethods
                    .oxide_camera_set_field_of_view(
                        Handle,
                        value
                    )
            );
        }
    }

    public void TranslateBy(
        Vec3 translation
    )
    {
        NativeMethods.Check(
            NativeMethods
                .oxide_camera_translate_by(
                    Handle,
                    translation
                )
        );
    }

    public void SetEuler(
        Angle pitch,
        Angle yaw,
        Angle roll
    )
    {
        NativeMethods.Check(
            NativeMethods
                .oxide_camera_set_euler(
                    Handle,
                    pitch,
                    yaw,
                    roll
                )
        );
    }

    public void SetClipPlanes(
        float nearPlane,
        float farPlane
    )
    {
        NativeMethods.Check(
            NativeMethods
                .oxide_camera_set_clip_planes(
                    Handle,
                    nearPlane,
                    farPlane
                )
        );
    }

    public float NearPlane
    {
        get
        {
            NativeMethods.Check(
                NativeMethods
                    .oxide_camera_get_near_plane(
                        Handle,
                        out float result
                    )
            );

            return result;
        }
    }

    public float FarPlane
    {
        get
        {
            NativeMethods.Check(
                NativeMethods
                    .oxide_camera_get_far_plane(
                        Handle,
                        out float result
                    )
            );

            return result;
        }
    }

    public void RotatePitchBy(
        Angle angle
    )
    {
        NativeMethods.Check(
            NativeMethods
                .oxide_camera_rotate_pitch_by(
                    Handle,
                    angle
                )
        );
    }

    public void RotateYawBy(
        Angle angle
    )
    {
        NativeMethods.Check(
            NativeMethods
                .oxide_camera_rotate_yaw_by(
                    Handle,
                    angle
                )
        );
    }

    public void RotateRollBy(
        Angle angle
    )
    {
        NativeMethods.Check(
            NativeMethods
                .oxide_camera_rotate_roll_by(
                    Handle,
                    angle
                )
        );
    }
}