use crate::{
    platform::window::PlatformWindow,
    math::Transform,
    math::Rect,
};

use super::{
    GraphicsResult,
    Mesh,
    Camera,
    DirectionalLight,
    SpotLight,
    PointLight,
    LightSource,
    Material,
    backend::d3d12::{
        D3D12Device,
        D3D12Renderer,
        D3D12SwapChain,
    }
};

/// Oxide's internal graphics subsystem.
///
/// `Graphics` owns the active graphics backend and coordinates
/// rendering, presentation, and surface resizing.
pub struct Graphics {
    device: D3D12Device,
    renderer: D3D12Renderer,
    swap_chain: D3D12SwapChain,
}

impl Graphics {
    /// Creates the graphics subsystem for the given window.
    pub(crate) fn new(window: &PlatformWindow) -> GraphicsResult<Self> {
        let size = window.inner().inner_size();

        let device = D3D12Device::new()?;

        let renderer = D3D12Renderer::new(&device)?;

        let swap_chain = D3D12SwapChain::new(
            &device,
            window.hwnd(),
            size.width,
            size.height,
        )?;

        Ok(Self {
            device,
            renderer,
            swap_chain,
        })
    }
    
    pub(crate) fn begin_frame(&mut self) -> GraphicsResult<()> {
        self.renderer.begin_frame(
            &self.device,
            &self.swap_chain,
        )?;

        Ok(())
    }

    pub(crate) fn end_frame(&mut self) -> GraphicsResult<()> {
        self.renderer.end_frame(
            &self.device,
            &self.swap_chain,
        )?;

        Ok(())
    }

    /// Draws a mesh during the current frame.
    pub fn draw_mesh(
        &mut self,
        mesh: &Mesh,
        material: &Material,
        transform: &Transform,
    ) -> GraphicsResult<()> {
        self.renderer.draw_mesh(
            &self.device,
            mesh,
            material,
            transform,
        )?;

        Ok(())
    }

    /// Resizes the graphics presentation surface.
    pub(crate) fn resize(
        &mut self,
        width: u32,
        height: u32,
    ) -> GraphicsResult<()> {
        if width == 0 || height == 0 {
            return Ok(());
        }

        self.renderer
            .prepare_for_resize(&self.device)?;

        self.swap_chain.resize(
            &self.device,
            width,
            height,
        )?;

        Ok(())
    }

    /// Binds a camera and destination rectangle for subsequent draw calls.
    pub fn bind_camera(
        &mut self,
        camera: &Camera,
        destination: Rect,
    ) -> GraphicsResult<()> {
        self.renderer.bind_camera(
            camera,
            destination,
        )?;

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

    /// Adds a light to the active lighting state.
    ///
    /// The light affects subsequent draw calls until the lights are cleared
    /// or the current frame ends.
    pub fn add_light<L: LightSource>(
        &mut self,
        light: &L,
    ) {
        light.add_to_graphics(self);
    }

    /// Removes all currently active lights.
    pub fn clear_lights(&mut self) {
        self.renderer.clear_lights();
    }

    pub(crate) fn add_point_light_internal(
        &mut self,
        light: &PointLight,
    ) {
        self.renderer.add_point_light(light);
    }

    pub(crate) fn add_spot_light_internal(
        &mut self,
        light: &SpotLight,
    ) {
        self.renderer.add_spot_light(light);
    }

    pub(crate) fn add_directional_light_internal(
        &mut self,
        light: &DirectionalLight,
    ) {
        self.renderer.add_directional_light(light);
    }
}