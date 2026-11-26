//! Pixel rectangles for camera viewports.

/// A rectangular region defined by its position and dimensions.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Rect {
    /// Left edge in physical pixels.
    pub x: f32,
    /// Top edge in physical pixels.
    pub y: f32,
    /// Horizontal extent in physical pixels.
    pub width: f32,
    /// Vertical extent in physical pixels.
    pub height: f32,
}

impl Rect {
    /// Creates a pixel rectangle from top-left coordinates and width/height extents.
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Width divided by height; height must be nonzero for a finite result.
    pub fn aspect_ratio(&self) -> f32 {
        self.width / self.height
    }
}
