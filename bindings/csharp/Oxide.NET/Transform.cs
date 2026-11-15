using System.Runtime.InteropServices;

namespace Oxide;

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
            _rotation = value;
    }

    public Vec3 Scale
    {
        readonly get =>
            _scale;

        set =>
            _scale = value;
    }
}