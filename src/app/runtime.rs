use super::{AppError, AppStage};

use std::{any::Any, panic::{catch_unwind, resume_unwind, AssertUnwindSafe}};
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

/// Initial window title and logical-pixel client size; defaults to Oxide at 1280 by 720.
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Initial window title; defaults to Oxide.
    pub title: String,
    /// Initial client width in logical pixels; must be positive.
    pub width: u32,
    /// Initial client height in logical pixels; must be positive.
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

/// Application lifecycle implemented by initialize, update, and draw callbacks.
pub trait Game {
    /// Called once after the window and graphics are ready; load persistent resources here.
    fn initialize(&mut self) {}
    /// Called once before drawing each frame with elapsed time in seconds; update game logic and read input here.
    fn update(&mut self, _time: &GameTime) {}
    /// Called each frame to bind cameras, set lighting, and submit draw calls.
    fn draw(&mut self, _graphics: &mut Graphics) {}
}

/// Timing information for the current frame.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GameTime {
    delta_seconds: f64,
    total_seconds: f64,
}

impl GameTime {
    /// Seconds elapsed since the previous frame's update.
    pub fn delta_seconds(&self) -> f64 {
        self.delta_seconds
    }

    /// Seconds elapsed since the application clock started.
    pub fn total_seconds(&self) -> f64 {
        self.total_seconds
    }
}

#[derive(Debug)]
enum AppCommand {
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

static RUNNING: AtomicBool = AtomicBool::new(false);
static EXIT_REQUESTED: AtomicBool = AtomicBool::new(false);

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
    EXIT_REQUESTED.store(true, Ordering::Release);
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

// Owns global per-run state, including cleanup after errors or callback panics.
struct RunSession;
impl RunSession {
    fn acquire() -> Result<Self, AppError> {
        RUNNING.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .map_err(|_| AppError::new(AppStage::AlreadyRunning,
                std::io::Error::other("nested and concurrent runs are unsupported")))?;
        reset_window_state();
        Ok(Self)
    }
}
impl Drop for RunSession {
    fn drop(&mut self) {
        reset_window_state();
        RUNNING.store(false, Ordering::Release);
    }
}
fn reset_window_state() {
    commands().clear();
    EXIT_REQUESTED.store(false, Ordering::Release);
    WINDOW_WIDTH.store(0, Ordering::Relaxed);
    WINDOW_HEIGHT.store(0, Ordering::Relaxed);
    WINDOW_FOCUSED.store(false, Ordering::Relaxed);
    WINDOW_FULLSCREEN.store(false, Ordering::Relaxed);
}

#[derive(Default)]
struct Lifecycle {
    error: Option<AppError>,
    panic: Option<Box<dyn Any + Send>>,
    stopped: bool,
}
impl Lifecycle {
    fn should_stop(&mut self) -> bool {
        self.stopped |= EXIT_REQUESTED.load(Ordering::Acquire);
        self.stopped
    }
    fn fail(&mut self, error: AppError) {
        if self.error.is_none() { self.error = Some(error); }
        self.stopped = true;
    }
    fn callback(&mut self, callback: impl FnOnce()) {
        if self.should_stop() { return; }
        if let Err(payload) = catch_unwind(AssertUnwindSafe(callback)) {
            self.panic = Some(payload);
            self.stopped = true;
        }
        self.should_stop();
    }
    fn graphics(&mut self, stage: AppStage, result: crate::graphics::GraphicsResult<()>) {
        if let Err(error) = result { self.fail(AppError::new(stage, error)); }
    }
    fn finish(self) -> Result<(), AppError> {
        if let Some(payload) = self.panic { resume_unwind(payload); }
        self.error.map_or(Ok(()), Err)
    }
}

struct App<G: Game> {
    game: G,
    lifecycle: Lifecycle,
    config: AppConfig,
    window: Option<PlatformWindow>,
    graphics: Option<Graphics>,
    start_time: Option<Instant>,
    last_frame_time: Option<Instant>,
}

impl<G: Game> App<G> {
    fn run_frame(&mut self, event_loop: &ActiveEventLoop) {
        if self.stop_if_requested(event_loop) { return; }
        let (Some(start_time), Some(last_frame_time)) = (self.start_time, self.last_frame_time) else { return; };
        let now = Instant::now();

        let time = GameTime {
            delta_seconds: (now - last_frame_time).as_secs_f64(),
            total_seconds: (now - start_time).as_secs_f64(),
        };

        self.last_frame_time = Some(now);

        self.lifecycle.callback(|| self.game.update(&time));
        if self.stop_if_requested(event_loop) { return; }
        self.apply_commands(event_loop);
        if self.stop_if_requested(event_loop) { return; }

        if let Some(graphics) = &mut self.graphics {
            self.lifecycle.graphics(AppStage::BeginFrame, graphics.begin_frame());
        }
        if self.stop_if_requested(event_loop) { return; }
        if let Some(graphics) = &mut self.graphics {
            self.lifecycle.callback(|| self.game.draw(graphics));
        }
        if self.stop_if_requested(event_loop) { return; }
        if let Some(graphics) = &mut self.graphics {
            self.lifecycle.graphics(AppStage::EndFrame, graphics.end_frame());
        }
        if self.stop_if_requested(event_loop) { return; }
        // Apply resizing only after submitting the frame, never inside an open list.
        self.apply_commands(event_loop);
        if self.stop_if_requested(event_loop) { return; }

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
            self.lifecycle.graphics(AppStage::Resize, graphics.resize(width, height));
        }
    }

    fn stop_if_requested(&mut self, event_loop: &ActiveEventLoop) -> bool {
        if self.lifecycle.should_stop() { event_loop.exit(); true } else { false }
    }

    fn apply_commands(&mut self, event_loop: &ActiveEventLoop) {
        if self.stop_if_requested(event_loop) { return; }
        let pending = {
            let mut queue = commands();
            std::mem::take(&mut *queue)
        };

        for command in pending {
            if self.stop_if_requested(event_loop) { break; }
            match command {

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
    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        self.lifecycle.stopped = true;
        drop(self.graphics.take());
        drop(self.window.take());
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.stop_if_requested(event_loop) || self.window.is_some() {
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

        let window = match event_loop.create_window(attributes) {
            Ok(window) => window,
            Err(error) => { self.lifecycle.fail(AppError::new(AppStage::Window, error)); self.stop_if_requested(event_loop); return; }
        };
        let platform_window = PlatformWindow::new(window);
        let graphics = match Graphics::new(&platform_window) {
            Ok(graphics) => graphics,
            Err(error) => { self.lifecycle.fail(AppError::new(AppStage::GraphicsInitialization, error)); self.stop_if_requested(event_loop); return; }
        };

        let size = platform_window.inner().inner_size();
        WINDOW_WIDTH.store(size.width, Ordering::Relaxed);
        WINDOW_HEIGHT.store(size.height, Ordering::Relaxed);
        WINDOW_FOCUSED.store(platform_window.inner().has_focus(), Ordering::Relaxed);

        self.window = Some(platform_window);
        self.graphics = Some(graphics);

        let now = Instant::now();

        self.start_time = Some(now);
        self.last_frame_time = Some(now);

        self.lifecycle.callback(|| self.game.initialize());
        if self.stop_if_requested(event_loop) { return; }
        self.apply_commands(event_loop);
        if self.stop_if_requested(event_loop) { return; }

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
        if self.stop_if_requested(event_loop) { return; }
        match event {
            WindowEvent::CloseRequested => { self.lifecycle.stopped = true; event_loop.exit(); },

            WindowEvent::RedrawRequested => {
                self.run_frame(event_loop);
            }

            WindowEvent::Resized(size) => {
                self.resize(size.width, size.height);
                self.stop_if_requested(event_loop);
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

/// Runs the blocking window event loop on the calling thread.
///
/// Calls initialize once, then update followed by draw each frame. Engine failures
/// return an AppError with the operation and original cause. Exit requests stop
/// before the next callback or frame submission. Rust callback panics are resumed
/// only after leaving the platform loop and dropping graphics/window resources.
/// Run on the platform's main event-loop thread. Winit permits one event loop per
/// process; nested/concurrent runs return an error. Zero dimensions are invalid.
pub fn run<G: Game>(game: G, config: AppConfig) -> Result<(), AppError> {
    validate_config(&config)?;
    let session = RunSession::acquire()?;
    // Some platform thread/lifetime checks panic instead of returning EventLoopError.
    let event_loop = catch_unwind(EventLoop::new).map_err(|payload| {
        let message = payload.downcast_ref::<&str>().map(|s| s.to_string())
            .or_else(|| payload.downcast_ref::<String>().cloned())
            .unwrap_or_else(|| "event loop creation panicked".into());
        AppError::new(AppStage::EventLoop, std::io::Error::other(message))
    })?.map_err(|error| AppError::new(AppStage::EventLoop, error))?;
    let mut app = App {
        game, lifecycle: Lifecycle::default(), config, window: None, graphics: None,
        start_time: None, last_frame_time: None,
    };
    if let Err(error) = event_loop.run_app(&mut app) {
        app.lifecycle.fail(AppError::new(AppStage::EventLoop, error));
    }
    // Make teardown precede returning errors or resuming a user panic.
    let lifecycle = std::mem::take(&mut app.lifecycle);
    drop(app.graphics.take());
    drop(app.window.take());
    drop(app);
    drop(session);
    lifecycle.finish()
}

fn validate_config(config: &AppConfig) -> Result<(), AppError> {
    if config.width == 0 || config.height == 0 || config.title.contains('\0') {
        return Err(AppError::new(AppStage::Configuration,
            std::io::Error::new(std::io::ErrorKind::InvalidInput,
                "window dimensions must be positive and title must not contain NUL")));
    }
    Ok(())
}
