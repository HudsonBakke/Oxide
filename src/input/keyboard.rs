use std::collections::HashSet;
use std::sync::{Mutex, MutexGuard, OnceLock};

use winit::event::ElementState;
use winit::keyboard::{
    KeyCode,
    PhysicalKey,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keys {
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    Key0, Key1, Key2, Key3, Key4,
    Key5, Key6, Key7, Key8, Key9,
    Escape, Space, Enter, Tab, Backspace,
    LeftShift, RightShift,
    LeftControl, RightControl,
    LeftAlt, RightAlt,
    Up, Down, Left, Right,
    Insert, Delete, Home, End, PageUp, PageDown,
    F1, F2, F3, F4, F5, F6,
    F7, F8, F9, F10, F11, F12,
}

#[derive(Debug, Clone, Default)]
pub struct KeyboardState {
    current: HashSet<Keys>,
    previous: HashSet<Keys>,
}

impl KeyboardState {
    pub fn is_key_down(&self, key: Keys) -> bool {
        self.current.contains(&key)
    }

    pub fn is_key_up(&self, key: Keys) -> bool {
        !self.is_key_down(key)
    }

    pub fn is_key_pressed(&self, key: Keys) -> bool {
        self.current.contains(&key)
            && !self.previous.contains(&key)
    }

    pub fn is_key_released(&self, key: Keys) -> bool {
        !self.current.contains(&key)
            && self.previous.contains(&key)
    }
}

pub struct Keyboard;

impl Keyboard {
    pub fn get_state() -> KeyboardState {
        lock_keyboard().clone()
    }
}

static KEYBOARD: OnceLock<Mutex<KeyboardState>> =
    OnceLock::new();

fn lock_keyboard() -> MutexGuard<'static, KeyboardState> {
    KEYBOARD
        .get_or_init(|| Mutex::new(KeyboardState::default()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

pub(crate) fn handle_keyboard_input(
    physical_key: PhysicalKey,
    state: ElementState,
) {
    let PhysicalKey::Code(code) = physical_key else {
        return;
    };

    let Some(key) = map_key(code) else {
        return;
    };

    let mut keyboard = lock_keyboard();

    match state {
        ElementState::Pressed => {
            keyboard.current.insert(key);
        }
        ElementState::Released => {
            keyboard.current.remove(&key);
        }
    }
}

pub(crate) fn advance_keyboard_frame() {
    let mut keyboard = lock_keyboard();
    keyboard.previous = keyboard.current.clone();
}

fn map_key(code: KeyCode) -> Option<Keys> {
    Some(match code {
        KeyCode::KeyA => Keys::A,
        KeyCode::KeyB => Keys::B,
        KeyCode::KeyC => Keys::C,
        KeyCode::KeyD => Keys::D,
        KeyCode::KeyE => Keys::E,
        KeyCode::KeyF => Keys::F,
        KeyCode::KeyG => Keys::G,
        KeyCode::KeyH => Keys::H,
        KeyCode::KeyI => Keys::I,
        KeyCode::KeyJ => Keys::J,
        KeyCode::KeyK => Keys::K,
        KeyCode::KeyL => Keys::L,
        KeyCode::KeyM => Keys::M,
        KeyCode::KeyN => Keys::N,
        KeyCode::KeyO => Keys::O,
        KeyCode::KeyP => Keys::P,
        KeyCode::KeyQ => Keys::Q,
        KeyCode::KeyR => Keys::R,
        KeyCode::KeyS => Keys::S,
        KeyCode::KeyT => Keys::T,
        KeyCode::KeyU => Keys::U,
        KeyCode::KeyV => Keys::V,
        KeyCode::KeyW => Keys::W,
        KeyCode::KeyX => Keys::X,
        KeyCode::KeyY => Keys::Y,
        KeyCode::KeyZ => Keys::Z,
        KeyCode::Digit0 => Keys::Key0,
        KeyCode::Digit1 => Keys::Key1,
        KeyCode::Digit2 => Keys::Key2,
        KeyCode::Digit3 => Keys::Key3,
        KeyCode::Digit4 => Keys::Key4,
        KeyCode::Digit5 => Keys::Key5,
        KeyCode::Digit6 => Keys::Key6,
        KeyCode::Digit7 => Keys::Key7,
        KeyCode::Digit8 => Keys::Key8,
        KeyCode::Digit9 => Keys::Key9,
        KeyCode::Escape => Keys::Escape,
        KeyCode::Space => Keys::Space,
        KeyCode::Enter => Keys::Enter,
        KeyCode::Tab => Keys::Tab,
        KeyCode::Backspace => Keys::Backspace,
        KeyCode::ShiftLeft => Keys::LeftShift,
        KeyCode::ShiftRight => Keys::RightShift,
        KeyCode::ControlLeft => Keys::LeftControl,
        KeyCode::ControlRight => Keys::RightControl,
        KeyCode::AltLeft => Keys::LeftAlt,
        KeyCode::AltRight => Keys::RightAlt,
        KeyCode::ArrowUp => Keys::Up,
        KeyCode::ArrowDown => Keys::Down,
        KeyCode::ArrowLeft => Keys::Left,
        KeyCode::ArrowRight => Keys::Right,
        KeyCode::Insert => Keys::Insert,
        KeyCode::Delete => Keys::Delete,
        KeyCode::Home => Keys::Home,
        KeyCode::End => Keys::End,
        KeyCode::PageUp => Keys::PageUp,
        KeyCode::PageDown => Keys::PageDown,
        KeyCode::F1 => Keys::F1,
        KeyCode::F2 => Keys::F2,
        KeyCode::F3 => Keys::F3,
        KeyCode::F4 => Keys::F4,
        KeyCode::F5 => Keys::F5,
        KeyCode::F6 => Keys::F6,
        KeyCode::F7 => Keys::F7,
        KeyCode::F8 => Keys::F8,
        KeyCode::F9 => Keys::F9,
        KeyCode::F10 => Keys::F10,
        KeyCode::F11 => Keys::F11,
        KeyCode::F12 => Keys::F12,
        _ => return None,
    })
}
