/// A rectangular region defined by its position and dimensions.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rect {
    pub const fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    ) -> Self {
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