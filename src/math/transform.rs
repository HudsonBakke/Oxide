use super::{
    Angle,
    Mat4,
    Vec3,
};

/// Position, orientation, and scale of an object in 3D space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    position: Vec3,

    pitch: Angle,
    yaw: Angle,
    roll: Angle,

    scale: Vec3,
}

impl Transform {
    /// Creates a transform with no translation or rotation
    /// and a scale of one.
    pub const fn new() -> Self {
        Self {
            position: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },

            pitch: Angle::ZERO,
            yaw: Angle::ZERO,
            roll: Angle::ZERO,

            scale: Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        }
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    pub fn translate_by(&mut self, translation: Vec3) {
        self.position = self.position + translation;
    }

    pub fn set_pitch(&mut self, pitch: Angle) {
        self.pitch = pitch;
    }

    pub fn set_yaw(&mut self, yaw: Angle) {
        self.yaw = yaw;
    }

    pub fn set_roll(&mut self, roll: Angle) {
        self.roll = roll;
    }

    pub fn rotate_pitch_by(&mut self, angle: Angle) {
        self.pitch = Angle::radians(
            self.pitch.as_radians() + angle.as_radians()
        );
    }

    pub fn rotate_yaw_by(&mut self, angle: Angle) {
        self.yaw = Angle::radians(
            self.yaw.as_radians() + angle.as_radians()
        );
    }

    pub fn rotate_roll_by(&mut self, angle: Angle) {
        self.roll = Angle::radians(
            self.roll.as_radians() + angle.as_radians()
        );
    }

    pub fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
    }

    /// Multiplies the current scale component-wise.
    pub fn scale_by(&mut self, scale: Vec3) {
        self.scale.x *= scale.x;
        self.scale.y *= scale.y;
        self.scale.z *= scale.z;
    }

    /// Produces the transformation matrix represented by this transform.
    pub(crate) fn matrix(&self) -> Mat4 {
        let translation = Mat4::translation(
            self.position.x,
            self.position.y,
            self.position.z,
        );

        let pitch =
            Mat4::rotation_x(self.pitch.as_radians());

        let yaw =
            Mat4::rotation_y(self.yaw.as_radians());

        let roll =
            Mat4::rotation_z(self.roll.as_radians());

        let scale = Mat4::scale(
            self.scale.x,
            self.scale.y,
            self.scale.z,
        );

        translation * yaw * pitch * roll * scale
    }

    pub(crate) fn matrix_around(&self, pivot: Vec3) -> Mat4 {
        let to_origin = Mat4::translation(
            -pivot.x,
            -pivot.y,
            -pivot.z,
        );

        self.matrix() * to_origin
    }

    /// Returns the transform's position.
    pub fn position(&self) -> Vec3 {
        self.position
    }

    /// Returns the transform's pitch.
    pub fn pitch(&self) -> Angle {
        self.pitch
    }

    /// Returns the transform's yaw.
    pub fn yaw(&self) -> Angle {
        self.yaw
    }

    /// Returns the transform's roll.
    pub fn roll(&self) -> Angle {
        self.roll
    }

    /// Returns the transform's scale.
    pub fn scale(&self) -> Vec3 {
        self.scale
    }

    pub(crate) fn normal_matrix(&self) -> Mat4 {
        let inverse_scale = Mat4::scale(
            if self.scale().x != 0.0 {
                1.0 / self.scale().x
            } else {
                0.0
            },
            if self.scale().y != 0.0 {
                1.0 / self.scale().y
            } else {
                0.0
            },
            if self.scale().z != 0.0 {
                1.0 / self.scale().z
            } else {
                0.0
            },
        );

        let pitch =
            Mat4::rotation_x(self.pitch().as_radians());

        let yaw =
            Mat4::rotation_y(self.yaw().as_radians());

        let roll =
            Mat4::rotation_z(self.roll().as_radians());

        yaw * pitch * roll * inverse_scale
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}