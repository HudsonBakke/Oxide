use crate::{
    OxideError, OxideResult,
    math::{Angle, Vec3},
};

use super::Color;

const LIGHT_EPSILON: f32 = 1.0e-6;

/// A light emitted outward in all directions from one point.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointLight {
    position: Vec3,
    color: Color,
    intensity: f32,
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
pub struct SpotLight {
    position: Vec3,
    direction: Vec3,
    angle: Angle,

    color: Color,
    intensity: f32,
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
pub struct DirectionalLight {
    direction: Vec3,
    color: Color,
    intensity: f32,
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

/// Sealed trait implemented by point, spot, and directional lights accepted by Graphics.
pub trait LightSource: private::LightSourceInternal + Copy {}

pub(crate) use private::LightSourceInternal;

mod private {
    /// Internal dispatch from a sealed light type into the graphics subsystem.
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
