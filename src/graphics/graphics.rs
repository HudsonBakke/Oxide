use crate::{math::Rect, math::Transform, platform::window::PlatformWindow};

use super::{
    Camera, DirectionalLight, Environment, GraphicsResult, LightSource, Material, Mesh, PointLight,
    SpotLight,
    backend::d3d12::{D3D12Device, D3D12Renderer, D3D12SwapChain},
};

/// Per-frame rendering interface supplied to the game's draw callback.
///
/// `Graphics` owns the active graphics backend and coordinates
/// rendering, presentation, and surface resizing.
pub struct Graphics {
    device: D3D12Device,
    renderer: D3D12Renderer,
    swap_chain: D3D12SwapChain,
}

impl Graphics {
    /// Linear exposure multiplier for the final display pass; defaults to one.
    ///
    /// Persists across frames. The last value set during draw applies to the
    /// entire surface, including all camera viewports and backgrounds.
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

        let swap_chain = D3D12SwapChain::new(&device, window.hwnd(), size.width, size.height)?;

        Ok(Self {
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

    /// Submits a mesh with opaque rendering and shadows, capturing its current draw state.
    ///
    /// Geometry is rendered at frame end. A camera must be bound first.
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
        if width == 0 || height == 0 {
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

#[cfg(test)]
mod tests {
    use super::validate_exposure;

    #[test]
    fn exposure_accepts_black_and_bright_values_but_rejects_invalid_inputs() {
        for valid in [0.0, 1.0, 8.0, f32::MAX] {
            assert!(validate_exposure(valid).is_ok());
        }
        for invalid in [-1.0, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
            assert!(validate_exposure(invalid).is_err());
        }
    }
}
