# 2D graphics and menu labels

Sprites and text are screen-space overlays. They use **physical window pixels**,
with `(0,0)` at the top-left, X to the right, and Y down. They need no camera and
work in a game that draws no 3D geometry. Every overlay appears after the entire
3D scene, including camera surfaces and transparent meshes. Among overlays,
submission order determines layering: later calls appear on top. Camera bindings
and mesh submission order do not change this behavior.

The interface draws images and labels; games supply their own menu layout,
button state and hit testing. See the `menus` examples for a complete menu with
mouse and keyboard input. No widget system or extra font asset is required.

## Simple drawing

Rust:

```rust,ignore
graphics.draw_2d(&texture, Rect::new(20.0, 20.0, 240.0, 80.0))?;
graphics.draw_text("Start game", Vec2::new(40.0, 42.0), Color::WHITE)?;
```

C#:

```csharp
graphics.Draw2D(texture, new Rect(20, 20, 240, 80));
graphics.DrawText("Start game", new Vec2(40, 42), Color.White);
```

Python:

```python
graphics.draw_2d(texture, Rect(20, 20, 240, 80))
graphics.draw_text("Start game", Vec2(40, 42), Color.WHITE)
```

A solid white texture, tinted per draw, is sufficient for menu panels and buttons.
Texture alpha and tint alpha multiply; zero means invisible, one means opaque.
Overlays use straight alpha. Imported image RGB is decoded from sRGB, tint is
linear, and blending happens in linear space. The output view encodes sRGB for
display. UI is independent of 3D exposure and Reinhard tone mapping, including
when exposure is zero. HDR image values used as UI are clamped to `[0,1]`.

## Sprite effects

The default call stretches the entire texture into the target. Use
`draw_2d_with_options` in Rust or an additional `Draw2DOptions` argument in C#
and Python for effects. Options are captured at submission; subsequent mutation
does not alter an earlier draw. Texture GPU resources survive until the frame
finishes, even if their CPU owner is released after submission.

| Effect | Rust / Python | C# | Meaning |
| --- | --- | --- | --- |
| Tint/opacity | `tint` | `Tint` | Linear RGBA multiplier in `[0,1]`, default white |
| Crop | `source` | `SourceRectangle` | Source pixels, default whole image |
| Rotation | `rotation` | `Rotation` | Clockwise `Angle`, default zero |
| Pivot | `origin` | `Origin` | Offset from target top-left in target pixels |
| Clip | `clip` | `ClipRectangle` | Unrotated full-window pixel rectangle |
| Flip X | `flip_horizontally` | `FlipHorizontally` | Mirror sampled UVs horizontally |
| Flip Y | `flip_vertically` | `FlipVertically` | Mirror sampled UVs vertically |

For rotation about the target center, set origin to half its width and height.
Origin affects only rotation; an unrotated image still fills the target exactly.
Clipping does not move or rescale the sprite. The renderer intersects clips with
the window and rounds their edges upward to integer scissor coordinates.

Source coordinates must be nonnegative, dimensions positive, and the region
must fit in the texture. Sampling clamps to source texel centers so a cropped
atlas cell does not bleed into its neighbors. Filtering is bilinear. Flips do
not change target geometry. Rectangles must be finite, within +/-10 million
pixels, and have nonnegative target/clip dimensions. Empty targets/clips and
zero opacity are no-ops. Invalid inputs produce normal binding exceptions or
Rust/C error results.

```csharp
graphics.Draw2D(texture, new Rect(360, 48, 160, 160), new Draw2DOptions {
    SourceRectangle = new Rect(32, 0, 32, 32),
    Tint = new Color(1, 1, 1, 0.8f),
    Rotation = Angle.Degrees(25),
    Origin = new Vec2(80, 80),
    FlipHorizontally = true,
    ClipRectangle = new Rect(340, 30, 200, 200)
});
```

## Text

The default is the installed **Segoe UI** font at 24 physical pixels. DirectWrite
provides Unicode shaping and system glyph fallback; Direct2D rasterizes a
coverage mask. Text is rendered through the same overlay path as sprites.
No font file, COM initialization, or additional installation is required.

`TextOptions` selects an installed family and em size, plus the sprite effects.
C# and Python expose these directly on the options object. Rust puts sprite
effects in `TextOptions.draw`. Source cropping is unsupported for text.

```csharp
var style = new TextOptions { FontFamily = "Segoe UI", FontSize = 32,
                              Tint = new Color(0.4f, 0.8f, 1) };
Vec2 size = graphics.MeasureText("Start game", style);
graphics.DrawText("Start game", new Vec2(40, 40), style);
```

Rust offers `draw_text_with_options` and `measure_text_with_options`; Python uses
`draw_text(..., options=style)` and `measure_text(text, style)`. Python's optional
`color` argument overrides tint for that draw without modifying the style.

Position is the logical layout's top-left; glyph overhangs are retained around
it. Measurement returns layout width including trailing spaces and line height,
before rotation. Empty text measures zero. Newlines are supported; line wrapping,
rich text, custom font files, and editable text controls are outside this initial
interface. Each label must fit 2048 by 2048 pixels including raster margins.
Font em size must be finite and from 1 through 256. Text/family names must not
contain NUL; limits are 65,536 UTF-8 bytes per label and 256 per font family name.

Labels are cached by text, family and size. Tint, position and other sprite
effects reuse the same image. The cache keeps up to 128 labels and 16 MiB of
pixel data, evicting the least recently used entries. Cache misses rasterize
synchronously on a private COM worker. This suits menus and HUD labels; very
large numbers of unique labels every frame incur rasterization/upload work.

## Examples

After pulling, rebuild shaders and the native library once from the repository
root. An existing editable Python installation continues to work.

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\build-shaders.ps1
cargo build --locked --lib --examples
cargo run --locked --example menus
dotnet run --project examples/csharp/Menus/Menus.csproj
python .\examples\python\menus.py
```

Use the mouse or Up/Down + Enter to select Pause, Exposure, and Quit. `F` flips
the rotating sprite, `C` toggles its clip, and Escape exits. Cycling exposure
through 0, 1, and 4 changes the 3D scene while leaving UI colors unchanged.
The panel is intentionally submitted before the 3D mesh to demonstrate overlay
semantics. A one-texel crop below the preview demonstrates atlas isolation.
