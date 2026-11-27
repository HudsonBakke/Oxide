# Automatic directional shadows

Every mesh draw carries its shadow behavior. Submit each object once, with the
same camera/material/transform API used for ordinary rendering. Oxide collects
the draws, builds the shadow maps internally, and renders camera views at frame
end. The game continues to own its meshes, materials, lights, and cameras.

```rust
graphics.set_directional_shadows(&sun, &settings)?;
graphics.bind_camera(&camera, graphics.full_rect())?;
graphics.add_light(&sun);
graphics.draw_mesh(&ground, &ground_material, &ground_transform)?;
graphics.draw_mesh_with_transparency(&window, &glass, &window_transform, 0.5)?;
```

```csharp
graphics.SetDirectionalShadows(sun, settings);
graphics.BindCamera(camera, graphics.FullRect);
graphics.AddLight(sun);
graphics.DrawMesh(ground, groundMaterial, groundTransform);
graphics.DrawMesh(window, glass, windowTransform, transparency: 0.5f);
```

```python
graphics.set_directional_shadows(sun, settings)
graphics.bind_camera(camera, graphics.full_rect)
graphics.add_light(sun)
graphics.draw_mesh(ground, ground_material, ground_transform)
graphics.draw_mesh(window, glass, window_transform, transparency=0.5)
```

The original three-argument draw remains available and defaults to opaque
shadows. Rust adds `draw_mesh_with_transparency`; C adds
`oxide_graphics_draw_mesh_with_transparency` while retaining the original
function. Shadow configuration uses `oxide_graphics_set_directional_shadows`
and `oxide_graphics_clear_directional_shadows` in C. The former explicit
begin/draw/end shadow-pass APIs have been removed.

## Transmission semantics

`transparency` must be finite and in [0, 1]:

| Value | Shadow behavior |
| --- | --- |
| 0 | Opaque; blocks the selected light |
| 0.5 | Passes half that light at a front-facing surface |
| 1 | No shadow contribution |

Overlapping transparent surfaces multiply transmission: two separate 0.5 panes
pass 0.25 of the direct light. An opaque blocker still blocks it completely,
regardless of whether it is before or behind transparent surfaces. Surfaces
behind the receiver do not attenuate its lighting. The parameter changes shadow
transmission only; it does not make the visible mesh see-through. Material alpha
is not sampled by the shadow passes. Transmission is neutral, without color tint.

There is one configured directional shadow light. Use the same unchanged light
value in `add_light`: direction, color, and intensity identify it. Configuration
persists across frames. Set or clear it before any mesh draws in a frame; camera
binding may precede configuration. Clearing disables shadows until configured
again. Other lights, ambient, IBL, emission, and backgrounds are unaffected.

The renderer stores opaque depth separately and peels the nearest **eight
distinct transparent surface depths** per light ray. Further transparent layers
are ignored. Coincident surfaces at the same depth are resolved as one surface;
keep physically separate panes at distinct depths. Transmission applies per
front-facing intersection, rather than once per mesh: a concave mesh can cross a
ray more than once. Ordinary back-face culling applies in both shadow and scene
rendering. Duplicate submissions of the same geometry, transform, and
transparency across camera views contribute only one caster.

Only partial transmission incurs the layered passes. Its maps are allocated
lazily at `min(resolution, 1024)` and reused. At 1024 they consume about 48 MiB;
the opaque map uses another 4 MiB at that resolution. A 3 by 3 sampling kernel
softens edges. This is a bounded real-time approximation, without cascades,
volumetric absorption, refraction, or colored transmission.

## Draw state and coverage

Each draw captures its camera/viewport, material values and textures, lights,
environment, mesh geometry version, transform, and transparency when submitted.
Later mutations or resource disposal do not alter an earlier draw. Draw order
and background-only viewports are preserved. GPU rendering failures can surface
at frame end; argument and resource-preparation errors are reported at submission.

All submitted meshes can cast shadows, including off-camera objects. A mesh the
game never submits cannot cast. All cameras share the frame's shadow maps.

`DirectionalShadowSettings::default()` creates the settings below. C# uses
`new DirectionalShadowSettings()`, Python uses `DirectionalShadowSettings()`,
and C uses `oxide_directional_shadow_settings_default(&settings)`.

| Field (Rust/Python/C) | Default | Meaning |
| --- | --- | --- |
| `center` | Origin | Finite world-space volume center |
| `size` | 30 | Full square width/height perpendicular to light rays; finite and positive |
| `depth` | 60 | Full extent along the rays, centered on `center`; finite and positive |
| `resolution` | 1024 | Opaque map square pixel size; powers of two from 128 through 4096 |
| `depth_bias` | 0.0001 | Finite non-negative comparison offset in normalized depth units |
| `normal_bias` | 0.01 | Finite non-negative geometric normal offset in world units |

C# fields use PascalCase. The volume must include the casters and receivers;
positions outside it are unshadowed. Fit coverage tightly for detail while
including elevated and off-camera casters. Excessive bias detaches shadows from
objects; insufficient bias can cause self-shadow speckling.

## Examples

The matching Rust, C#, and Python scenes contain a moving orange box (initially
0.5 transmission), an opaque blue pillar, a green box with no shadow, and two
cyan half-transmitting panels aligned to show overlapping transmission.
**Space** toggles shadows; **T** cycles the orange box through 0, 0.5, and 1.
The title displays the current state. No external assets are required.

From the repository root in PowerShell, rebuild once, then run examples individually:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\build-shaders.ps1
cargo build --locked --lib --examples
cargo run --locked --example shadows
dotnet run --project examples/csharp/Shadows/Shadows.csproj
python .\examples\python\shadows.py
```

The existing editable Python install picks up the changed API after rebuilding
the native DLL. Reinstalling the bindings is unnecessary.
