/// A linear RGBA color.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    /// Red component.
    pub r: f32,
    /// Green component.
    pub g: f32,
    /// Blue component.
    pub b: f32,
    /// Alpha component: zero is transparent and one is opaque.
    pub a: f32,
}

impl Color {
    /// Opaque black.
    pub const BLACK: Self = Self::rgb(0.0, 0.0, 0.0);
    /// Opaque white.
    pub const WHITE: Self = Self::rgb(1.0, 1.0, 1.0);

    /// Opaque red.
    pub const RED: Self = Self::rgb(1.0, 0.0, 0.0);
    /// Opaque green.
    pub const GREEN: Self = Self::rgb(0.0, 1.0, 0.0);
    /// Opaque blue.
    pub const BLUE: Self = Self::rgb(0.0, 0.0, 1.0);

    /// Opaque yellow.
    pub const YELLOW: Self = Self::rgb(1.0, 1.0, 0.0);

    /// Creates an RGB color with alpha set to one.
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Creates a color from red, green, blue, and alpha components without clamping.
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}
