# Directional shadows

Oxide supports one directional shadow map per frame. The game explicitly
submits opaque casters to a depth pass, then draws its scene normally. Meshes,
materials, transforms, cameras, and lights remain owned by the game.

```rust
// In draw, before binding any camera:
graphics.begin_shadow_pass(&sun, &shadow_settings)?;
for (mesh, transform) in &casters {
    graphics.draw_shadow_mesh(mesh, transform)?;
}
graphics.end_shadow_pass()?;

graphics.bind_camera(&camera, graphics.full_rect())?;
graphics.add_light(&sun);
// Ordinary draw_mesh calls now receive shadows from sun.
```

Use the same unchanged directional light value for the shadow pass and scene
lighting. Beginning the pass does not add a light. The renderer matches the
light's direction, color, and intensity. Other directional lights, point and
spot lights, ambient light, IBL, background, and emission are unaffected.

Submit the same transforms and meshes to both passes so moving geometry and
mesh pivots agree. Draw every object that can cast into the covered region,
including off-camera objects. A receiver need not be submitted as a caster.
All scene cameras share the map for that frame. Skip the pass to disable
shadows; a previous frame's map is never applied automatically.

Calls made out of order return errors: only one shadow pass may be opened in a
frame, it must precede the first camera binding, only shadow meshes can be drawn
inside it, and it must be ended before binding a camera or finishing the frame.

## Coverage and quality

`DirectionalShadowSettings::default()` creates these settings. C# uses
`new DirectionalShadowSettings()`, Python uses `DirectionalShadowSettings()`,
and C uses `oxide_directional_shadow_settings_default(&settings)`.

| Field (Rust/Python/C) | Default | Meaning |
| --- | --- | --- |
| `center` | Origin | Finite world-space center of the volume |
| `size` | 30 | Full square width and height, perpendicular to the light rays; finite and positive |
| `depth` | 60 | Full extent along the light rays, centered on `center`; finite and positive |
| `resolution` | 1024 | Square pixel size; powers of two from 128 through 4096 |
| `depth_bias` | 0.0001 | Finite non-negative normalized depth offset subtracted at comparison |
| `normal_bias` | 0.01 | Finite non-negative receiver offset along the geometric normal in world units |

C# fields use PascalCase. The coverage volume is orthographic and aligned to
the light, with depth increasing in the direction its rays travel. Both casters
and receivers must fit within it; fragments outside it are unshadowed. Fit it
tightly to the playable region for better detail, while retaining enough depth
to include elevated and off-camera casters. The map is allocated on first use
and reused until the resolution changes.

A 3 by 3 kernel of bilinear depth comparisons softens edges. Increase depth or
normal bias to reduce self-shadow speckling; excessive bias separates shadows
from objects. Smaller coverage or higher resolution improves detail. This is a
single fixed volume, without cascades, contact shadows, or point/spot shadows.
Materials are not sampled in the depth pass, so transparent cutouts cast solid
shadows. Geometry uses the same winding and back-face culling as scene draws.

## Bindings and examples

C# uses `BeginShadowPass(light, settings)`, `DrawShadowMesh(mesh, transform)`,
and `EndShadowPass()`. Python uses `begin_shadow_pass`, `draw_shadow_mesh`, and
`end_shadow_pass`. C uses the corresponding `oxide_graphics_*` functions.

The three shadows examples render the same scene with a moving, rotating box,
a stationary pillar, and a ground slab. **Space** toggles the shadow pass; the
window title displays its state. No asset downloads or content builds are needed.

From the repository root in PowerShell:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\build-shaders.ps1
cargo build --locked --lib --examples
cargo run --locked --example shadows
dotnet run --project examples/csharp/Shadows/Shadows.csproj
python .\examples\python\shadows.py
```

Run the examples individually and close each window before launching another.
The existing editable Python install picks up the new API after the native DLL
is rebuilt; reinstalling the bindings is unnecessary.
