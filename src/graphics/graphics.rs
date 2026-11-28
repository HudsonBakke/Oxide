use crate::{math::{Rect, Rect3D, Vec2}, math::Transform, platform::window::PlatformWindow};

use super::{
    Color, Texture, Draw2DOptions, TextOptions, Camera, DirectionalLight, Environment, GraphicsResult, LightSource, Material, Mesh, PointLight,
    SpotLight,
    backend::d3d12::{D3D12Device, D3D12Renderer, D3D12SwapChain},
};

/// Per-frame rendering interface supplied to the game's draw callback.
///
/// `Graphics` owns the active graphics backend and coordinates
/// rendering, presentation, and surface resizing.
pub struct Graphics {
    text_cache: super::text::TextCache,
    device: D3D12Device,
    renderer: D3D12Renderer,
    swap_chain: D3D12SwapChain,
}

impl Graphics {
    /// Linear exposure multiplier for the final display pass; defaults to one.
    ///
    /// Persists across frames. The last value set during draw applies to the
    /// entire 3D surface, including all camera viewports and backgrounds.
    /// Screen-space sprites and text are unaffected.
    pub fn exposure(&self) -> f32 {
        self.renderer.exposure()
    }

    /// Sets final-pass exposure; zero produces black.
    ///
    /// Returns InvalidArgument for negative, infinite, or NaN values.
    pub fn set_exposure(&mut self, exposure: f32) -> crate::OxideResult<()> {
        validate_exposure(exposure)?;
        self.renderer.set_exposure(exposure);
        Ok(())
    }

    /// Creates the graphics subsystem for the given window.
    pub(crate) fn new(window: &PlatformWindow) -> GraphicsResult<Self> {
        let size = window.inner().inner_size();

        let device = D3D12Device::new()?;

        let renderer = D3D12Renderer::new(&device)?;

        let swap_chain = D3D12SwapChain::new(&device, window.hwnd()?, size.width.max(1), size.height.max(1))?;

        Ok(Self {
            text_cache: Default::default(),
            device,
            renderer,
            swap_chain,
        })
    }

    pub(crate) fn begin_frame(&mut self) -> GraphicsResult<()> {
        self.renderer.begin_frame(&self.device, &self.swap_chain)?;

        Ok(())
    }

    pub(crate) fn end_frame(&mut self) -> GraphicsResult<()> {
        self.renderer.end_frame(&self.device, &self.swap_chain)?;

        Ok(())
    }

    /// Draws a texture over the 3D scene in full-window physical pixels.
    /// No camera is required. Overlays retain submission order and ignore exposure.
    pub fn draw_2d(&mut self, texture: &Texture, target: Rect) -> GraphicsResult<()> {
        self.draw_2d_with_options(texture, target, Draw2DOptions::default())
    }

    /// Draws a straight-alpha sprite with captured tint, crop, rotation and clip.
    /// Source pixels must lie inside the texture. Invalid values return an error;
    /// zero target dimensions or alpha draw nothing. Assets may be dropped after submission.
    pub fn draw_2d_with_options(&mut self, texture: &Texture, target: Rect, options: Draw2DOptions) -> GraphicsResult<()> {
        self.renderer.draw_2d(&self.device, texture, target, options)?;
        Ok(())
    }

    /// Draws a label using the default installed font at 24 physical pixels.
    pub fn draw_text(&mut self, text: &str, position: Vec2, color: Color) -> GraphicsResult<()> {
        let mut options = TextOptions::default(); options.draw.tint = color;
        self.draw_text_with_options(text, position, &options)
    }

    /// Draws UTF-8 text in an installed font using screen-space sprite effects.
    /// Position is the layout top-left. Newlines are supported; wrapping is explicit.
    /// Labels must fit 2048 by 2048 pixels. Source cropping is unsupported for text.
    pub fn draw_text_with_options(&mut self, text: &str, position: Vec2, options: &TextOptions) -> GraphicsResult<()> {
        let image = self.text_cache.get(text, options)?;
        let mut draw = options.draw;
        draw.origin = draw.origin - image.offset;
        self.draw_2d_with_options(&image.texture, Rect::new(position.x + image.offset.x, position.y + image.offset.y, image.texture.width() as f32, image.texture.height() as f32), draw)
    }

    /// Measures the default font's logical layout in physical pixels, including trailing spaces.
    pub fn measure_text(&mut self, text: &str) -> GraphicsResult<Vec2> {
        self.measure_text_with_options(text, &TextOptions::default())
    }

    /// Measures a label with the same font and size used by drawing, before rotation.
    /// Empty text has zero size. Measurement shares a bounded cache with drawing.
    pub fn measure_text_with_options(&mut self, text: &str, options: &TextOptions) -> GraphicsResult<Vec2> {
        Ok(self.text_cache.get(text, options)?.size)
    }

    /// Submits a mesh with opaque rendering and shadows, capturing its current draw state.
    ///
    /// Geometry is rendered at frame end. A camera must be bound first.
    /// See [`Self::draw_mesh_with_transparency`] for ownership and capacity errors.
    pub fn draw_mesh(
        &mut self,
        mesh: &Mesh,
        material: &Material,
        transform: &Transform,
    ) -> GraphicsResult<()> {
        self.draw_mesh_with_transparency(mesh, material, transform, 0.0)
    }

    /// Submits a mesh with finite transparency in [0, 1].
    ///
    /// Zero is opaque, one is invisible and casts no shadow, and intermediate values
    /// transmit that fraction of light at each front-facing surface. Up to eight
    /// distinct transparent depths are stored per ray; further layers are ignored.
    /// Visible opacity is one minus transparency. Camera, lights, material,
    /// geometry, and transform are captured at this call, before later mutations.
    /// Submitted meshes/materials can be dropped before the frame completes.
    ///
    /// # Errors
    /// Returns an error for invalid transparency, a missing camera, more than
    /// eight active lights of any type, more than 32 shadow projections per frame,
    /// or GPU allocation/descriptor exhaustion. Earlier successful draws remain
    /// queued. Release unused assets and finish the frame before retrying a
    /// descriptor allocation that exceeded the available contiguous capacity.
    pub fn draw_mesh_with_transparency(
        &mut self,
        mesh: &Mesh,
        material: &Material,
        transform: &Transform,
        transparency: f32,
    ) -> GraphicsResult<()> {
        self.renderer.draw_mesh(&self.device, mesh, material, transform, transparency)?;
        Ok(())
    }

    /// Resizes the graphics presentation surface.
    pub(crate) fn resize(&mut self, width: u32, height: u32) -> GraphicsResult<()> {
        if width == 0 || height == 0 || (width == self.swap_chain.width() && height == self.swap_chain.height()) {
            return Ok(());
        }

        self.renderer.prepare_for_resize(&self.device)?;

        self.swap_chain.resize(&self.device, width, height)?;

        Ok(())
    }

    /// Returns the preferred backbuffer width in physical pixels.
    pub fn preferred_backbuffer_width(&self) -> u32 {
        crate::app::window_width()
    }

    /// Returns the preferred backbuffer height in physical pixels.
    pub fn preferred_backbuffer_height(&self) -> u32 {
        crate::app::window_height()
    }

    /// Requests a new backbuffer/window width.
    pub fn set_preferred_backbuffer_width(&mut self, width: u32) {
        crate::app::set_preferred_backbuffer_size(width, crate::app::window_height());
    }

    /// Requests a new backbuffer/window height.
    pub fn set_preferred_backbuffer_height(&mut self, height: u32) {
        crate::app::set_preferred_backbuffer_size(crate::app::window_width(), height);
    }

    /// Binds a camera to a world-space display surface for subsequent draws.
    ///
    /// Oxide creates an HDR image (512 pixels on its longest edge), then displays
    /// it on the rectangle in every screen-bound camera. Its normal selects the
    /// visible side. Feed cameras omit monitor surfaces, preventing recursion.
    /// Bind each visible surface every frame; at most 16 distinct surfaces are
    /// supported per frame. Equal rectangles share a feed; the last binding wins.
    pub fn bind_camera_3d(&mut self, camera: &Camera, destination: Rect3D) -> GraphicsResult<()> {
        self.bind_camera_3d_with_resolution(camera, destination, 0, 0)
    }

    /// Binds a world-space camera surface with an explicit image width/height.
    ///
    /// Both dimensions must be 1..2048, or both zero for automatic size. The
    /// camera aspect is determined by the world rectangle, independent of pixel
    /// resolution. One rectangle cannot use different sizes within a frame.
    pub fn bind_camera_3d_with_resolution(&mut self, camera: &Camera, destination: Rect3D, width: u32, height: u32) -> GraphicsResult<()> {
        self.renderer.bind_camera_3d(&self.device, camera, destination, width, height)?;
        Ok(())
    }

    /// Binds a camera and destination rectangle for subsequent draw calls.
    pub fn bind_camera(&mut self, camera: &Camera, destination: Rect) -> GraphicsResult<()> {
        self.renderer.bind_camera(&self.device, camera, destination)?;

        Ok(())
    }

    /// Returns a rectangle covering the entire current render surface.
    pub fn full_rect(&self) -> Rect {
        Rect::new(
            0.0,
            0.0,
            self.swap_chain.width() as f32,
            self.swap_chain.height() as f32,
        )
    }

    /// Sets the environment used for ambient diffuse and specular IBL.
    pub fn set_environment(&mut self, environment: &Environment) {
        self.renderer.set_environment(environment.clone());
    }

    /// Removes environment lighting.
    pub fn clear_environment(&mut self) {
        self.renderer.clear_environment();
    }

    /// Adds a light to the active lighting state.
    ///
    /// The light affects subsequent draw calls until the lights are cleared
    /// or the current frame ends.
    pub fn add_light<L: LightSource + ?Sized>(&mut self, light: &L) {
        light.add_to_graphics(self);
    }

    /// Removes all currently active lights.
    pub fn clear_lights(&mut self) {
        self.renderer.clear_lights();
    }

    pub(crate) fn add_point_light_internal(&mut self, light: &PointLight) {
        self.renderer.add_point_light(light);
    }

    pub(crate) fn add_spot_light_internal(&mut self, light: &SpotLight) {
        self.renderer.add_spot_light(light);
    }

    pub(crate) fn add_directional_light_internal(&mut self, light: &DirectionalLight) {
        self.renderer.add_directional_light(light);
    }
}

fn validate_exposure(exposure: f32) -> crate::OxideResult<()> {
    if !exposure.is_finite() || exposure < 0.0 {
        return Err(crate::OxideError::InvalidArgument {
            argument: "exposure",
            message: "exposure must be finite and non-negative",
        });
    }
    Ok(())
}
