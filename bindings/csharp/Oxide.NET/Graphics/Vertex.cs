using System.Runtime.InteropServices;

using Oxide.Math;

namespace Oxide.Graphics;

[StructLayout(LayoutKind.Sequential)]
public struct Vertex
{
    public Vec3 Position;
    public Vec3 Normal;
    public Vec2 TexCoord;

    public Vertex(
        Vec3 position,
        Vec3 normal,
        Vec2 texCoord
    )
    {
        Position = position;
        Normal = normal;
        TexCoord = texCoord;
    }
}