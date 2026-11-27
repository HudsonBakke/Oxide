//! Validated world-space rectangles used as camera display surfaces.
use super::Vec3;
use crate::{OxideError, OxideResult};

/// A world-space rectangle with image corners and an explicit front-face normal.
///
/// Corners are ordered top-left, top-right, bottom-right, bottom-left as seen in
/// the image. The normal chooses the visible side independently of that order.
/// Construction normalizes the normal and rejects nonrectangular/degenerate data.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect3D {
    top_left: Vec3,
    top_right: Vec3,
    bottom_right: Vec3,
    bottom_left: Vec3,
    normal: Vec3,
}
impl Rect3D {
    /// Creates a rectangle from four finite world-space corners and a finite,
    /// nonzero perpendicular normal. Edges must exceed 0.00001 world units;
    /// rectangle/perpendicularity checks use a relative tolerance of 0.0001.
    pub fn new(top_left: Vec3, top_right: Vec3, bottom_right: Vec3, bottom_left: Vec3, normal: Vec3) -> OxideResult<Self> {
        let invalid = |message| OxideError::InvalidArgument { argument: "rectangle", message };
        for point in [top_left, top_right, bottom_right, bottom_left, normal] {
            if [point.x, point.y, point.z].iter().any(|v| !v.is_finite() || v.abs() > 1.0e12) {
                return Err(invalid("corners and normal must be finite and within +/-1e12"));
            }
        }
        let right = top_right - top_left;
        let down = bottom_left - top_left;
        let width = right.length(); let height = down.length();
        if width <= 1.0e-5 || height <= 1.0e-5 || normal.length() <= 1.0e-6 {
            return Err(invalid("rectangle edges and normal must be nonzero"));
        }
        if right.dot(&down).abs() > 1.0e-4 * width * height ||
            (bottom_right - (top_right + down)).length() > 1.0e-4 * width.max(height) {
            return Err(invalid("corners must form a rectangle in top-left, top-right, bottom-right, bottom-left order"));
        }
        let normal = normal.normalized();
        if normal.dot(&(right / width)).abs() > 1.0e-4 || normal.dot(&(down / height)).abs() > 1.0e-4 {
            return Err(invalid("normal must be perpendicular to both rectangle edges"));
        }
        Ok(Self { top_left, top_right, bottom_right, bottom_left, normal })
    }
    /// Image top-left corner in world coordinates.
    pub fn top_left(&self) -> Vec3 { self.top_left }
    /// Image top-right corner in world coordinates.
    pub fn top_right(&self) -> Vec3 { self.top_right }
    /// Image bottom-right corner in world coordinates.
    pub fn bottom_right(&self) -> Vec3 { self.bottom_right }
    /// Image bottom-left corner in world coordinates.
    pub fn bottom_left(&self) -> Vec3 { self.bottom_left }
    /// Unit vector pointing toward the visible front side.
    pub fn normal(&self) -> Vec3 { self.normal }
    /// Horizontal extent in world units.
    pub fn width(&self) -> f32 { (self.top_right - self.top_left).length() }
    /// Vertical extent in world units.
    pub fn height(&self) -> f32 { (self.bottom_left - self.top_left).length() }
    /// World-space width divided by height.
    pub fn aspect_ratio(&self) -> f32 { self.width() / self.height() }
    pub(crate) fn validated(&self) -> OxideResult<Self> {
        Self::new(self.top_left, self.top_right, self.bottom_right, self.bottom_left, self.normal)
    }
    pub(crate) fn corners(&self) -> [Vec3;4] { [self.top_left, self.top_right, self.bottom_right, self.bottom_left] }
}
