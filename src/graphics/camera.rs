use crate::math::{Angle, Mat4, Quaternion, Transform, Vec3};

/// Defines a viewpoint from which a 3D scene can be rendered.
#[derive(Debug, Clone, Copy, PartialEq)]
/// A perspective camera defining the viewpoint used to render a scene.
pub struct Camera {
    transform: Transform,

    field_of_view: Angle,
    near_plane: f32,
    far_plane: f32,
}

impl Camera {
    /// Creates a camera with default projection settings.
    pub const fn new() -> Self {
        Self {
            transform: Transform::new(),

            field_of_view: Angle::radians(std::f32::consts::FRAC_PI_3),

            near_plane: 0.1,
            far_plane: 1000.0,
        }
    }

    /// Public Oxide position API.
    pub fn position(&self) -> Vec3 {
        self.transform.position()
    }

    /// Public Oxide set position API.
    pub fn set_position(&mut self, position: Vec3) {
        self.transform.set_position(position);
    }

    /// Public Oxide translate by API.
    pub fn translate_by(&mut self, translation: Vec3) {
        self.transform.translate_by(translation);
    }

    /// Public Oxide rotate pitch by API.
    pub fn rotate_pitch_by(&mut self, angle: Angle) {
        self.transform.rotate_pitch_by(angle);
    }

    /// Public Oxide rotate yaw by API.
    pub fn rotate_yaw_by(&mut self, angle: Angle) {
        self.transform.rotate_yaw_by(angle);
    }

    /// Public Oxide rotate roll by API.
    pub fn rotate_roll_by(&mut self, angle: Angle) {
        self.transform.rotate_roll_by(angle);
    }

    /// Public Oxide rotation API.
    pub fn rotation(&self) -> Quaternion {
        self.transform.rotation()
    }

    /// Public Oxide set rotation API.
    pub fn set_rotation(&mut self, rotation: Quaternion) {
        self.transform.set_rotation(rotation);
    }

    /// Public Oxide set euler API.
    pub fn set_euler(&mut self, pitch: Angle, yaw: Angle, roll: Angle) {
        self.transform.set_euler(pitch, yaw, roll);
    }

    /// Public Oxide field of view API.
    pub fn field_of_view(&self) -> Angle {
        self.field_of_view
    }

    /// Public Oxide set field of view API.
    pub fn set_field_of_view(&mut self, field_of_view: Angle) -> crate::OxideResult<()> {
        let radians = field_of_view.as_radians();

        if radians <= 0.0 || radians >= std::f32::consts::PI {
            return Err(crate::OxideError::InvalidArgument {
                argument: "field_of_view",

                message: "field of view must be between 0 and pi radians",
            });
        }

        self.field_of_view = field_of_view;

        Ok(())
    }

    /// Public Oxide near plane API.
    pub fn near_plane(&self) -> f32 {
        self.near_plane
    }

    /// Public Oxide far plane API.
    pub fn far_plane(&self) -> f32 {
        self.far_plane
    }

    /// Public Oxide set clip planes API.
    pub fn set_clip_planes(&mut self, near_plane: f32, far_plane: f32) -> crate::OxideResult<()> {
        if near_plane <= 0.0 {
            return Err(crate::OxideError::InvalidArgument {
                argument: "near_plane",

                message: "near plane must be greater than zero",
            });
        }

        if far_plane <= near_plane {
            return Err(crate::OxideError::InvalidArgument {
                argument: "far_plane",

                message: "far plane must be farther than the near plane",
            });
        }

        self.near_plane = near_plane;

        self.far_plane = far_plane;

        Ok(())
    }

    pub(crate) fn view_matrix(&self) -> Mat4 {
        let position = self.transform.position();

        let inverse_translation = Mat4::translation(-position.x, -position.y, -position.z);

        let inverse_rotation = self.transform.rotation().conjugate().to_matrix();

        inverse_rotation * inverse_translation
    }

    pub(crate) fn projection_matrix(&self, aspect_ratio: f32) -> Mat4 {
        Mat4::perspective(
            self.field_of_view.as_radians(),
            aspect_ratio,
            self.near_plane,
            self.far_plane,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}
