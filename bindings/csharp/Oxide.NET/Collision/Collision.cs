using Oxide.Graphics;
using Oxide.Math;
using Oxide.Native;

namespace Oxide.Collision;

/// <summary>Triangle-mesh intersection and containment queries.</summary>
public static class Collision
{
    /// <summary>Tests intersection of two transformed triangle meshes, including containment, using their model-space pivots.</summary>
    public static bool Collides(Mesh meshA, Transform transformA,

                                Mesh meshB, Transform transformB)
    {
        ArgumentNullException.ThrowIfNull(meshA);

        ArgumentNullException.ThrowIfNull(meshB);

        NativeMethods.Check(NativeMethods.oxide_collision_collides(meshA.Handle, transformA,

                                                                   meshB.Handle, transformB,

                                                                   out byte result));

        return result != 0;
    }
}
