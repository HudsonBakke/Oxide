use crate::math::{
    Angle,
    Vec3,
};

use super::Color;

/// A light emitted outward in all directions from one point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointLight {
    position: Vec3,
    color: Color,
    intensity: f32,
}

impl PointLight {
    pub fn new(
        position: Vec3,
        color: Color,
        intensity: f32,
    ) -> Self {
        assert!(
            intensity >= 0.0,
            "light intensity cannot be negative"
        );

        Self {
            position,
            color,
            intensity,
        }
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    pub fn set_intensity(&mut self, intensity: f32) {
        assert!(
            intensity >= 0.0,
            "light intensity cannot be negative"
        );

        self.intensity = intensity;
    }
}

/// A light emitted from one point within a directional cone.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpotLight {
    position: Vec3,
    direction: Vec3,
    angle: Angle,

    color: Color,
    intensity: f32,
}

impl SpotLight {
    pub fn new(
        position: Vec3,
        direction: Vec3,
        angle: Angle,
        color: Color,
        intensity: f32,
    ) -> Self {
        assert!(
            direction.length() > 0.0,
            "spot light direction cannot be zero"
        );

        assert!(
            angle.as_radians() > 0.0
                && angle.as_radians() < std::f32::consts::PI,
            "spot light angle must be between 0 and pi radians"
        );

        assert!(
            intensity >= 0.0,
            "light intensity cannot be negative"
        );

        Self {
            position,
            direction: direction.normalized(),
            angle,
            color,
            intensity,
        }
    }

    pub fn position(&self) -> Vec3 {
        self.position
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn set_direction(&mut self, direction: Vec3) {
        assert!(
            direction.length() > 0.0,
            "spot light direction cannot be zero"
        );

        self.direction = direction.normalized();
    }

    pub fn angle(&self) -> Angle {
        self.angle
    }

    pub fn set_angle(&mut self, angle: Angle) {
        assert!(
            angle.as_radians() > 0.0
                && angle.as_radians() < std::f32::consts::PI,
            "spot light angle must be between 0 and pi radians"
        );

        self.angle = angle;
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    pub fn set_intensity(&mut self, intensity: f32) {
        assert!(
            intensity >= 0.0,
            "light intensity cannot be negative"
        );

        self.intensity = intensity;
    }
}

/// A light whose rays travel in one uniform direction.
///
/// Directional lights have no position or distance attenuation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectionalLight {
    direction: Vec3,
    color: Color,
    intensity: f32,
}

impl DirectionalLight {
    pub fn new(
        direction: Vec3,
        color: Color,
        intensity: f32,
    ) -> Self {
        assert!(
            direction.length() > 0.0,
            "directional light direction cannot be zero"
        );

        assert!(
            intensity >= 0.0,
            "light intensity cannot be negative"
        );

        Self {
            direction: direction.normalized(),
            color,
            intensity,
        }
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn set_direction(&mut self, direction: Vec3) {
        assert!(
            direction.length() > 0.0,
            "directional light direction cannot be zero"
        );

        self.direction = direction.normalized();
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn intensity(&self) -> f32 {
        self.intensity
    }

    pub fn set_intensity(&mut self, intensity: f32) {
        assert!(
            intensity >= 0.0,
            "light intensity cannot be negative"
        );

        self.intensity = intensity;
    }
}

pub trait LightSource: Copy {
    #[doc(hidden)]
    fn add_to_graphics(
        &self,
        graphics: &mut super::Graphics,
    );
}

impl LightSource for PointLight {
    fn add_to_graphics(
        &self,
        graphics: &mut super::Graphics,
    ) {
        graphics.add_point_light_internal(self);
    }
}

impl LightSource for SpotLight {
    fn add_to_graphics(
        &self,
        graphics: &mut super::Graphics,
    ) {
        graphics.add_spot_light_internal(self);
    }
}

impl LightSource for DirectionalLight {
    fn add_to_graphics(
        &self,
        graphics: &mut super::Graphics,
    ) {
        graphics.add_directional_light_internal(self);
    }
}