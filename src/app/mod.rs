use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{WindowAttributes, WindowId},
};

use std::time::Instant;

use crate::platform::window::PlatformWindow;
use crate::graphics::Graphics;

struct App<G: Game> {
    game: G,
    config: AppConfig,
    window: Option<PlatformWindow>,
    graphics: Option<Graphics>,
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

        if let Some(graphics) = &mut self.graphics {
            graphics
                .begin_frame()
                .expect("failed to begin Oxide graphics frame");

            self.game.draw(graphics);

            graphics
                .end_frame()
                .expect("failed to finish Oxide graphics frame");
        }

        if let Some(window) = &self.window {
            window.inner().request_redraw();
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        if let Some(graphics) = &mut self.graphics {
            graphics
                .resize(width, height)
                .expect("failed to resize Oxide graphics");
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

        let graphics = Graphics::new(&platform_window)
            .expect("failed to initialize Oxide graphics");

        self.window = Some(platform_window);
        self.graphics = Some(graphics);

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
        graphics: None,
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
    fn draw(&mut self, _graphics: &mut Graphics) {}
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