/// A rectangular region defined by its position and dimensions.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
/// Rectangle expressed by position and size, commonly used for camera viewports.
pub struct Rect {
    /// Public Oxide x API.
    pub x: f32,
    /// Public Oxide y API.
    pub y: f32,
    /// Public Oxide width API.
    pub width: f32,
    /// Public Oxide height API.
    pub height: f32,
}

impl Rect {
    /// Public Oxide fn API.
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Returns this rectangle's width-to-height ratio.
    pub fn aspect_ratio(&self) -> f32 {
        self.width / self.height
    }
}
