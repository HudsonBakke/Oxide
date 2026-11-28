# GPU resource lifetime and capacity

A draw captures the geometry version and rendering state it needs. Dropping a
mesh or material after submitting its draw is safe: GPU buffers, texture data,
and descriptor ranges stay valid until the frame completes. Oxide currently
waits for the GPU at frame end, and only reclaims resources at these idle
boundaries. Reclamation must remain fence-protected if frames later overlap.

GPU mesh entries are retired when their CPU mesh is dropped or a newer drawn
geometry version replaces them. Immutable texture clones share an identity and
ownership token; their GPU entry survives while any clone remains alive.
Materials and the configured environment also own texture clones. Derived
irradiance/specular maps, normal views, material descriptor pairs, and environment
descriptor triplets are reclaimed with their source textures. Temporary texture
upload buffers are released after the GPU finishes their copy commands.

Shadow maps are reused between frames while needed. Slots beyond the completed
frame's shadow requests are released, and transmission maps are released after a
frame with no partial casters. Reserved views are reset before retired shadow
storage can be referenced again. Unused constant-buffer pages are also retired
at a later GPU-idle reset.

## Capacity and failure behavior

| Resource | Behavior |
| --- | --- |
| Constant uploads | Grow in 4 MiB pages; existing GPU addresses stay stable throughout the frame. Each individual constant record must be 1 byte through 64 KiB and is aligned to 256 bytes. Allocation failures return errors. |
| Ordinary descriptors | 2,463 reusable slots in the current 4,096-slot heap; 1,633 slots are reserved for HDR/shadows. Material pairs require 2 contiguous slots, environment tables 3. Fragmentation or exhaustion returns an error describing the requested/free/largest ranges. |
| Scene lights | At most 8 point, 8 spot, and 8 directional lights per draw. Extra lights produce a draw error, rather than disappearing silently. Clear or reduce active lights before retrying. |
| Shadow snapshots | At most 32 distinct shadow projections/settings per frame. Exceeding this returns a draw error; changing only intensity or color reuses a projection. |

Descriptor slots are returned to a coalescing allocator, so repeatedly loading
and dropping scenes does not permanently consume the heap. Assets dropped during
a frame still occupy slots until that frame completes. A failed resource
allocation does not enqueue the mesh; earlier queued draws remain valid. A
partially prepared draw may have recorded texture uploads, which stay alive
through frame completion and are then eligible for reclamation. To reclaim such
slots, release unused CPU assets and finish the current frame before retrying.

The C ABI returns the renderer error through its normal status/error channel;
C# and Python surface their existing exceptions. The rendering API does not need
an explicit resource-cache cleanup call.

Rect3D camera targets have independent HDR/depth storage and reusable descriptor
slots. Unused targets retire after GPU completion; frame registrations are cleared
each frame. The default image's longest edge is 512 pixels, explicit sizes are
limited to 2048 per dimension, and a frame supports 16 distinct surfaces. See
[camera surfaces](camera-surfaces.md) for scheduling and lifetime rules.
