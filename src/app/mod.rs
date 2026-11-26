use std::sync::{
    Mutex, MutexGuard, OnceLock,
    atomic::{AtomicBool, AtomicU32, Ordering},
};

#[cfg(target_os = "windows")]
use winit::platform::windows::WindowAttributesExtWindows;

use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, DeviceId, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    window::{CursorGrabMode, Fullscreen, Icon, WindowAttributes, WindowId},
};

use std::time::Instant;

use crate::graphics::{Graphics, Texture};
use crate::input::{
    advance_keyboard_frame, advance_mouse_frame, handle_cursor_moved, handle_keyboard_input,
    handle_mouse_input, handle_mouse_motion, handle_mouse_wheel,
};
use crate::platform::window::PlatformWindow;

#[derive(Debug)]
enum AppCommand {
    Exit,
    SetTitle(String),
    SetIcon {
        width: u32,
        height: u32,
        rgba: Vec<u8>,
    },
    SetBackbufferSize(u32, u32),
    SetFullscreen(bool),
    SetCursorVisible(bool),
    SetCursorGrabbed(bool),
}

static COMMANDS: OnceLock<Mutex<Vec<AppCommand>>> = OnceLock::new();
static WINDOW_WIDTH: AtomicU32 = AtomicU32::new(0);
static WINDOW_HEIGHT: AtomicU32 = AtomicU32::new(0);
static WINDOW_FOCUSED: AtomicBool = AtomicBool::new(false);
static WINDOW_FULLSCREEN: AtomicBool = AtomicBool::new(false);

fn commands() -> MutexGuard<'static, Vec<AppCommand>> {
    COMMANDS
        .get_or_init(|| Mutex::new(Vec::new()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn push_command(command: AppCommand) {
    commands().push(command);
}

/// Requests that the running Oxide application exit.
pub fn exit() {
    push_command(AppCommand::Exit);
}

/// Changes the window title.
pub fn set_window_name(name: impl Into<String>) {
    push_command(AppCommand::SetTitle(name.into()));
}

/// Changes the window icon using the RGBA pixels of a texture.
pub fn set_window_logo(texture: &Texture) {
    push_command(AppCommand::SetIcon {
        width: texture.width(),
        height: texture.height(),
        rgba: texture.pixels().to_vec(),
    });
}

/// Requests a new window/backbuffer size.
pub fn set_preferred_backbuffer_size(width: u32, height: u32) {
    if width == 0 || height == 0 {
        return;
    }

    push_command(AppCommand::SetBackbufferSize(width, height));
}

/// Returns the current client width in physical pixels.
pub fn window_width() -> u32 {
    WINDOW_WIDTH.load(Ordering::Relaxed)
}

/// Returns the current client height in physical pixels.
pub fn window_height() -> u32 {
    WINDOW_HEIGHT.load(Ordering::Relaxed)
}

/// Returns whether the application window currently has focus.
pub fn is_window_focused() -> bool {
    WINDOW_FOCUSED.load(Ordering::Relaxed)
}

/// Enables or disables borderless fullscreen on the current monitor.
pub fn set_fullscreen(fullscreen: bool) {
    push_command(AppCommand::SetFullscreen(fullscreen));
}

/// Returns whether Oxide has requested fullscreen mode.
pub fn is_fullscreen() -> bool {
    WINDOW_FULLSCREEN.load(Ordering::Relaxed)
}

/// Shows or hides the OS cursor over the game window.
pub fn set_cursor_visible(visible: bool) {
    push_command(AppCommand::SetCursorVisible(visible));
}

/// Grabs/releases the cursor for mouse-look style controls.
pub fn set_cursor_grabbed(grabbed: bool) {
    push_command(AppCommand::SetCursorGrabbed(grabbed));
}

struct App<G: Game> {
    game: G,
    config: AppConfig,
    window: Option<PlatformWindow>,
    graphics: Option<Graphics>,
    start_time: Option<Instant>,
    last_frame_time: Option<Instant>,
}

impl<G: Game> App<G> {
    fn run_frame(&mut self, event_loop: &ActiveEventLoop) {
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
        self.apply_commands(event_loop);

        if let Some(graphics) = &mut self.graphics {
            graphics
                .begin_frame()
                .expect("failed to begin Oxide graphics frame");

            self.game.draw(graphics);

            graphics
                .end_frame()
                .expect("failed to finish Oxide graphics frame");
        }

        self.apply_commands(event_loop);

        advance_keyboard_frame();
        advance_mouse_frame();

        if let Some(window) = &self.window {
            window.inner().request_redraw();
        }
    }

    fn resize(&mut self, width: u32, height: u32) {
        WINDOW_WIDTH.store(width, Ordering::Relaxed);
        WINDOW_HEIGHT.store(height, Ordering::Relaxed);

        if let Some(graphics) = &mut self.graphics {
            graphics
                .resize(width, height)
                .expect("failed to resize Oxide graphics");
        }
    }

    fn apply_commands(&mut self, event_loop: &ActiveEventLoop) {
        let pending = {
            let mut queue = commands();
            std::mem::take(&mut *queue)
        };

        for command in pending {
            match command {
                AppCommand::Exit => event_loop.exit(),

                AppCommand::SetTitle(title) => {
                    if let Some(window) = &self.window {
                        window.inner().set_title(&title);
                    }
                }

                AppCommand::SetIcon {
                    width,
                    height,
                    rgba,
                } => {
                    if let Some(window) = &self.window {
                        if let Ok(icon) = Icon::from_rgba(rgba, width, height) {
                            window.inner().set_window_icon(Some(icon));
                        }
                    }
                }

                AppCommand::SetBackbufferSize(width, height) => {
                    if let Some(window) = &self.window {
                        let _ = window
                            .inner()
                            .request_inner_size(winit::dpi::PhysicalSize::new(width, height));
                    }

                    self.resize(width, height);
                }

                AppCommand::SetFullscreen(fullscreen) => {
                    if let Some(window) = &self.window {
                        let mode = if fullscreen {
                            Some(Fullscreen::Borderless(None))
                        } else {
                            None
                        };

                        window.inner().set_fullscreen(mode);
                        WINDOW_FULLSCREEN.store(fullscreen, Ordering::Relaxed);
                    }
                }

                AppCommand::SetCursorVisible(visible) => {
                    if let Some(window) = &self.window {
                        window.inner().set_cursor_visible(visible);
                    }
                }

                AppCommand::SetCursorGrabbed(grabbed) => {
                    if let Some(window) = &self.window {
                        let result = if grabbed {
                            window
                                .inner()
                                .set_cursor_grab(CursorGrabMode::Locked)
                                .or_else(|_| {
                                    window.inner().set_cursor_grab(CursorGrabMode::Confined)
                                })
                        } else {
                            window.inner().set_cursor_grab(CursorGrabMode::None)
                        };

                        let _ = result;
                    }
                }
            }
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

        #[cfg(target_os = "windows")]
        let attributes = attributes.with_drag_and_drop(false);

        let window = event_loop
            .create_window(attributes)
            .expect("failed to create Oxide window");

        let platform_window = PlatformWindow::new(window);

        let graphics =
            Graphics::new(&platform_window).expect("failed to initialize Oxide graphics");

        let size = platform_window.inner().inner_size();
        WINDOW_WIDTH.store(size.width, Ordering::Relaxed);
        WINDOW_HEIGHT.store(size.height, Ordering::Relaxed);
        WINDOW_FOCUSED.store(platform_window.inner().has_focus(), Ordering::Relaxed);

        self.window = Some(platform_window);
        self.graphics = Some(graphics);

        let now = Instant::now();

        self.start_time = Some(now);
        self.last_frame_time = Some(now);

        self.game.initialize();
        self.apply_commands(event_loop);

        if let Some(window) = &self.window {
            window.inner().request_redraw();
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent,
    ) {
        if let DeviceEvent::MouseMotion { delta } = event {
            handle_mouse_motion(delta);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),

            WindowEvent::RedrawRequested => {
                self.run_frame(event_loop);
            }

            WindowEvent::Resized(size) => {
                self.resize(size.width, size.height);
            }

            WindowEvent::Focused(focused) => {
                WINDOW_FOCUSED.store(focused, Ordering::Relaxed);
            }

            WindowEvent::KeyboardInput { event, .. } => {
                handle_keyboard_input(event.physical_key, event.state);
            }

            WindowEvent::MouseInput { state, button, .. } => {
                handle_mouse_input(button, state);
            }

            WindowEvent::CursorMoved { position, .. } => {
                handle_cursor_moved(position);
            }

            WindowEvent::MouseWheel { delta, .. } => {
                handle_mouse_wheel(delta);
            }

            _ => {}
        }
    }
}

/// Runs an Oxide game using the given configuration.
pub fn run<G: Game>(game: G, config: AppConfig) {
    let event_loop = EventLoop::new().expect("failed to create event loop");

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
/// Public Oxide AppConfig API.
pub struct AppConfig {
    /// Public Oxide title API.
    pub title: String,
    /// Public Oxide width API.
    pub width: u32,
    /// Public Oxide height API.
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

/// Public Oxide Game API.
pub trait Game {
    fn initialize(&mut self) {}
    fn update(&mut self, _time: &GameTime) {}
    fn draw(&mut self, _graphics: &mut Graphics) {}
}

/// Timing information for the current frame.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
/// Public Oxide GameTime API.
pub struct GameTime {
    delta_seconds: f64,
    total_seconds: f64,
}

impl GameTime {
    /// Public Oxide delta seconds API.
    pub fn delta_seconds(&self) -> f64 {
        self.delta_seconds
    }

    /// Public Oxide total seconds API.
    pub fn total_seconds(&self) -> f64 {
        self.total_seconds
    }
}
