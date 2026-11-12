using System;

using Oxide.Native;

namespace Oxide;

public sealed class Camera
    : IDisposable
{
    private NativeCameraHandle
        _handle;

    private bool
        _disposed;

    public Camera()
    {
        NativeMethods.Check(
            NativeMethods
                .oxide_camera_create(
                    out _handle
                )
        );
    }

    internal NativeCameraHandle Handle
    {
        get
        {
            EnsureNotDisposed();

            return _handle;
        }
    }

    public Vec3 Position
    {
        get
        {
            EnsureNotDisposed();

            NativeMethods.Check(
                NativeMethods
                    .oxide_camera_get_position(
                        _handle,
                        out Vec3 result
                    )
            );

            return result;
        }

        set
        {
            EnsureNotDisposed();

            NativeMethods.Check(
                NativeMethods
                    .oxide_camera_set_position(
                        _handle,
                        value
                    )
            );
        }
    }

    public Quaternion Rotation
    {
        get
        {
            EnsureNotDisposed();

            NativeMethods.Check(
                NativeMethods
                    .oxide_camera_get_rotation(
                        _handle,
                        out Quaternion result
                    )
            );

            return result;
        }

        set
        {
            EnsureNotDisposed();

            NativeMethods.Check(
                NativeMethods
                    .oxide_camera_set_rotation(
                        _handle,
                        value
                    )
            );
        }
    }

    public Angle FieldOfView
    {
        get
        {
            EnsureNotDisposed();

            NativeMethods.Check(
                NativeMethods
                    .oxide_camera_get_field_of_view(
                        _handle,
                        out Angle result
                    )
            );

            return result;
        }

        set
        {
            EnsureNotDisposed();

            NativeMethods.Check(
                NativeMethods
                    .oxide_camera_set_field_of_view(
                        _handle,
                        value
                    )
            );
        }
    }

    public void TranslateBy(
        Vec3 translation
    )
    {
        EnsureNotDisposed();

        NativeMethods.Check(
            NativeMethods
                .oxide_camera_translate_by(
                    _handle,
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
        EnsureNotDisposed();

        NativeMethods.Check(
            NativeMethods
                .oxide_camera_set_euler(
                    _handle,
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
        EnsureNotDisposed();

        NativeMethods.Check(
            NativeMethods
                .oxide_camera_set_clip_planes(
                    _handle,
                    nearPlane,
                    farPlane
                )
        );
    }

    public void Dispose()
    {
        if (_disposed)
        {
            return;
        }

        NativeMethods.Check(
            NativeMethods
                .oxide_camera_destroy(
                    _handle
                )
        );

        _handle = default;
        _disposed = true;

        GC.SuppressFinalize(
            this
        );
    }

    private void EnsureNotDisposed()
    {
        ObjectDisposedException
            .ThrowIf(
                _disposed,
                this
            );
    }
}