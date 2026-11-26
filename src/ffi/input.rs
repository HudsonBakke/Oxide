use crate::input::{
    Keyboard,
    Keys,
    Mouse,
    MouseButton,
};
use crate::math::Vec2;

use super::error::{
    ffi_boundary,
    set_last_error,
};
use super::OxideStatus;

const KEY_WORDS: usize = 2;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct OxideKeyboardState {
    pub current: [u64; KEY_WORDS],
    pub previous: [u64; KEY_WORDS],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct OxideMouseState {
    pub current_buttons: u64,
    pub previous_buttons: u64,
    pub position: Vec2,
    pub delta: Vec2,
    pub scroll_delta: Vec2,
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_keyboard_get_state(
    result: *mut OxideKeyboardState,
) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() {
            set_last_error("result pointer cannot be null");
            return OxideStatus::InvalidArgument;
        }

        let state = Keyboard::get_state();
        let mut native = OxideKeyboardState::default();

        for key in ALL_KEYS {
            let index = key_index(key);

            if state.is_key_down(key) {
                set_bit(&mut native.current, index);
            }

            if state.was_key_down(key) {
                set_bit(&mut native.previous, index);
            }
        }

        unsafe {
            *result = native;
        }

        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_mouse_get_state(
    result: *mut OxideMouseState,
) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() {
            set_last_error("result pointer cannot be null");
            return OxideStatus::InvalidArgument;
        }

        let state = Mouse::get_state();
        let mut current = 0_u64;
        let mut previous = 0_u64;

        for button in standard_and_other_buttons() {
            let bit = mouse_button_bit(button);

            if state.is_button_down(button) {
                current |= bit;
            }

            if state.was_button_down(button) {
                previous |= bit;
            }
        }

        unsafe {
            *result = OxideMouseState {
                current_buttons: current,
                previous_buttons: previous,
                position: state.position(),
                delta: state.delta(),
                scroll_delta: state.scroll_delta(),
            };
        }

        OxideStatus::Ok
    })
}

fn set_bit(words: &mut [u64; KEY_WORDS], index: usize) {
    words[index / 64] |= 1_u64 << (index % 64);
}

fn key_index(key: Keys) -> usize {
    key as usize
}

fn mouse_button_bit(button: MouseButton) -> u64 {
    match button {
        MouseButton::Left => 1 << 0,
        MouseButton::Right => 1 << 1,
        MouseButton::Middle => 1 << 2,
        MouseButton::Back => 1 << 3,
        MouseButton::Forward => 1 << 4,
        MouseButton::Other(value) if value < 59 => {
            1_u64 << (5 + value)
        }
        MouseButton::Other(_) => 0,
    }
}

fn standard_and_other_buttons() -> impl Iterator<Item = MouseButton> {
    [
        MouseButton::Left,
        MouseButton::Right,
        MouseButton::Middle,
        MouseButton::Back,
        MouseButton::Forward,
    ]
    .into_iter()
    .chain((0_u16..59).map(MouseButton::Other))
}

const ALL_KEYS: [Keys; 69] = [
    Keys::A, Keys::B, Keys::C, Keys::D, Keys::E, Keys::F, Keys::G,
    Keys::H, Keys::I, Keys::J, Keys::K, Keys::L, Keys::M, Keys::N,
    Keys::O, Keys::P, Keys::Q, Keys::R, Keys::S, Keys::T, Keys::U,
    Keys::V, Keys::W, Keys::X, Keys::Y, Keys::Z,
    Keys::Key0, Keys::Key1, Keys::Key2, Keys::Key3, Keys::Key4,
    Keys::Key5, Keys::Key6, Keys::Key7, Keys::Key8, Keys::Key9,
    Keys::Escape, Keys::Space, Keys::Enter, Keys::Tab, Keys::Backspace,
    Keys::LeftShift, Keys::RightShift, Keys::LeftControl,
    Keys::RightControl, Keys::LeftAlt, Keys::RightAlt,
    Keys::Up, Keys::Down, Keys::Left, Keys::Right,
    Keys::Insert, Keys::Delete, Keys::Home, Keys::End,
    Keys::PageUp, Keys::PageDown,
    Keys::F1, Keys::F2, Keys::F3, Keys::F4, Keys::F5, Keys::F6,
    Keys::F7, Keys::F8, Keys::F9, Keys::F10, Keys::F11, Keys::F12,
];
