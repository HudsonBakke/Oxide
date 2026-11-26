use super::Color;

/// Explicit scene environment parameters.
///
/// Environment contains rendering inputs, not semantic presets. The caller
/// decides what values represent daylight, night, a studio, or anything else.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Environment {
    /// Constant ambient irradiance applied to non-emissive surfaces.
    pub ambient_color: Color,
    pub ambient_intensity: f32,

    /// Analytic directional environment used for reflections and additional
    /// hemispherical irradiance.
    pub sky_color: Color,
    pub horizon_color: Color,
    pub ground_color: Color,
    pub environment_intensity: f32,

    /// Clear color shown where no geometry is rendered.
    pub background_color: Color,
}

impl Environment {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        ambient_color: Color,
        ambient_intensity: f32,
        sky_color: Color,
        horizon_color: Color,
        ground_color: Color,
        environment_intensity: f32,
        background_color: Color,
    ) -> crate::OxideResult<Self> {
        validate_intensity("ambient_intensity", ambient_intensity)?;
        validate_intensity("environment_intensity", environment_intensity)?;

        Ok(Self {
            ambient_color,
            ambient_intensity,
            sky_color,
            horizon_color,
            ground_color,
            environment_intensity,
            background_color,
        })
    }
}

fn validate_intensity(
    argument: &'static str,
    value: f32,
) -> crate::OxideResult<()> {
    if !value.is_finite() || value < 0.0 {
        return Err(crate::OxideError::InvalidArgument {
            argument,
            message: "environment intensity must be finite and non-negative",
        });
    }

    Ok(())
}
