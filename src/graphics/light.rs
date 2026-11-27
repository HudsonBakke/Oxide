use crate::{
    OxideError, OxideResult,
    math::{Angle, Vec3},
};

use super::{Color, ShadowSettings};

const LIGHT_EPSILON: f32 = 1.0e-6;

/// A light emitted outward in all directions from one point.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointLight {
    position: Vec3,
    color: Color,
    intensity: f32,
    casts_shadows: u32,
    shadow_settings: ShadowSettings,
}

impl PointLight {
    /// Creates a point light; intensity must not be negative.
    pub fn new(position: Vec3, color: Color, intensity: f32) -> OxideResult<Self> {
        if intensity < 0.0 {
            return Err(OxideError::InvalidArgument {
                argument: "intensity",
                message: "light intensity cannot be negative",
            });
        }

        Ok(Self {
            position,
            color,
            intensity,
            casts_shadows: 0,
            shadow_settings: ShadowSettings::default(),
        })
    }

    /// World-space position.
    pub fn position(&self) -> Vec3 {
        self.position
    }

    /// Sets the world-space position.
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    /// RGB light color; alpha is not used for lighting.
    pub fn color(&self) -> Color {
        self.color
    }

    /// Sets the color.
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    /// Non-negative light intensity.
    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    /// Sets non-negative light intensity.
    pub fn set_intensity(&mut self, intensity: f32) -> OxideResult<()> {
        if intensity < 0.0 {
            return Err(OxideError::InvalidArgument {
                argument: "intensity",
                message: "light intensity cannot be negative",
            });
        }

        self.intensity = intensity;

        Ok(())
    }
    /// Whether this light casts shadows; defaults to false.
    pub fn casts_shadows(&self) -> bool { self.casts_shadows != 0 }

    /// Enables or disables this light's shadows using its existing default/custom settings.
    pub fn set_casts_shadows(&mut self, enabled: bool) { self.casts_shadows = enabled as u32; }

    /// Optional quality and coverage settings; defaults are supplied by the constructor.
    pub fn shadow_settings(&self) -> &ShadowSettings { &self.shadow_settings }

    /// Overrides shadow quality/coverage after checking all setting ranges.
    pub fn set_shadow_settings(&mut self, settings: ShadowSettings) -> OxideResult<()> {
        settings.validate()?;
        self.shadow_settings = settings;
        Ok(())
    }

}

/// A light emitted from one point within a directional cone.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpotLight {
    position: Vec3,
    direction: Vec3,
    angle: Angle,

    color: Color,
    intensity: f32,
    casts_shadows: u32,
    shadow_settings: ShadowSettings,
}

impl SpotLight {
    /// Creates a cone light; requires a nonzero direction, non-negative intensity, and half-angle in (0, pi) radians.
    pub fn new(
        position: Vec3,
        direction: Vec3,
        angle: Angle,
        color: Color,
        intensity: f32,
    ) -> OxideResult<Self> {
        if direction.length() <= LIGHT_EPSILON {
            return Err(OxideError::InvalidArgument {
                argument: "direction",
                message: "spot light direction cannot be zero",
            });
        }

        let radians = angle.as_radians();

        if radians <= 0.0 || radians >= std::f32::consts::PI {
            return Err(OxideError::InvalidArgument {
                argument: "angle",
                message: "spot light angle must be between 0 and pi radians",
            });
        }

        if intensity < 0.0 {
            return Err(OxideError::InvalidArgument {
                argument: "intensity",
                message: "light intensity cannot be negative",
            });
        }

        Ok(Self {
            position,
            direction: direction.normalized(),
            angle,
            color,
            intensity,
            casts_shadows: 0,
            shadow_settings: ShadowSettings::default(),
        })
    }

    /// World-space position.
    pub fn position(&self) -> Vec3 {
        self.position
    }

    /// Sets the world-space position.
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    /// Normalized world-space direction in which the light rays travel.
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    /// Sets and normalizes the direction.
    pub fn set_direction(&mut self, direction: Vec3) -> OxideResult<()> {
        if direction.length() <= LIGHT_EPSILON {
            return Err(OxideError::InvalidArgument {
                argument: "direction",
                message: "spot light direction cannot be zero",
            });
        }

        self.direction = direction.normalized();

        Ok(())
    }

    /// Spotlight cone angle.
    pub fn angle(&self) -> Angle {
        self.angle
    }

    /// Sets the spotlight cone angle.
    pub fn set_angle(&mut self, angle: Angle) -> OxideResult<()> {
        let radians = angle.as_radians();

        if radians <= 0.0 || radians >= std::f32::consts::PI {
            return Err(OxideError::InvalidArgument {
                argument: "angle",
                message: "spot light angle must be between 0 and pi radians",
            });
        }

        self.angle = angle;

        Ok(())
    }

    /// RGB light color; alpha is not used for lighting.
    pub fn color(&self) -> Color {
        self.color
    }

    /// Sets the color.
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    /// Non-negative light intensity.
    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    /// Sets non-negative light intensity.
    pub fn set_intensity(&mut self, intensity: f32) -> OxideResult<()> {
        if intensity < 0.0 {
            return Err(OxideError::InvalidArgument {
                argument: "intensity",
                message: "light intensity cannot be negative",
            });
        }

        self.intensity = intensity;

        Ok(())
    }
    /// Whether this light casts shadows; defaults to false.
    pub fn casts_shadows(&self) -> bool { self.casts_shadows != 0 }

    /// Enables or disables this light's shadows using its existing default/custom settings.
    pub fn set_casts_shadows(&mut self, enabled: bool) { self.casts_shadows = enabled as u32; }

    /// Optional quality and coverage settings; defaults are supplied by the constructor.
    pub fn shadow_settings(&self) -> &ShadowSettings { &self.shadow_settings }

    /// Overrides shadow quality/coverage after checking all setting ranges.
    pub fn set_shadow_settings(&mut self, settings: ShadowSettings) -> OxideResult<()> {
        settings.validate()?;
        self.shadow_settings = settings;
        Ok(())
    }

}

/// A light whose rays travel in one uniform direction.
///
/// Directional lights have no position or distance attenuation.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectionalLight {
    direction: Vec3,
    color: Color,
    intensity: f32,
    casts_shadows: u32,
    shadow_settings: ShadowSettings,
}

impl DirectionalLight {
    /// Creates a directional light; requires a nonzero direction and non-negative intensity.
    pub fn new(direction: Vec3, color: Color, intensity: f32) -> OxideResult<Self> {
        if direction.length() <= LIGHT_EPSILON {
            return Err(OxideError::InvalidArgument {
                argument: "direction",
                message: "directional light direction cannot be zero",
            });
        }

        if intensity < 0.0 {
            return Err(OxideError::InvalidArgument {
                argument: "intensity",
                message: "light intensity cannot be negative",
            });
        }

        Ok(Self {
            direction: direction.normalized(),
            color,
            intensity,
            casts_shadows: 0,
            shadow_settings: ShadowSettings::default(),
        })
    }

    /// Normalized world-space direction in which the light rays travel.
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    /// Sets and normalizes the direction.
    pub fn set_direction(&mut self, direction: Vec3) -> OxideResult<()> {
        if direction.length() <= LIGHT_EPSILON {
            return Err(OxideError::InvalidArgument {
                argument: "direction",
                message: "directional light direction cannot be zero",
            });
        }

        self.direction = direction.normalized();

        Ok(())
    }

    /// RGB light color; alpha is not used for lighting.
    pub fn color(&self) -> Color {
        self.color
    }

    /// Sets the color.
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    /// Non-negative light intensity.
    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    /// Sets non-negative light intensity.
    pub fn set_intensity(&mut self, intensity: f32) -> OxideResult<()> {
        if intensity < 0.0 {
            return Err(OxideError::InvalidArgument {
                argument: "intensity",
                message: "light intensity cannot be negative",
            });
        }

        self.intensity = intensity;

        Ok(())
    }
    /// Whether this light casts shadows; defaults to false.
    pub fn casts_shadows(&self) -> bool { self.casts_shadows != 0 }

    /// Enables or disables this light's shadows using its existing default/custom settings.
    pub fn set_casts_shadows(&mut self, enabled: bool) { self.casts_shadows = enabled as u32; }

    /// Optional quality and coverage settings; defaults are supplied by the constructor.
    pub fn shadow_settings(&self) -> &ShadowSettings { &self.shadow_settings }

    /// Overrides shadow quality/coverage after checking all setting ranges.
    pub fn set_shadow_settings(&mut self, settings: ShadowSettings) -> OxideResult<()> {
        settings.validate()?;
        self.shadow_settings = settings;
        Ok(())
    }

}

// ---------------------------------------------------------
// Sealed LightSource implementation
// ---------------------------------------------------------

/// Sealed trait implemented by point, spot, and directional lights accepted by Graphics.
pub trait LightSource: private::LightSourceInternal + Copy {
    /// RGB light color; alpha is unused.
    fn color(&self) -> Color;
    /// Changes the light color.
    fn set_color(&mut self, color: Color);
    /// Non-negative intensity.
    fn intensity(&self) -> f32;
    /// Changes non-negative intensity.
    fn set_intensity(&mut self, intensity: f32) -> OxideResult<()>;
    /// Whether this light casts shadows.
    fn casts_shadows(&self) -> bool;
    /// Enables shadows without requiring custom settings.
    fn set_casts_shadows(&mut self, enabled: bool);
    /// Optional shadow quality and coverage settings.
    fn shadow_settings(&self) -> &ShadowSettings;
    /// Sets validated shadow quality and coverage.
    fn set_shadow_settings(&mut self, settings: ShadowSettings) -> OxideResult<()>;
}

pub(crate) use private::LightSourceInternal;

mod private {
    /// Internal dispatch from a sealed light type into the graphics subsystem.
    pub trait LightSourceInternal: Copy {
        /// Adds this concrete light to the active graphics lighting state.
        fn add_to_graphics(&self, graphics: &mut crate::graphics::Graphics);
    }
}

impl LightSourceInternal for PointLight {
    fn add_to_graphics(&self, graphics: &mut super::Graphics) {
        graphics.add_point_light_internal(self);
    }
}

impl LightSource for PointLight {
    fn color(&self) -> Color { self.color() }
    fn set_color(&mut self, color: Color) { self.set_color(color); }
    fn intensity(&self) -> f32 { self.intensity() }
    fn set_intensity(&mut self, value: f32) -> OxideResult<()> { self.set_intensity(value) }
    fn casts_shadows(&self) -> bool { self.casts_shadows() }
    fn set_casts_shadows(&mut self, enabled: bool) { self.set_casts_shadows(enabled); }
    fn shadow_settings(&self) -> &ShadowSettings { self.shadow_settings() }
    fn set_shadow_settings(&mut self, settings: ShadowSettings) -> OxideResult<()> { self.set_shadow_settings(settings) }
}

impl LightSourceInternal for SpotLight {
    fn add_to_graphics(&self, graphics: &mut super::Graphics) {
        graphics.add_spot_light_internal(self);
    }
}

impl LightSource for SpotLight {
    fn color(&self) -> Color { self.color() }
    fn set_color(&mut self, color: Color) { self.set_color(color); }
    fn intensity(&self) -> f32 { self.intensity() }
    fn set_intensity(&mut self, value: f32) -> OxideResult<()> { self.set_intensity(value) }
    fn casts_shadows(&self) -> bool { self.casts_shadows() }
    fn set_casts_shadows(&mut self, enabled: bool) { self.set_casts_shadows(enabled); }
    fn shadow_settings(&self) -> &ShadowSettings { self.shadow_settings() }
    fn set_shadow_settings(&mut self, settings: ShadowSettings) -> OxideResult<()> { self.set_shadow_settings(settings) }
}

impl LightSourceInternal for DirectionalLight {
    fn add_to_graphics(&self, graphics: &mut super::Graphics) {
        graphics.add_directional_light_internal(self);
    }
}

impl LightSource for DirectionalLight {
    fn color(&self) -> Color { self.color() }
    fn set_color(&mut self, color: Color) { self.set_color(color); }
    fn intensity(&self) -> f32 { self.intensity() }
    fn set_intensity(&mut self, value: f32) -> OxideResult<()> { self.set_intensity(value) }
    fn casts_shadows(&self) -> bool { self.casts_shadows() }
    fn set_casts_shadows(&mut self, enabled: bool) { self.set_casts_shadows(enabled); }
    fn shadow_settings(&self) -> &ShadowSettings { self.shadow_settings() }
    fn set_shadow_settings(&mut self, settings: ShadowSettings) -> OxideResult<()> { self.set_shadow_settings(settings) }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn shared_contract<L: LightSource>(mut light: L) {
        assert!(!light.casts_shadows());
        light.set_casts_shadows(true);
        assert!(light.casts_shadows());
        assert_eq!(*light.shadow_settings(), ShadowSettings::default());
        let custom = ShadowSettings { resolution: 256, ..Default::default() };
        light.set_shadow_settings(custom).unwrap();
        assert_eq!(*light.shadow_settings(), custom);
        assert!(light.set_shadow_settings(ShadowSettings { far_plane: 0.0, ..custom }).is_err());
        assert_eq!(*light.shadow_settings(), custom);
    }
    #[test]
    fn light_types_share_shadow_defaults_and_configuration() {
        shared_contract(PointLight::new(Vec3::default(), Color::WHITE, 1.0).unwrap());
        shared_contract(DirectionalLight::new(Vec3::new(0.0, -1.0, 0.0), Color::WHITE, 1.0).unwrap());
        shared_contract(SpotLight::new(Vec3::default(), Vec3::new(0.0, -1.0, 0.0), Angle::degrees(30.0), Color::WHITE, 1.0).unwrap());
        assert_eq!(std::mem::size_of::<PointLight>(), 76);
        assert_eq!(std::mem::size_of::<DirectionalLight>(), 76);
        assert_eq!(std::mem::size_of::<SpotLight>(), 92);
    }
}
