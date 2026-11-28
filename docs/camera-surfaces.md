# Rect3D camera surfaces

Bind a camera to a world-space rectangle to create a live monitor. Oxide renders
its subsequent draw calls into an internal HDR texture and automatically displays
that image on the rectangle in every camera bound to an ordinary screen Rect.
No public render-pass, render-target, texture-management, or additional DrawMesh
call is needed for the monitor itself.

## Rectangle geometry

Rect3D contains four world-space image corners in this order: top-left, top-right,
bottom-right, bottom-left. Its explicit normal chooses the visible front side,
independently of corner winding. The constructor normalizes this normal.

Corners and normal must be finite, within +/-1e12, with edges longer than 0.00001
world units. Corners must form a rectangle; the normal must be nonzero and
perpendicular to both edges. Geometry checks use a relative tolerance of 0.0001.
Corners identify fixed UV positions: (0,0), (1,0), (1,1), (0,1). Changing the
normal alone changes visibility, not image coordinates. A zero/default value is
invalid. Raw C/Python fields are validated again when bound.

## C#

```csharp
Rect3D monitor = new(
    new Vec3(-1, 1, 0), new Vec3(1, 1, 0),
    new Vec3(1, -1, 0), new Vec3(-1, -1, 0),
    new Vec3(0, 0, 1)
);

// Inside Draw:
graphics.BindCamera(securityCamera, monitor);
DrawRemoteRoom(graphics); // Ordinary DrawMesh calls fill the feed.

graphics.BindCamera(playerCamera, graphics.FullRect);
DrawRoom(graphics);       // The monitor appears automatically in this view.
```

`DrawRemoteRoom` and `DrawRoom` stand for application draw routines, not engine
scene objects. BindCamera captures the camera at binding time. Each DrawMesh
captures its existing material, geometry, transform, environment, lights, and
transparency state. Submit the geometry needed by each camera normally; binding
a surface does not implicitly reuse another camera's scene submissions.

For an explicit image size, use `BindCamera(camera, monitor, 1024, 768)`.
The normal screen-rectangle overload is unchanged.

## Rust

```rust,ignore
use oxide::math::{Rect3D, Vec3};
let monitor = Rect3D::new(
    Vec3::new(-1.0,1.0,0.0), Vec3::new(1.0,1.0,0.0),
    Vec3::new(1.0,-1.0,0.0), Vec3::new(-1.0,-1.0,0.0),
    Vec3::new(0.0,0.0,1.0),
)?;
graphics.bind_camera_3d(&security_camera, monitor)?;
// Submit feed meshes.
graphics.bind_camera(&player_camera, graphics.full_rect())?;
// Submit the screen camera's meshes.
```

`bind_camera_3d_with_resolution(camera, rectangle, width, height)` selects an
explicit size. The C ABI exposes the same distinction through
`oxide_graphics_bind_camera_3d` and its `_with_resolution` variant. Construct C
rectangles with `oxide_rect3d_create`; values use the same 60-byte layout across
Rust, C, C#, and Python.

## Python

```python
from oxide import Rect3D, Vec3
monitor = Rect3D(
    Vec3(-1,1,0), Vec3(1,1,0), Vec3(1,-1,0), Vec3(-1,-1,0), Vec3(0,0,1)
)
graphics.bind_camera_3d(security_camera, monitor)
# Submit feed meshes.
graphics.bind_camera(player_camera, graphics.full_rect)
# Submit screen meshes.
```

Optional `width=1024, height=768` keywords select the image dimensions.

## Rendering and lifetime contract

- Bind every visible monitor during each frame. Registrations are frame-local;
  an unbound rectangle does not keep appearing with an old image.
- At most 16 distinct rectangles may be bound per frame. Exact validated
  corner/normal equality identifies a rectangle within that frame. Rebinding the
  same rectangle shares its target; the last camera group replaces its image.
  Using different resolutions for one rectangle within a frame returns an error.
- Feeds are rendered before all screen views, regardless of binding order. A
  screen camera bound first therefore sees the current frame's feed.
- **Feed cameras omit all automatic monitor surfaces.** Monitors are displayed
  only in screen-bound cameras. This first implementation has no recursive
  monitors, mirrors, portals, or previous-frame feedback.
- Monitor surfaces are opaque, unlit, and visible only from the normal's side.
  They test/write scene depth after opaque meshes and before transparent meshes,
  so opaque objects can hide them and transparent foreground objects blend over
  them. Feed contents support existing lighting, shadows, and transparency.
- Automatic monitor geometry is not a shadow caster. Draw ordinary mesh geometry
  for a bezel, backing, or physical shadow caster when needed.
- Each camera group has independent depth in its viewport. Overlapping screen
  camera rectangles retain ordinary color submission order without stale depth
  from a previous camera group.
- Images remain linear RGBA16F HDR. Sampling a monitor adds no extra exposure,
  tone mapping, or sRGB conversion; the final display pass performs those once.
- Default size is 512 pixels on the longest edge, preserving the rectangle's
  aspect to integer-pixel precision. Explicit dimensions are 1..2048 each, or
  both zero for automatic size. Projection/background aspect always follows the
  world rectangle; explicit pixel dimensions control sampling density.
- Each target has independent color and depth storage, consuming 12 bytes per
  pixel before allocation overhead, plus one ordinary descriptor. Targets are
  reused by frame slot, resized when necessary, and retired after a completed
  frame no longer uses them. No CPU Texture asset or readback copy is created.

## Demos and validation

Rebuild shaders and the native library before using updated bindings:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\build-shaders.ps1
cargo build --locked --lib --examples
cargo run --locked --example monitors
dotnet run --project examples/csharp/Monitors/Monitors.csproj
python .\examples\python\monitors.py
```

Run demos individually from the repository root. Both screens show the same
remote room from different cameras, with one camera moving. Space switches
whether screen or feed commands are submitted first, T cycles the foreground
box's transparency, B flips monitor normals, and Escape exits. An existing
editable Python install needs no reinstall.

All tests remain under tests/. WARP readbacks cover both submission orders,
independent images/viewports, image orientation, front-face selection, opaque
occlusion, transparent blending, HDR preservation, resizing/reuse, and retirement.
Binding tests validate layout, constructors, overloads, and Python demo controls.
