using Oxide.Graphics;
using Oxide.Math;
using Oxide.Native;

namespace Oxide.Collision;

public static class Collision
{
    public static bool Collides(
        Mesh meshA,
        Transform transformA,

        Mesh meshB,
        Transform transformB
    )
    {
        ArgumentNullException.ThrowIfNull(
            meshA
        );

        ArgumentNullException.ThrowIfNull(
            meshB
        );

        NativeMethods.Check(
            NativeMethods
                .oxide_collision_collides(
                    meshA.Handle,
                    transformA,

                    meshB.Handle,
                    transformB,

                    out byte result
                )
        );

        return result != 0;
    }
}