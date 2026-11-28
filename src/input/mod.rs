//! Physical keyboard and mouse snapshots with held-state and per-frame transition queries.

mod keyboard;
mod mouse;

pub use keyboard::{Keyboard, KeyboardState, Keys};

pub use mouse::{Mouse, MouseButton, MouseState};

pub(crate) use keyboard::{advance_keyboard_frame, handle_keyboard_input};

pub(crate) use mouse::{
    advance_mouse_frame, handle_cursor_moved, handle_mouse_input, handle_mouse_motion,
    handle_mouse_wheel,
};

// Shared lifecycle operations keep keyboard and mouse semantics aligned in all bindings.
pub(crate) fn release_input() {
    keyboard::release_keyboard();
    mouse::release_mouse();
}

pub(crate) fn reset_input() {
    keyboard::reset_keyboard();
    mouse::reset_mouse();
}

pub(crate) use mouse::clear_mouse_motion;
