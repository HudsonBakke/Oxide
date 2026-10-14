use std::ops::{Add, Div, Mul, Sub};

/// A 3-dimensional vector
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec3 {
    /// The X component of the vector
    pub x: f32,

    /// The Y component of the vector
    pub y: f32,

    /// The Y component of the vector
    pub z: f32,
}

impl Vec3 {
    /// Creates a new vector with the given components
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Returns the magnitude of this vector
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Returns a new vector which is a normalized copy of this vector
    pub fn normalized(&self) -> Self {
        let length = self.length();

        if length == 0.0 {
            return Self::default();
        }

        Self {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    /// Normalizes this vector in-place
    pub fn normalize(&mut self) {
        let length = self.length();

        if length == 0.0 {
            return;
        }

        self.x /= length;
        self.y /= length;
        self.z /= length;
    }

    /// Returns the dot product of 2 vectors
    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x
            + self.y * other.y
            + self.z * other.z
    }

    /// Returns the cross product of 2 vectors
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}