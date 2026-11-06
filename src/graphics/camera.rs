use crate::math::{
    Angle,
    Mat4,
    Transform,
    Vec3,
    Quaternion,
};

/// Defines a viewpoint from which a 3D scene can be rendered.
#[derive(Debug, Clone, Copy, PartialEq)]
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

            field_of_view: Angle::radians(
                std::f32::consts::FRAC_PI_3
            ),

            near_plane: 0.1,
            far_plane: 1000.0,
        }
    }

    pub fn position(&self) -> Vec3 {
        self.transform.position()
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.transform.set_position(position);
    }

    pub fn translate_by(&mut self, translation: Vec3) {
        self.transform.translate_by(translation);
    }

    pub fn rotate_pitch_by(&mut self, angle: Angle) {
        self.transform.rotate_pitch_by(angle);
    }

    pub fn rotate_yaw_by(&mut self, angle: Angle) {
        self.transform.rotate_yaw_by(angle);
    }

    pub fn rotate_roll_by(&mut self, angle: Angle) {
        self.transform.rotate_roll_by(angle);
    }

    pub fn rotation(&self) -> Quaternion {
        self.transform.rotation()
    }

    pub fn set_rotation(
        &mut self,
        rotation: Quaternion,
    ) {
        self.transform.set_rotation(rotation);
    }

    pub fn set_euler(
        &mut self,
        pitch: Angle,
        yaw: Angle,
        roll: Angle,
    ) {
        self.transform.set_euler(
            pitch,
            yaw,
            roll,
        );
    }

    pub fn field_of_view(&self) -> Angle {
        self.field_of_view
    }

    pub fn set_field_of_view(&mut self, field_of_view: Angle) {
        assert!(
            field_of_view.as_radians() > 0.0
                && field_of_view.as_radians()
                    < std::f32::consts::PI,
            "camera field of view must be between 0 and pi radians"
        );

        self.field_of_view = field_of_view;
    }

    pub fn near_plane(&self) -> f32 {
        self.near_plane
    }

    pub fn far_plane(&self) -> f32 {
        self.far_plane
    }

    pub fn set_clip_planes(
        &mut self,
        near_plane: f32,
        far_plane: f32,
    ) {
        assert!(
            near_plane > 0.0,
            "camera near plane must be greater than zero"
        );

        assert!(
            far_plane > near_plane,
            "camera far plane must be farther than the near plane"
        );

        self.near_plane = near_plane;
        self.far_plane = far_plane;
    }

    pub(crate) fn view_matrix(&self) -> Mat4 {
        let position =
            self.transform.position();

        let inverse_translation =
            Mat4::translation(
                -position.x,
                -position.y,
                -position.z,
            );

        let inverse_rotation =
            self.transform
                .rotation()
                .conjugate()
                .to_matrix();

        inverse_rotation
            * inverse_translation
    }

    pub(crate) fn projection_matrix(
        &self,
        aspect_ratio: f32,
    ) -> Mat4 {
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
