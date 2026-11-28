# Rendering and color conventions

Oxide accumulates scene lighting, emission, and visible backgrounds in a
full-resolution RGBA16F render target. At frame end, a fullscreen pass applies
exposure and Reinhard tone mapping (`x / (1 + x)`). An sRGB render-target
view applies the final transfer function in hardware. The swap chain remains
an ordinary 8-bit SDR surface.
This supports HDR scene values without requiring an HDR monitor.

There is one display transform per frame, after all camera viewports.
[2D overlays and text](drawing-2d.md) compose afterward, using linear-space
alpha blending through that sRGB view; they ignore exposure and tone mapping. Background
shaders do not tone-map independently. The HDR surface follows the window size
and is recreated after resize when the previous frame has finished on the GPU.

## Texture interpretation

| Source or use | Interpretation |
| --- | --- |
| LDR image files, glTF images, and RGBA8 procedural bytes used as color | sRGB RGB; alpha remains linear |
| Normal-map use of any RGBA8 image | Numeric linear channels; no sRGB decode |
| HDR/EXR image data | Linear floating-point radiance |
| `Texture::solid(Color)` and equivalent bindings | Linear color components, quantized to RGBA8 |
| Material factors, emission, light colors, and clear colors | Linear values |

Color decoding happens before texture filtering. An image reused as both an
albedo and normal map receives separate sRGB and numeric shader-resource views.
The GPU upload is shared. Raw backgrounds, GGX-filtered specular maps, and
irradiance maps occupy separate cache entries even if they share a source image.
CPU environment preprocessing decodes sRGB before bilinear filtering too.
The GGX lighting map is bounded to 512 pixels on its longest axis and has up to
five roughness levels; tiny inputs use only as many mips as their dimensions
support. The original background keeps its full resolution. This keeps
the one-time CPU convolution cost independent of the size of an imported sky.

glTF base-color factors multiply decoded colors in the shader; factors are not
baked into encoded bytes. Compiled texture and material payload revisions have
been bumped so old caches rebuild automatically with encoding metadata intact.

## Exposure

Exposure is a finite, non-negative **linear multiplier**, not an EV value. It
defaults to one; two is one stop brighter, one-half is one stop darker, and zero
produces black. Set it in a draw callback:

```rust
graphics.set_exposure(2.0)?;
```

```csharp
graphics.Exposure = 2.0f;
```

```python
graphics.exposure = 2.0
```

C uses `oxide_graphics_get_exposure` and `oxide_graphics_set_exposure`. The
setting persists between frames. The last value set in a draw callback applies
to the entire final surface, including all cameras and the background.

The three materials examples support **Q** to halve exposure, **E** to double
it, and **R** to reset it. The title displays the current value. Changing exposure
should affect the sky, lit surfaces, and emissive surfaces together. Also check
window resizing and the Rust split-screen example when validating a GPU backend.

## Shadows

Point, spot, and directional lights own optional shadow state and use queued mesh draws, with
configurable coverage, resolution, receiver bias, and per-draw transparency.
See [directional shadows](shadows.md) for the submission API and examples. Shadows modulate direct lighting in
linear HDR space before the shared display transform.

## Visible transparency

The per-draw `transparency` scalar controls both visible opacity and shadow
transmission. Opaque meshes establish depth first; partially transparent meshes
blend back to front per camera with depth testing and no depth writes. Blending
happens in linear RGBA16F before exposure and tone mapping. A value of one is
invisible and contributes no color or depth. See [shadows and transparency](shadows.md)
for the shared API and the mesh-sorting limitations.

Camera feeds can be bound to world-space rectangles. Their HDR images are sampled
without an intermediate display conversion. See [camera surfaces](camera-surfaces.md).
