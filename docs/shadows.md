# Light-owned shadows and transparency

Point, spot, and directional lights share color, intensity, and shadow state.
Enable shadows on a light and submit it normally. Constructors supply usable
settings automatically; constructing or tuning `ShadowSettings` is optional.
Shadows default to disabled.

```csharp
ILightSource light = new PointLight(new Vec3(2, 7, 4), Color.White, 100);
light.CastsShadows = true;
graphics.AddLight(light);
graphics.DrawMesh(mesh, material, transform);
```

The same code accepts `DirectionalLight` or `SpotLight`. C# retains typed
`AddLight` overloads as well as `AddLight(ILightSource)`; the interface supports
the three built-in light types. They remain value structs, so assigning to an
interface boxes a copy. Mutate the interface instance you intend to submit.

```rust
fn enable_shadows<L: oxide::graphics::LightSource>(light: &mut L) {
    light.set_casts_shadows(true);
}
// Use any concrete light with the same generic API.
enable_shadows(&mut sun);
graphics.add_light(&sun);
graphics.draw_mesh(&mesh, &material, &transform)?;
```

```python
light = PointLight(Vec3(2, 7, 4), Color.WHITE, 100, casts_shadows=True)
graphics.add_light(light)
graphics.draw_mesh(mesh, material, transform, transparency=0.5)
```

Python's three concrete types implement `LightSource` with common `color`,
`intensity`, `casts_shadows`, and `shadow_settings` attributes. Rust uses the
sealed `LightSource` trait. The original graphics-level
`SetDirectionalShadows` / `ClearDirectionalShadows` functions have been removed.
`AddLight` captures the light value, including shadow state; later changes to
the source object affect the next submission, not earlier draws.

C uses `oxide_point_light_init`, `oxide_spot_light_init`, or
`oxide_directional_light_init` to initialize a light with default settings.
Set its `casts_shadows` field to 1 and call the existing typed add-light function.
The native light structs have grown: rebuild native clients against the new
header and DLL. Their shadow settings are initialized by the light initializer;
users need not initialize settings separately.

## Optional settings

```csharp
light.ShadowSettings = new ShadowSettings { Resolution = 512 };
```

```rust
sun.set_shadow_settings(ShadowSettings { resolution: 512, ..Default::default() })?;
```

```python
light.shadow_settings = ShadowSettings(resolution=512)
```

C uses `oxide_shadow_settings_default` before overriding fields. The previous
`DirectionalShadowSettings` name remains a compatibility alias/converter.

| Rust/Python/C field | Default | Meaning |
| --- | --- | --- |
| `center` | Origin | Directional volume center in world space |
| `size` | 30 | Directional full square width/height perpendicular to rays |
| `depth` | 60 | Directional full extent along the rays, centered on `center` |
| `resolution` | 1024 | Square opaque-map resolution; power of two from 128 to 4096 |
| `depth_bias` | 0.0001 | Non-negative normalized depth comparison offset |
| `normal_bias` | 0.01 | Non-negative receiver offset along the geometric normal, in world units |
| `near_plane` | 0.1 | Positive point/spot near clipping distance, in world units |
| `far_plane` | 60 | Point/spot far clipping distance, greater than near_plane |

All floating values must be finite; size/depth must be positive. C# fields use
PascalCase. Center/size/depth control directional coverage. Point and spot
coverage follows the light's position/direction and uses near/far clipping.
Locations outside shadow coverage remain lit. Excessive bias detaches shadows;
insufficient bias can cause self-shadow speckling.

## Rendering and limits

Directional lights use orthographic maps. Spot lights use their actual cone
angle and perspective maps. Point lights use six perspective faces to cover all
directions. Spots with half-angle at least approximately 90 degrees also use six
faces so wide cones retain their lighting semantics.

Several lights can cast shadows simultaneously. The renderer supports eight
lights of each type per draw and 32 distinct shadow projections/settings per
frame; exceeding the latter returns a draw error. Changing only intensity/color
reuses a projection. Lights allocate maps lazily and reuse GPU storage. Disabled
lights create no shadow requests. Shadow state is never inferred by comparing
a scene light's color/intensity with a separately configured graphics light.

Each opaque map uses four bytes per texel per face. Partial transmission adds
eight depth/value layers per face, with six bytes per texel per layer. Their
resolution is capped at 1024 for directional lights, 512 for narrow spots, and
256 for six-face point/wide-spot maps. Maps are only allocated when used. Point
shadows require six views; enabling many shadowed lights increases GPU work and
memory. Keep resolutions appropriate for the scene.

Every mesh draw carries the same scalar `transparency` for rendering and shadows:

| Value | Result |
| --- | --- |
| 0 (default) | Opaque, blocks direct light |
| 0.5 | Half-opacity rendering, transmits half the direct light |
| 1 | Invisible; no color, depth, or shadow contribution |

Two separate half-transmitting panes pass a quarter of the light. Opaque
blockers still stop it completely, and blockers behind a receiver do not shadow
it. Shadow maps retain the nearest eight distinct transparent surface depths
per ray; further layers are ignored. Coincident surfaces resolve as one depth.
Transmission applies per front-facing intersection, so concave meshes can cross
a ray more than once. Duplicate draws of the same geometry/transform/transparency
across camera views contribute one caster. Shadow transmission remains neutral,
without color tint, refraction, or volumetric absorption.

Opaque meshes establish depth first. Partially transparent meshes blend back to
front per camera in linear HDR, testing opaque depth without writing it.
Sorting uses the transformed bounds center; intersecting or self-overlapping
transparent meshes can need splitting. Texture/base-color alpha do not provide
extra alpha masks. Each draw captures its geometry version, material, transform,
camera, lights, and environment. Off-camera submitted meshes can cast shadows;
meshes never submitted cannot. Argument errors are reported at submission;
GPU execution errors can surface at frame end.

## Examples

All three shadows examples demonstrate the shared light API without explicitly
constructing shadow settings. **L** cycles directional, point, and spot lights;
**Space** toggles the active light's shadows; **T** cycles the moving box's
transparency. The opaque yellow marker behind the box demonstrates see-through
rendering. The cyan panels demonstrate overlapping transmission.

Rebuild once from the repository root, then run examples individually:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\build-shaders.ps1
cargo build --locked --lib --examples
cargo run --locked --example shadows
dotnet run --project examples/csharp/Shadows/Shadows.csproj
python .\examples\python\shadows.py
```

An existing editable Python install needs no reinstall after rebuilding the DLL.
