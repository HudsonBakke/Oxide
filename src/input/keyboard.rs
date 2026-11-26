use std::collections::HashSet;
use std::sync::{Mutex, MutexGuard, OnceLock};

use winit::event::ElementState;
use winit::keyboard::{KeyCode, PhysicalKey};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Physical keyboard keys, independent of text input and keyboard layout.
pub enum Keys {
    /// Physical A key.
    A,
    /// Physical B key.
    B,
    /// Physical C key.
    C,
    /// Physical D key.
    D,
    /// Physical E key.
    E,
    /// Physical F key.
    F,
    /// Physical G key.
    G,
    /// Physical H key.
    H,
    /// Physical I key.
    I,
    /// Physical J key.
    J,
    /// Physical K key.
    K,
    /// Physical L key.
    L,
    /// Physical M key.
    M,
    /// Physical N key.
    N,
    /// Physical O key.
    O,
    /// Physical P key.
    P,
    /// Physical Q key.
    Q,
    /// Physical R key.
    R,
    /// Physical S key.
    S,
    /// Physical T key.
    T,
    /// Physical U key.
    U,
    /// Physical V key.
    V,
    /// Physical W key.
    W,
    /// Physical X key.
    X,
    /// Physical Y key.
    Y,
    /// Physical Z key.
    Z,
    /// Top-row digit 0 key.
    Key0,
    /// Top-row digit 1 key.
    Key1,
    /// Top-row digit 2 key.
    Key2,
    /// Top-row digit 3 key.
    Key3,
    /// Top-row digit 4 key.
    Key4,
    /// Top-row digit 5 key.
    Key5,
    /// Top-row digit 6 key.
    Key6,
    /// Top-row digit 7 key.
    Key7,
    /// Top-row digit 8 key.
    Key8,
    /// Top-row digit 9 key.
    Key9,
    /// Escape key.
    Escape,
    /// Space key.
    Space,
    /// Enter key.
    Enter,
    /// Tab key.
    Tab,
    /// Backspace key.
    Backspace,
    /// Left shift key.
    LeftShift,
    /// Right shift key.
    RightShift,
    /// Left control key.
    LeftControl,
    /// Right control key.
    RightControl,
    /// Left alt key.
    LeftAlt,
    /// Right alt key.
    RightAlt,
    /// Up key.
    Up,
    /// Down key.
    Down,
    /// Left key.
    Left,
    /// Right key.
    Right,
    /// Insert key.
    Insert,
    /// Delete key.
    Delete,
    /// Home key.
    Home,
    /// End key.
    End,
    /// Page up key.
    PageUp,
    /// Page down key.
    PageDown,
    /// F1 key.
    F1,
    /// F2 key.
    F2,
    /// F3 key.
    F3,
    /// F4 key.
    F4,
    /// F5 key.
    F5,
    /// F6 key.
    F6,
    /// F7 key.
    F7,
    /// F8 key.
    F8,
    /// F9 key.
    F9,
    /// F10 key.
    F10,
    /// F11 key.
    F11,
    /// F12 key.
    F12,
}

#[derive(Debug, Clone, Default)]
/// Snapshot of current and previous keyboard state used for level and edge queries.
pub struct KeyboardState {
    current: HashSet<Keys>,
    previous: HashSet<Keys>,
}

impl KeyboardState {
    /// Returns whether the key is held.
    pub fn is_key_down(&self, key: Keys) -> bool {
        self.current.contains(&key)
    }

    /// Returns whether the key is not held.
    pub fn is_key_up(&self, key: Keys) -> bool {
        !self.is_key_down(key)
    }

    /// Returns whether the key transitioned from up to down this frame.
    pub fn is_key_pressed(&self, key: Keys) -> bool {
        self.current.contains(&key) && !self.previous.contains(&key)
    }

    /// Returns whether the key transitioned from down to up this frame.
    pub fn is_key_released(&self, key: Keys) -> bool {
        !self.current.contains(&key) && self.previous.contains(&key)
    }

    pub(crate) fn was_key_down(&self, key: Keys) -> bool {
        self.previous.contains(&key)
    }
}

/// Entry point for retrieving the current keyboard snapshot.
pub struct Keyboard;

impl Keyboard {
    /// Returns the current input snapshot.
    pub fn get_state() -> KeyboardState {
        lock_keyboard().clone()
    }
}

static KEYBOARD: OnceLock<Mutex<KeyboardState>> = OnceLock::new();

fn lock_keyboard() -> MutexGuard<'static, KeyboardState> {
    KEYBOARD
        .get_or_init(|| Mutex::new(KeyboardState::default()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

pub(crate) fn handle_keyboard_input(physical_key: PhysicalKey, state: ElementState) {
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
