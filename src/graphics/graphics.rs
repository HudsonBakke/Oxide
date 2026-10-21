use crate::platform::window::PlatformWindow;

use super::{
    GraphicsResult,
    Mesh,
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
    pub fn draw_mesh(&mut self, mesh: &Mesh) -> GraphicsResult<()> {
        self.renderer.draw_mesh(
            &self.device,
            mesh,
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
}