using Oxide.Native;
using Oxide.Math;

namespace Oxide.Graphics;

/// <summary>Perspective camera looking along local -Z with +Y up at its default orientation.</summary>
/// <remarks>Owns a native handle released by a SafeHandle finalizer. Native failures raise OxideException.</remarks>
public sealed class Camera
{
    private readonly CameraSafeHandle _handle;

    /// <summary>Creates a camera at the origin with identity rotation, a 60-degree vertical field of view, and clipping distances 0.1 and 1000.</summary>
    public Camera()
    {
        NativeMethods.Check(NativeMethods.oxide_camera_create(out NativeCameraHandle handle));

        _handle = new CameraSafeHandle(handle);
    }

    internal NativeCameraHandle Handle
    {
        get {
            if (_handle.IsInvalid || _handle.IsClosed)
            {
                throw new ObjectDisposedException(nameof(Camera));
            }

            return _handle.NativeHandle;
        }
    }
    /// <summary>Position in world space.</summary>
    public Vec3 Position
    {
        get {
            NativeMethods.Check(NativeMethods.oxide_camera_get_position(Handle, out Vec3 result));

            return result;
        }

        set {
            NativeMethods.Check(NativeMethods.oxide_camera_set_position(Handle, value));
        }
    }
    /// <summary>Orientation quaternion; assigned values are normalized.</summary>
    public Quaternion Rotation
    {
        get {
            NativeMethods.Check(NativeMethods.oxide_camera_get_rotation(Handle,
                                                                        out Quaternion result));

            return result;
        }

        set {
            NativeMethods.Check(NativeMethods.oxide_camera_set_rotation(Handle, value));
        }
    }
    /// <summary>Vertical perspective field of view; use an angle strictly between zero and pi radians.</summary>
    public Angle FieldOfView
    {
        get {
            NativeMethods.Check(NativeMethods.oxide_camera_get_field_of_view(Handle,
                                                                             out Angle result));

            return result;
        }

        set {
            NativeMethods.Check(NativeMethods.oxide_camera_set_field_of_view(Handle, value));
        }
    }

    /// <summary>Adds a world-space translation without rotating it by the current orientation.</summary>
    public void TranslateBy(Vec3 translation)
    {
        NativeMethods.Check(NativeMethods.oxide_camera_translate_by(Handle, translation));
    }

    /// <summary>Sets orientation to yaw * pitch * roll, about Y, X, and Z respectively.</summary>
    public void SetEuler(Angle pitch, Angle yaw, Angle roll)
    {
        NativeMethods.Check(NativeMethods.oxide_camera_set_euler(Handle, pitch, yaw, roll));
    }

    /// <summary>Sets clipping distances in world units; requires 0 &lt; near &lt; far.</summary>
    public void SetClipPlanes(float nearPlane, float farPlane)
    {
        NativeMethods.Check(
            NativeMethods.oxide_camera_set_clip_planes(Handle, nearPlane, farPlane));
    }

    /// <summary>Distance from the camera to the near clipping plane in world units.</summary>
    public float NearPlane
    {
        get {
            NativeMethods.Check(NativeMethods.oxide_camera_get_near_plane(Handle,
                                                                          out float result));

            return result;
        }
    }
    /// <summary>Distance from the camera to the far clipping plane in world units.</summary>
    public float FarPlane
    {
        get {
            NativeMethods.Check(NativeMethods.oxide_camera_get_far_plane(Handle, out float result));

            return result;
        }
    }

    /// <summary>Applies an additional rotation about the world X axis.</summary>
    public void RotatePitchBy(Angle angle)
    {
        NativeMethods.Check(NativeMethods.oxide_camera_rotate_pitch_by(Handle, angle));
    }

    /// <summary>Applies an additional rotation about the world Y axis.</summary>
    public void RotateYawBy(Angle angle)
    {
        NativeMethods.Check(NativeMethods.oxide_camera_rotate_yaw_by(Handle, angle));
    }

    /// <summary>Applies an additional rotation about the world Z axis.</summary>
    public void RotateRollBy(Angle angle)
    {
        NativeMethods.Check(NativeMethods.oxide_camera_rotate_roll_by(Handle, angle));
    }
}
