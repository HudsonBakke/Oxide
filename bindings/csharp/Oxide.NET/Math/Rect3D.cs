using System.Runtime.InteropServices;
using Oxide.Native;
namespace Oxide.Math;

/// <summary>World-space camera display rectangle with four image corners and an explicit front normal.</summary>
[StructLayout(LayoutKind.Sequential)]
public readonly struct Rect3D : IEquatable<Rect3D>
{
    /// <summary>Image top-left corner in world coordinates.</summary>
    public readonly Vec3 TopLeft;
    /// <summary>Image top-right corner in world coordinates.</summary>
    public readonly Vec3 TopRight;
    /// <summary>Image bottom-right corner in world coordinates.</summary>
    public readonly Vec3 BottomRight;
    /// <summary>Image bottom-left corner in world coordinates.</summary>
    public readonly Vec3 BottomLeft;
    /// <summary>Unit normal selecting the visible front side.</summary>
    public readonly Vec3 Normal;
    /// <summary>Validates rectangular finite corners and normalizes a nonzero perpendicular normal.</summary>
    /// <remarks>Edges must exceed 0.00001 world units; rectangle/perpendicularity tolerance is 0.0001 relative. The zero-initialized default is invalid and cannot be bound.</remarks>
    public Rect3D(Vec3 topLeft, Vec3 topRight, Vec3 bottomRight, Vec3 bottomLeft, Vec3 normal)
    {
        NativeMethods.Check(NativeMethods.oxide_rect3d_create(topLeft, topRight, bottomRight, bottomLeft, normal, out Rect3D value));
        this = value;
    }
    /// <summary>Horizontal extent in world units.</summary>
    public float Width => (TopRight-TopLeft).Length;
    /// <summary>Vertical extent in world units.</summary>
    public float Height => (BottomLeft-TopLeft).Length;
    /// <summary>World-space width divided by height.</summary>
    public float AspectRatio => Width/Height;
    /// <summary>Tests exact corner and normal equality.</summary>
    public bool Equals(Rect3D other) => TopLeft==other.TopLeft && TopRight==other.TopRight && BottomRight==other.BottomRight && BottomLeft==other.BottomLeft && Normal==other.Normal;
    /// <summary>Tests exact rectangle equality.</summary>
    public override bool Equals(object? other) => other is Rect3D rectangle && Equals(rectangle);
    /// <summary>Hash consistent with exact rectangle equality.</summary>
    public override int GetHashCode() => HashCode.Combine(TopLeft,TopRight,BottomRight,BottomLeft,Normal);
    /// <summary>Tests exact rectangle equality.</summary>
    public static bool operator ==(Rect3D left,Rect3D right) => left.Equals(right);
    /// <summary>Tests exact rectangle inequality.</summary>
    public static bool operator !=(Rect3D left,Rect3D right) => !left.Equals(right);
}
