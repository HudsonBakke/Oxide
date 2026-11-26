use super::{Color, Texture};

/// Explicit scene environment parameters.
///
/// Environment contains rendering inputs, not semantic presets. The caller
/// decides exactly what constitutes ambient light, reflections, and background.
#[derive(Debug, Clone)]
pub struct Environment {
    pub ambient_color: Color,
    pub ambient_intensity: f32,

    /// Optional equirectangular texture sampled for image-based lighting.
    pub environment_texture: Option<Texture>,
    pub environment_intensity: f32,

    /// Solid fallback background. A texture-backed visible sky will be added
    /// with the dedicated background pass rather than being conflated with IBL.
    pub background_color: Color,
}

impl Environment {
    pub fn new(
        ambient_color: Color,
        ambient_intensity: f32,
        environment_texture: Option<Texture>,
        environment_intensity: f32,
        background_color: Color,
    ) -> crate::OxideResult<Self> {
        validate_intensity("ambient_intensity", ambient_intensity)?;
        validate_intensity("environment_intensity", environment_intensity)?;

        Ok(Self {
            ambient_color,
            ambient_intensity,
            environment_texture,
            environment_intensity,
            background_color,
        })
    }
}

fn validate_intensity(argument: &'static str, value: f32) -> crate::OxideResult<()> {
    if !value.is_finite() || value < 0.0 {
        return Err(crate::OxideError::InvalidArgument {
            argument,
            message: "environment intensity must be finite and non-negative",
        });
    }
    Ok(())
}
