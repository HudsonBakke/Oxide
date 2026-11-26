use std::collections::HashSet;
use std::sync::{Mutex, MutexGuard, OnceLock};

use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, MouseButton as WinitMouseButton, MouseScrollDelta};

use crate::math::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
    Other(u16),
}

#[derive(Debug, Clone)]
pub struct MouseState {
    current: HashSet<MouseButton>,
    previous: HashSet<MouseButton>,
    position: Vec2,
    delta: Vec2,
    scroll_delta: Vec2,
}

impl Default for MouseState {
    fn default() -> Self {
        Self {
            current: HashSet::new(),
            previous: HashSet::new(),
            position: Vec2::default(),
            delta: Vec2::default(),
            scroll_delta: Vec2::default(),
        }
    }
}

impl MouseState {
    pub fn is_button_down(&self, button: MouseButton) -> bool {
        self.current.contains(&button)
    }

    pub fn is_button_up(&self, button: MouseButton) -> bool {
        !self.is_button_down(button)
    }

    pub fn is_button_pressed(&self, button: MouseButton) -> bool {
        self.current.contains(&button) && !self.previous.contains(&button)
    }

    pub fn is_button_released(&self, button: MouseButton) -> bool {
        !self.current.contains(&button) && self.previous.contains(&button)
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn delta(&self) -> Vec2 {
        self.delta
    }

    pub fn scroll_delta(&self) -> Vec2 {
        self.scroll_delta
    }
}

pub struct Mouse;

impl Mouse {
    pub fn get_state() -> MouseState {
        lock_mouse().clone()
    }
}

static MOUSE: OnceLock<Mutex<MouseState>> = OnceLock::new();

fn lock_mouse() -> MutexGuard<'static, MouseState> {
    MOUSE
        .get_or_init(|| Mutex::new(MouseState::default()))
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

pub(crate) fn handle_mouse_input(button: WinitMouseButton, state: ElementState) {
    let button = map_button(button);
    let mut mouse = lock_mouse();

    match state {
        ElementState::Pressed => {
            mouse.current.insert(button);
        }
        ElementState::Released => {
            mouse.current.remove(&button);
        }
    }
}

pub(crate) fn handle_cursor_moved(position: PhysicalPosition<f64>) {
    let mut mouse = lock_mouse();
    let next = Vec2::new(position.x as f32, position.y as f32);

    mouse.delta = mouse.delta + (next - mouse.position);
    mouse.position = next;
}

pub(crate) fn handle_mouse_wheel(delta: MouseScrollDelta) {
    let mut mouse = lock_mouse();

    let delta = match delta {
        MouseScrollDelta::LineDelta(x, y) => Vec2::new(x, y),
        MouseScrollDelta::PixelDelta(position) => Vec2::new(position.x as f32, position.y as f32),
    };

    mouse.scroll_delta = mouse.scroll_delta + delta;
}

pub(crate) fn advance_mouse_frame() {
    let mut mouse = lock_mouse();
    mouse.previous = mouse.current.clone();
    mouse.delta = Vec2::default();
    mouse.scroll_delta = Vec2::default();
}

fn map_button(button: WinitMouseButton) -> MouseButton {
    match button {
        WinitMouseButton::Left => MouseButton::Left,
        WinitMouseButton::Right => MouseButton::Right,
        WinitMouseButton::Middle => MouseButton::Middle,
        WinitMouseButton::Back => MouseButton::Back,
        WinitMouseButton::Forward => MouseButton::Forward,
        WinitMouseButton::Other(value) => MouseButton::Other(value),
    }
}
