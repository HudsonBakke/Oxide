//! Application lifecycle, window commands, and elapsed frame timing.
mod error;
mod runtime;

pub use error::{AppError, AppStage};
pub use runtime::{AppConfig, Game, GameTime, exit, is_fullscreen, is_window_focused,
    run, set_cursor_grabbed, set_cursor_visible, set_fullscreen, set_preferred_backbuffer_size,
    set_window_logo, set_window_name, window_height, window_width};
