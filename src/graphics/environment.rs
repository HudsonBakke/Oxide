use super::Color;

/// Analytic environment lighting used for ambient diffuse illumination and
/// specular reflections.
///
/// This first environment representation is intentionally procedural. It gives
/// the PBR renderer a full directional light field without requiring an HDR
/// cubemap asset or an offline prefiltering pipeline.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Environment {
    pub sky_color: Color,
    pub horizon_color: Color,
    pub ground_color: Color,
    pub intensity: f32,
}

impl Environment {
    pub fn new(
        sky_color: Color,
        horizon_color: Color,
        ground_color: Color,
        intensity: f32,
    ) -> crate::OxideResult<Self> {
        if !intensity.is_finite() || intensity < 0.0 {
            return Err(crate::OxideError::InvalidArgument {
                argument: "intensity",
                message: "environment intensity must be finite and non-negative",
            });
        }

        Ok(Self {
            sky_color,
            horizon_color,
            ground_color,
            intensity,
        })
    }

    /// A neutral outdoor-like environment suitable as a general PBR default.
    pub fn daylight() -> Self {
        Self {
            sky_color: Color::rgb(0.28, 0.48, 0.90),
            horizon_color: Color::rgb(0.72, 0.78, 0.88),
            ground_color: Color::rgb(0.16, 0.13, 0.10),
            intensity: 0.65,
        }
    }
}
