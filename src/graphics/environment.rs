use super::{Color, Texture};

/// Explicit scene environment parameters.
///
/// Environment contains rendering inputs, not semantic presets. Lighting and
/// visible background textures are independent and both use equirectangular
/// projection.
#[derive(Debug, Clone)]
/// Explicit ambient, image-based-lighting, and background settings for a scene.
pub struct Environment {
    /// Constant ambient-light color.
    pub ambient_color: Color,
    /// Non-negative ambient-light multiplier.
    pub ambient_intensity: f32,

    /// Optional equirectangular texture used for image-based lighting.
    pub environment_texture: Option<Texture>,
    /// Non-negative IBL multiplier.
    pub environment_intensity: f32,

    /// Optional equirectangular texture used only as the visible background.
    pub background_texture: Option<Texture>,
    /// Fallback background color.
    pub background_color: Color,
}

impl Environment {
    /// Public Oxide new API.
    pub fn new(
        ambient_color: Color,
        ambient_intensity: f32,
        environment_texture: Option<Texture>,
        environment_intensity: f32,
        background_texture: Option<Texture>,
        background_color: Color,
    ) -> crate::OxideResult<Self> {
        validate_intensity("ambient_intensity", ambient_intensity)?;
        validate_intensity("environment_intensity", environment_intensity)?;
        Ok(Self {
            ambient_color,
            ambient_intensity,
            environment_texture,
            environment_intensity,
            background_texture,
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
