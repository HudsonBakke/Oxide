using System.Runtime.InteropServices;

namespace Oxide.Math;

[StructLayout(LayoutKind.Sequential)]
public struct Transform
{
    private Vec3 _position;
    private Quaternion _rotation;
    private Vec3 _scale;

    public Transform()
    {
        _position =
            new Vec3(
                0.0f,
                0.0f,
                0.0f
            );

        _rotation =
            Quaternion.Identity;

        _scale =
            new Vec3(
                1.0f,
                1.0f,
                1.0f
            );
    }

    public Vec3 Position
    {
        readonly get =>
            _position;

        set =>
            _position = value;
    }

    public Quaternion Rotation
    {
        readonly get =>
            _rotation;

        set =>
            _rotation =
                value.Normalized();
    }

    public Vec3 Scale
    {
        readonly get =>
            _scale;

        set =>
            _scale = value;
    }

    public void TranslateBy(
        Vec3 translation
    )
    {
        _position +=
            translation;
    }

    public void SetEuler(
        Angle pitch,
        Angle yaw,
        Angle roll
    )
    {
        _rotation =
            Quaternion.FromEuler(
                pitch,
                yaw,
                roll
            );
    }

    public void RotatePitchBy(
        Angle angle
    )
    {
        Quaternion rotation =
            Quaternion.FromAxisAngle(
                new Vec3(
                    1.0f,
                    0.0f,
                    0.0f
                ),
                angle
            );

        _rotation =
            (rotation * _rotation)
                .Normalized();
    }

    public void RotateYawBy(
        Angle angle
    )
    {
        Quaternion rotation =
            Quaternion.FromAxisAngle(
                new Vec3(
                    0.0f,
                    1.0f,
                    0.0f
                ),
                angle
            );

        _rotation =
            (rotation * _rotation)
                .Normalized();
    }

    public void RotateRollBy(
        Angle angle
    )
    {
        Quaternion rotation =
            Quaternion.FromAxisAngle(
                new Vec3(
                    0.0f,
                    0.0f,
                    1.0f
                ),
                angle
            );

        _rotation =
            (rotation * _rotation)
                .Normalized();
    }

    public void ScaleBy(
        Vec3 scale
    )
    {
        _scale =
            new Vec3(
                _scale.X * scale.X,
                _scale.Y * scale.Y,
                _scale.Z * scale.Z
            );
    }
}