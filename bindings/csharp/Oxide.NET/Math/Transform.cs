using System.Runtime.InteropServices;
using Oxide.Graphics;
using Oxide.Native;

namespace Oxide.Math;

[StructLayout(LayoutKind.Sequential)]
/// <summary>Position, rotation, and scale describing an object's placement.</summary>
public struct Transform
    : IEquatable<Transform>
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

    public static Transform FromAttachment(
        Mesh parent,
        Transform parentTransform,
        Vec3 parentAttachPoint,

        Mesh child,
        Vec3 childAttachPoint,

        Vec3 direction,
        Angle rotation,
        Vec3 scale
    )
    {
        ArgumentNullException.ThrowIfNull(
            parent
        );

        ArgumentNullException.ThrowIfNull(
            child
        );

        NativeMethods.Check(
            NativeMethods
                .oxide_transform_from_attachment(
                    parent.Handle,
                    parentTransform,
                    parentAttachPoint,

                    child.Handle,
                    childAttachPoint,

                    direction,
                    rotation,
                    scale,

                    out Transform result
                )
        );

        return result;
    }

    public readonly bool Equals(
        Transform other
    )
    {
        return _position
                == other._position
            && _rotation
                == other._rotation
            && _scale
                == other._scale;
    }

    public override readonly bool Equals(
        object? obj
    )
    {
        return obj
            is Transform other
            && Equals(other);
    }

    public override readonly int GetHashCode()
    {
        return HashCode.Combine(
            _position,
            _rotation,
            _scale
        );
    }

    public static bool operator ==(
        Transform left,
        Transform right
    ) =>
        left.Equals(right);

    public static bool operator !=(
        Transform left,
        Transform right
    ) =>
        !left.Equals(right);
}