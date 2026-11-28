use crate::{
    math::{Angle, Rect, Vec2},
    OxideError, OxideResult,
};

use super::Color;

/// Per-draw screen-space sprite controls. Values are captured at submission.
///
/// Coordinates, source regions, origin and clipping use physical pixels. Origin
/// is relative to the target's top-left; it changes the rotation pivot, without
/// changing an unrotated target. Flips affect UVs rather than target geometry.
#[derive(Debug, Clone, Copy)]
pub struct Draw2DOptions {
    /// Linear RGBA multiplier in `0..=1`, initially opaque white.
    pub tint: Color,
    /// Source region in texture pixels, or None for the entire texture.
    pub source: Option<Rect>,
    /// Clockwise screen-space rotation, initially zero.
    pub rotation: Angle,
    /// Rotation pivot relative to the target top-left, in target pixels.
    pub origin: Vec2,
    /// Optional screen-space clipping rectangle; None clips to the window.
    pub clip: Option<Rect>,
    /// Mirrors the sampled image horizontally.
    pub flip_horizontally: bool,
    /// Mirrors the sampled image vertically.
    pub flip_vertically: bool,
}

impl Default for Draw2DOptions {
    fn default() -> Self {
        Self {
            tint: Color::WHITE,
            source: None,
            rotation: Angle::radians(0.0),
            origin: Vec2::default(),
            clip: None,
            flip_horizontally: false,
            flip_vertically: false,
        }
    }
}

impl Draw2DOptions {
    pub(crate) fn validate(&self) -> OxideResult<()> {
        if [self.tint.r, self.tint.g, self.tint.b, self.tint.a]
            .iter()
            .any(|v| !v.is_finite() || !(0.0..=1.0).contains(v))
        {
            return Err(invalid("2D tint channels must be finite and in [0,1]"));
        }
        if !self.rotation.as_radians().is_finite()
            || [self.origin.x, self.origin.y]
                .iter()
                .any(|v| !v.is_finite() || v.abs() > 1.0e7)
        {
            return Err(invalid(
                "rotation must be finite; origin must be finite and within +/-1e7 pixels",
            ));
        }
        if let Some(source) = self.source {
            validate_rect(source)?;
            if source.width <= 0.0 || source.height <= 0.0 || source.x < 0.0 || source.y < 0.0 {
                return Err(invalid(
                    "source region needs nonnegative coordinates and positive dimensions",
                ));
            }
        }
        if let Some(clip) = self.clip {
            validate_rect(clip)?;
        }
        Ok(())
    }
}

/// Font selection and drawing controls for a UTF-8 menu label.
///
/// Font families are installed Windows fonts; Segoe UI at 24 physical pixels is
/// the default. DirectWrite supplies shaping and fallback. No font file is needed.
#[derive(Debug, Clone)]
pub struct TextOptions {
    /// Installed font family, initially Segoe UI.
    pub font_family: String,
    /// Font em size in physical pixels, finite and in `1..=256`.
    pub font_size: f32,
    /// Tint, rotation, origin, clipping and flipping controls for the label.
    pub draw: Draw2DOptions,
}

impl Default for TextOptions {
    fn default() -> Self {
        Self {
            font_family: "Segoe UI".into(),
            font_size: 24.0,
            draw: Draw2DOptions::default(),
        }
    }
}

impl TextOptions {
    pub(crate) fn validate(&self, text: &str) -> OxideResult<()> {
        self.draw.validate()?;
        if self.draw.source.is_some() {
            return Err(invalid("source cropping is unsupported for text"));
        }
        if !self.font_size.is_finite()
            || !(1.0..=256.0).contains(&self.font_size)
            || self.font_family.is_empty()
            || self.font_family.len() > 256
            || self.font_family.contains('\0')
            || text.len() > 65536
            || text.contains('\0')
        {
            return Err(invalid(
                "text/font must contain no NUL; text is limited to 65536 UTF-8 bytes, family to 256 bytes, and font size to 1..256 pixels",
            ));
        }
        Ok(())
    }
}

pub(crate) fn validate_rect(rect: Rect) -> OxideResult<()> {
    if [rect.x, rect.y, rect.width, rect.height]
        .iter()
        .any(|v| !v.is_finite() || v.abs() > 1.0e7)
        || rect.width < 0.0
        || rect.height < 0.0
    {
        return Err(invalid(
            "rectangles need finite coordinates within +/-1e7 and nonnegative dimensions",
        ));
    }
    Ok(())
}

fn invalid(message: &'static str) -> OxideError {
    OxideError::InvalidArgument { argument: "draw_2d", message }
}
