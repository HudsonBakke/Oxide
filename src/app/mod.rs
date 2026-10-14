use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{WindowAttributes, WindowId},
};

use std::time::Instant;

use crate::platform::window::PlatformWindow;
use crate::graphics::backend::d3d12::{
    D3D12Device,
    D3D12Renderer,
    D3D12SwapChain,
};

struct App<G: Game> {
    game: G,
    config: AppConfig,
    window: Option<PlatformWindow>,
    graphics_device: Option<D3D12Device>,
    swap_chain: Option<D3D12SwapChain>,
    renderer: Option<D3D12Renderer>,
    start_time: Option<Instant>,
    last_frame_time: Option<Instant>,
}

impl<G: Game> App<G> {
    fn run_frame(&mut self) {
        let now = Instant::now();

        let start_time = self
            .start_time
            .expect("application clock was not initialized");

        let last_frame_time = self
            .last_frame_time
            .expect("application clock was not initialized");

        let time = GameTime {
            delta_seconds: (now - last_frame_time).as_secs_f64(),
            total_seconds: (now - start_time).as_secs_f64(),
        };

        self.last_frame_time = Some(now);

        self.game.update(&time);
        self.game.draw();

        if let (
            Some(device),
            Some(swap_chain),
            Some(renderer),
        ) = (
            &self.graphics_device,
            &self.swap_chain,
            &mut self.renderer,
        ) {
            renderer
                .render(device, swap_chain)
                .expect("Direct3D 12 rendering failed");
        }

        if let Some(window) = &self.window {
            window.inner().request_redraw();
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        if let (
            Some(device),
            Some(renderer),
            Some(swap_chain),
        ) = (
            &self.graphics_device,
            &mut self.renderer,
            &mut self.swap_chain,
        ) {
            renderer
                .prepare_for_resize(device)
                .expect("failed to prepare graphics system for resize");

            swap_chain
                .resize(device, width, height)
                .expect("failed to resize Direct3D 12 swap chain");
        }
    }
}

impl<G: Game> ApplicationHandler for App<G> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let attributes = WindowAttributes::default()
            .with_title(&self.config.title)
            .with_inner_size(winit::dpi::LogicalSize::new(
                self.config.width,
                self.config.height,
            ));

        let window = event_loop
            .create_window(attributes)
            .expect("failed to create Oxide window");

        let platform_window = PlatformWindow::new(window);

        let graphics_device =
            D3D12Device::new().expect("failed to create Direct3D 12 device");

        let renderer = D3D12Renderer::new(&graphics_device)
            .expect("failed to create Direct3D 12 renderer");

        let swap_chain = D3D12SwapChain::new(
            &graphics_device,
            platform_window.hwnd(),
            self.config.width,
            self.config.height,
        )
        .expect("failed to create Direct3D 12 swap chain");

        self.window = Some(platform_window);
        self.graphics_device = Some(graphics_device);
        self.swap_chain = Some(swap_chain);
        self.renderer = Some(renderer);

        let now = Instant::now();

        self.start_time = Some(now);
        self.last_frame_time = Some(now);

        self.game.initialize();

        if let Some(window) = &self.window {
            window.inner().request_redraw();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                self.run_frame();
            }

            WindowEvent::Resized(size) => {
                self.resize(size.width, size.height);
            }

            _ => {}
        }
    }
}

/// Runs an Oxide game using the given configuration.
pub fn run<G: Game>(game: G, config: AppConfig) {
    let event_loop = EventLoop::new()
        .expect("failed to create event loop");

    let mut app = App {
        game,
        config,
        window: None,
        graphics_device: None,
        swap_chain: None,
        renderer: None,
        start_time: None,
        last_frame_time: None,
    };

    event_loop
        .run_app(&mut app)
        .expect("Oxide application failed");
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            title: String::from("Oxide"),
            width: 1280,
            height: 720,
        }
    }
}

pub trait Game {
    fn initialize(&mut self) {}
    fn update(&mut self, _time: &GameTime) {}
    fn draw(&mut self) {}
}

/// Timing information for the current frame.
#[derive(Debug, Clone, Copy)]
pub struct GameTime {
    delta_seconds: f64,
    total_seconds: f64,
}

impl GameTime {
    /// Returns the elapsed time since the previous update, in seconds.
    pub fn delta_seconds(&self) -> f64 {
        self.delta_seconds
    }

    /// Returns the total elapsed time since the application started, in seconds.
    pub fn total_seconds(&self) -> f64 {
        self.total_seconds
    }
}