using Oxide.Native;
using Oxide.Math;

namespace Oxide.Graphics;

/// <summary>Perspective camera defining the viewpoint used to render a scene.</summary>
public sealed class Camera
{
    private readonly CameraSafeHandle
        _handle;

    /// <summary>Initializes or accesses Camera.</summary>
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

        /// <summary>Gets or sets the camera position in world space.</summary>
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

        /// <summary>Gets or sets the camera orientation.</summary>
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

        /// <summary>Gets or sets the camera's vertical perspective field of view.</summary>
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

    /// <summary>Translates the camera in world space.</summary>
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

    /// <summary>Sets camera orientation from pitch, yaw, and roll.</summary>
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

        /// <summary>Sets the near and far clipping distances.</summary>
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

        /// <summary>Gets the near clipping distance.</summary>
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

        /// <summary>Gets the far clipping distance.</summary>
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

    /// <summary>Applies an incremental pitch rotation.</summary>
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

    /// <summary>Applies an incremental yaw rotation.</summary>
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

    /// <summary>Applies an incremental roll rotation.</summary>
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