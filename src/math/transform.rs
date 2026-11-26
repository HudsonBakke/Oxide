//! World placement using translation, quaternion orientation, and local scale.

use super::{Angle, Mat4, Quaternion, Vec3};

/// Position, orientation, and scale of an object in 3D space.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    position: Vec3,
    rotation: Quaternion,
    scale: Vec3,
}

impl Transform {
    /// Creates a transform at the origin with identity rotation and unit scale.
    pub const fn new() -> Self {
        Self {
            position: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },

            rotation: Quaternion::IDENTITY,

            scale: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        }
    }

    /// Position in world space.
    pub fn position(&self) -> Vec3 {
        self.position
    }

    /// Sets the position in world space.
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    /// Adds a world-space translation without rotating it by the current orientation.
    pub fn translate_by(&mut self, translation: Vec3) {
        self.position = self.position + translation;
    }

    /// Orientation quaternion; assigned values are normalized.
    pub fn rotation(&self) -> Quaternion {
        self.rotation
    }

    /// Sets the orientation to a normalized copy of the quaternion.
    pub fn set_rotation(&mut self, rotation: Quaternion) {
        self.rotation = rotation.normalized();
    }

    /// Sets the transform's orientation from Euler angles.
    ///
    /// Composition is yaw * pitch * roll; the rightmost rotation acts first.
    pub fn set_euler(&mut self, pitch: Angle, yaw: Angle, roll: Angle) {
        self.rotation = Quaternion::from_euler(pitch, yaw, roll);
    }

    /// Applies an additional rotation about the world X axis.
    pub fn rotate_pitch_by(&mut self, angle: Angle) {
        let rotation = Quaternion::from_axis_angle(Vec3::new(1.0, 0.0, 0.0), angle);

        self.rotation = (rotation * self.rotation).normalized();
    }

    /// Applies an additional rotation about the world Y axis.
    pub fn rotate_yaw_by(&mut self, angle: Angle) {
        let rotation = Quaternion::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), angle);

        self.rotation = (rotation * self.rotation).normalized();
    }

    /// Applies an additional rotation about the world Z axis.
    pub fn rotate_roll_by(&mut self, angle: Angle) {
        let rotation = Quaternion::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), angle);

        self.rotation = (rotation * self.rotation).normalized();
    }

    /// Component-wise scale along the model's local axes.
    pub fn scale(&self) -> Vec3 {
        self.scale
    }

    /// Sets the component-wise scale along the model's local axes.
    pub fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
    }

    /// Multiplies the current scale component by component.
    pub fn scale_by(&mut self, scale: Vec3) {
        self.scale.x *= scale.x;
        self.scale.y *= scale.y;
        self.scale.z *= scale.z;
    }

    /// Produces the transformation matrix represented by
    /// this transform.
    pub(crate) fn matrix(&self) -> Mat4 {
        let translation = Mat4::translation(self.position.x, self.position.y, self.position.z);

        let rotation = self.rotation.to_matrix();

        let scale = Mat4::scale(self.scale.x, self.scale.y, self.scale.z);

        translation * rotation * scale
    }

    /// Produces this transformation around a model-space pivot.
    pub(crate) fn matrix_around(&self, pivot: Vec3) -> Mat4 {
        let to_origin = Mat4::translation(-pivot.x, -pivot.y, -pivot.z);

        self.matrix() * to_origin
    }

    /// Produces the matrix used to transform surface normals.
    pub(crate) fn normal_matrix(&self) -> Mat4 {
        let inverse_scale = Mat4::scale(
            if self.scale.x != 0.0 {
                1.0 / self.scale.x
            } else {
                0.0
            },
            if self.scale.y != 0.0 {
                1.0 / self.scale.y
            } else {
                0.0
            },
            if self.scale.z != 0.0 {
                1.0 / self.scale.z
            } else {
                0.0
            },
        );

        self.rotation.to_matrix() * inverse_scale
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}
