//! Quaternion construction, composition, and vector rotation.

use std::ops::Mul;

use super::{Angle, Mat4, Vec3};

/// A quaternion representing a 3D rotation.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion {
    /// X component.
    pub x: f32,
    /// Y component.
    pub y: f32,
    /// Z component.
    pub z: f32,
    /// W component.
    pub w: f32,
}

impl Quaternion {
    /// Rotation quaternion (0, 0, 0, 1), which leaves vectors unchanged.
    pub const IDENTITY: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };

    /// Creates a quaternion from XYZW components without normalizing it.
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Creates a rotation about a nonzero axis, normalizing the axis before use.
    pub fn from_axis_angle(axis: Vec3, angle: Angle) -> Self {
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

    /// Euclidean magnitude of the components.
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    /// Returns a unit-length quaternion; a zero quaternion becomes the identity.
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

    /// Creates an orientation from pitch, yaw, and roll.
    ///
    /// Rotation order matches Transform:
    /// yaw * pitch * roll; the rightmost rotation acts first on a vector.
    pub fn from_euler(pitch: Angle, yaw: Angle, roll: Angle) -> Self {
        let pitch_rotation = Self::from_axis_angle(Vec3::new(1.0, 0.0, 0.0), pitch);

        let yaw_rotation = Self::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), yaw);

        let roll_rotation = Self::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), roll);

        (yaw_rotation * pitch_rotation * roll_rotation).normalized()
    }

    /// Returns the quaternion with negated XYZ components; the inverse for a unit quaternion.
    pub fn conjugate(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }

    /// Rotates a vector by this quaternion; use a unit quaternion for a pure rotation.
    pub fn rotate_vector(&self, vector: Vec3) -> Vec3 {
        let q = Vec3::new(self.x, self.y, self.z);

        let t = q.cross(&vector) * 2.0;

        vector + t * self.w + q.cross(&t)
    }

    /// Returns the shortest rotation aligning two nonzero directions; vector lengths are ignored.
    pub fn from_to(from: Vec3, to: Vec3) -> Self {
        let from = from.normalized();
        let to = to.normalized();

        let dot = from.dot(&to);

        if dot > 1.0 - 1.0e-6 {
            return Self::IDENTITY;
        }

        if dot < -1.0 + 1.0e-6 {
            let candidate = if from.x.abs() <= from.y.abs() && from.x.abs() <= from.z.abs() {
                Vec3::new(1.0, 0.0, 0.0)
            } else if from.y.abs() <= from.z.abs() {
                Vec3::new(0.0, 1.0, 0.0)
            } else {
                Vec3::new(0.0, 0.0, 1.0)
            };

            let axis = from.cross(&candidate).normalized();

            return Self::from_axis_angle(axis, Angle::radians(std::f32::consts::PI));
        }

        let axis = from.cross(&to);

        Self::new(axis.x, axis.y, axis.z, 1.0 + dot).normalized()
    }

    /// Converts this quaternion into a rotation matrix.
    pub(crate) fn to_matrix(&self) -> Mat4 {
        let q = self.normalized();

        let xx = q.x * q.x;
        let yy = q.y * q.y;
        let zz = q.z * q.z;

        let xy = q.x * q.y;
        let xz = q.x * q.z;
        let yz = q.y * q.z;

        let wx = q.w * q.x;
        let wy = q.w * q.y;
        let wz = q.w * q.z;

        Mat4::from_array([
            [1.0 - 2.0 * (yy + zz), 2.0 * (xy - wz), 2.0 * (xz + wy), 0.0],
            [2.0 * (xy + wz), 1.0 - 2.0 * (xx + zz), 2.0 * (yz - wx), 0.0],
            [2.0 * (xz - wy), 2.0 * (yz + wx), 1.0 - 2.0 * (xx + yy), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
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
            x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,

            y: self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,

            z: self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,

            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
        }
    }
}
