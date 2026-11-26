using System.Runtime.InteropServices;
using Oxide.Graphics;
using Oxide.Native;

namespace Oxide.Math;

[StructLayout(LayoutKind.Sequential)]
/// <summary>World-space position, quaternion orientation, and component-wise local scale. A default constructed transform has unit scale.</summary>
public struct Transform : IEquatable<Transform>
{
    private Vec3 _position;
    private Quaternion _rotation;
    private Vec3 _scale;

    /// <summary>Creates a transform at the origin with identity rotation and unit scale.</summary>
    public Transform()
    {
        _position = new Vec3(0.0f, 0.0f, 0.0f);

        _rotation = Quaternion.Identity;

        _scale = new Vec3(1.0f, 1.0f, 1.0f);
    }

    /// <summary>Position in world space.</summary>
    public Vec3 Position
    {
        readonly get => _position;

        set => _position = value;
    }
    /// <summary>Orientation quaternion; assigned values are normalized.</summary>
    public Quaternion Rotation
    {
        readonly get => _rotation;

        set => _rotation = value.Normalized();
    }
    /// <summary>Component-wise scale along the model's local axes.</summary>
    public Vec3 Scale
    {
        readonly get => _scale;

        set => _scale = value;
    }

    /// <summary>Adds a world-space translation without rotating it by the current orientation.</summary>
    public void TranslateBy(Vec3 translation)
    {
        _position += translation;
    }

    /// <summary>Sets orientation to yaw * pitch * roll, about Y, X, and Z respectively.</summary>
    public void SetEuler(Angle pitch, Angle yaw, Angle roll)
    {
        _rotation = Quaternion.FromEuler(pitch, yaw, roll);
    }

    /// <summary>Applies an additional rotation about the world X axis.</summary>
    public void RotatePitchBy(Angle angle)
    {
        Quaternion rotation = Quaternion.FromAxisAngle(new Vec3(1.0f, 0.0f, 0.0f), angle);

        _rotation = (rotation * _rotation).Normalized();
    }

    /// <summary>Applies an additional rotation about the world Y axis.</summary>
    public void RotateYawBy(Angle angle)
    {
        Quaternion rotation = Quaternion.FromAxisAngle(new Vec3(0.0f, 1.0f, 0.0f), angle);

        _rotation = (rotation * _rotation).Normalized();
    }

    /// <summary>Applies an additional rotation about the world Z axis.</summary>
    public void RotateRollBy(Angle angle)
    {
        Quaternion rotation = Quaternion.FromAxisAngle(new Vec3(0.0f, 0.0f, 1.0f), angle);

        _rotation = (rotation * _rotation).Normalized();
    }

    /// <summary>Multiplies the current scale component by component.</summary>
    public void ScaleBy(Vec3 scale)
    {
        _scale = new Vec3(_scale.X * scale.X, _scale.Y * scale.Y, _scale.Z * scale.Z);
    }

    /// <summary>Returns a child transform aligning model-space attachment points using a world-space direction, twist angle, and child scale.</summary>
    public static Transform FromAttachment(Mesh parent, Transform parentTransform,
                                           Vec3 parentAttachPoint,

                                           Mesh child, Vec3 childAttachPoint,

                                           Vec3 direction, Angle rotation, Vec3 scale)
    {
        ArgumentNullException.ThrowIfNull(parent);

        ArgumentNullException.ThrowIfNull(child);

        NativeMethods.Check(NativeMethods.oxide_transform_from_attachment(
            parent.Handle, parentTransform, parentAttachPoint,

            child.Handle, childAttachPoint,

            direction, rotation, scale,

            out Transform result));

        return result;
    }

    /// <summary>Tests exact component equality; no floating-point tolerance is applied.</summary>
    public readonly bool Equals(Transform other)
    {
        return _position == other._position && _rotation == other._rotation &&
               _scale == other._scale;
    }

    /// <summary>Tests exact component equality; no floating-point tolerance is applied.</summary>
    public override readonly bool Equals(object? obj)
    {
        return obj is Transform other && Equals(other);
    }

    /// <summary>Returns a hash consistent with exact component equality.</summary>
    public override readonly int GetHashCode()
    {
        return HashCode.Combine(_position, _rotation, _scale);
    }

    /// <summary>Tests exact component equality.</summary>
    public static bool operator ==(Transform left, Transform right) => left.Equals(right);

    /// <summary>Tests exact component inequality.</summary>
    public static bool operator !=(Transform left, Transform right) => !left.Equals(right);
}
