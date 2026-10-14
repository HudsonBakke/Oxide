use std::ops::{Add, Div, Mul, Sub};
use super::vec3::Vec3;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub fn from_position(v: Vec3) -> Self {
        Self::new(v.x, v.y, v.z, 1.0)
    }

    pub fn from_direction(v: Vec3) -> Self {
        Self::new(v.x, v.y, v.z, 0.0)
    }
}

impl Add for Vec4 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Vec4 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}

impl Div<f32> for Vec4 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar,
        }
    }
}