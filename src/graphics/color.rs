/// A linear RGBA color.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Linear RGBA color with floating-point channels.
pub struct Color {
    /// Public Oxide r API.
    pub r: f32,
    /// Public Oxide g API.
    pub g: f32,
    /// Public Oxide b API.
    pub b: f32,
    /// Public Oxide a API.
    pub a: f32,
}

impl Color {
    /// Public Oxide BLACK API.
    pub const BLACK: Self = Self::rgb(0.0, 0.0, 0.0);
    /// Public Oxide WHITE API.
    pub const WHITE: Self = Self::rgb(1.0, 1.0, 1.0);

    /// Public Oxide RED API.
    pub const RED: Self = Self::rgb(1.0, 0.0, 0.0);
    /// Public Oxide GREEN API.
    pub const GREEN: Self = Self::rgb(0.0, 1.0, 0.0);
    /// Public Oxide BLUE API.
    pub const BLUE: Self = Self::rgb(0.0, 0.0, 1.0);

    /// Public Oxide YELLOW API.
    pub const YELLOW: Self = Self::rgb(1.0, 1.0, 0.0);

    /// Public Oxide fn API.
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Public Oxide fn API.
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}
