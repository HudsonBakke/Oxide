use crate::{
    OxideError, OxideResult,
    math::{Angle, Vec3},
};

use super::Color;

const LIGHT_EPSILON: f32 = 1.0e-6;

/// A light emitted outward in all directions from one point.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Omnidirectional light emitted from a point in world space.
pub struct PointLight {
    position: Vec3,
    color: Color,
    intensity: f32,
}

impl PointLight {
    /// Public Oxide new API.
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

    /// Light or material color.
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
}

/// A light emitted from one point within a directional cone.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Cone-shaped light with position, direction, angle, color, and intensity.
pub struct SpotLight {
    position: Vec3,
    direction: Vec3,
    angle: Angle,

    color: Color,
    intensity: f32,
}

impl SpotLight {
    /// Public Oxide new API.
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

    /// Normalized light or mesh direction.
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

    /// Light or material color.
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
}

/// A light whose rays travel in one uniform direction.
///
/// Directional lights have no position or distance attenuation.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
/// Infinitely distant light whose rays share one world-space direction.
pub struct DirectionalLight {
    direction: Vec3,
    color: Color,
    intensity: f32,
}

impl DirectionalLight {
    /// Public Oxide new API.
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
        })
    }

    /// Normalized light or mesh direction.
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

    /// Light or material color.
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
}

// ---------------------------------------------------------
// Sealed LightSource implementation
// ---------------------------------------------------------

/// Public Oxide LightSource API.
pub trait LightSource: private::LightSourceInternal + Copy {}

pub(crate) use private::LightSourceInternal;

mod private {
    /// Public Oxide LightSourceInternal API.
    pub trait LightSourceInternal: Copy {
        fn add_to_graphics(&self, graphics: &mut crate::graphics::Graphics);
    }
}

impl LightSourceInternal for PointLight {
    fn add_to_graphics(&self, graphics: &mut super::Graphics) {
        graphics.add_point_light_internal(self);
    }
}

impl LightSource for PointLight {}

impl LightSourceInternal for SpotLight {
    fn add_to_graphics(&self, graphics: &mut super::Graphics) {
        graphics.add_spot_light_internal(self);
    }
}

impl LightSource for SpotLight {}

impl LightSourceInternal for DirectionalLight {
    fn add_to_graphics(&self, graphics: &mut super::Graphics) {
        graphics.add_directional_light_internal(self);
    }
}

impl LightSource for DirectionalLight {}
