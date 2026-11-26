mod keyboard;
mod mouse;

pub use keyboard::{
    Keyboard,
    KeyboardState,
    Keys,
};

pub use mouse::{
    Mouse,
    MouseButton,
    MouseState,
};

pub(crate) use keyboard::{
    advance_keyboard_frame,
    handle_keyboard_input,
};

pub(crate) use mouse::{
    advance_mouse_frame,
    handle_cursor_moved,
    handle_mouse_input,
    handle_mouse_wheel,
};
