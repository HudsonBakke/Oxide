use std::ops::Mul;

use super::{
    Angle,
    Vec3,
};

/// A quaternion representing a 3D rotation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub const IDENTITY: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };

    pub const fn new(
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    ) -> Self {
        Self { x, y, z, w }
    }

    /// Creates a rotation around an arbitrary axis.
    pub fn from_axis_angle(
        axis: Vec3,
        angle: Angle,
    ) -> Self {
        let axis = axis.normalized();

        let half_angle = angle.as_radians() * 0.5;
        let sin = half_angle.sin();

        Self {
            x: axis.x * sin,
            y: axis.y * sin,
            z: axis.z * sin,
            w: half_angle.cos(),
        }
    }

    /// Returns the magnitude of this quaternion.
    pub fn length(&self) -> f32 {
        (
            self.x * self.x
            + self.y * self.y
            + self.z * self.z
            + self.w * self.w
        ).sqrt()
    }

    /// Returns a normalized copy of this quaternion.
    pub fn normalized(&self) -> Self {
        let length = self.length();

        if length == 0.0 {
            return Self::IDENTITY;
        }

        Self {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
            w: self.w / length,
        }
    }
}

impl Default for Quaternion {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Mul for Quaternion {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x:
                self.w * other.x
                + self.x * other.w
                + self.y * other.z
                - self.z * other.y,

            y:
                self.w * other.y
                - self.x * other.z
                + self.y * other.w
                + self.z * other.x,

            z:
                self.w * other.z
                + self.x * other.y
                - self.y * other.x
                + self.z * other.w,

            w:
                self.w * other.w
                - self.x * other.x
                - self.y * other.y
                - self.z * other.z,
        }
    }
}