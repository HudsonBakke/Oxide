use crate::math::{
    Angle,
    Mat4,
    Transform,
    Vec3,
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

    pub fn pitch(&self) -> Angle {
        self.transform.pitch()
    }

    pub fn set_pitch(&mut self, pitch: Angle) {
        self.transform.set_pitch(pitch);
    }

    pub fn rotate_pitch_by(&mut self, angle: Angle) {
        self.transform.rotate_pitch_by(angle);
    }

    pub fn yaw(&self) -> Angle {
        self.transform.yaw()
    }

    pub fn set_yaw(&mut self, yaw: Angle) {
        self.transform.set_yaw(yaw);
    }

    pub fn rotate_yaw_by(&mut self, angle: Angle) {
        self.transform.rotate_yaw_by(angle);
    }

    pub fn roll(&self) -> Angle {
        self.transform.roll()
    }

    pub fn set_roll(&mut self, roll: Angle) {
        self.transform.set_roll(roll);
    }

    pub fn rotate_roll_by(&mut self, angle: Angle) {
        self.transform.rotate_roll_by(angle);
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
        let position = self.transform.position();

        let inverse_translation = Mat4::translation(
            -position.x,
            -position.y,
            -position.z,
        );

        let inverse_pitch =
            Mat4::rotation_x(
                -self.transform.pitch().as_radians()
            );

        let inverse_yaw =
            Mat4::rotation_y(
                -self.transform.yaw().as_radians()
            );

        let inverse_roll =
            Mat4::rotation_z(
                -self.transform.roll().as_radians()
            );

        inverse_roll
            * inverse_pitch
            * inverse_yaw
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Vec4;

    const EPSILON: f32 = 0.0001;

    fn assert_close(
        actual: f32,
        expected: f32,
    ) {
        assert!(
            (actual - expected).abs() < EPSILON,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn camera_translation_moves_world_opposite_direction() {
        let mut camera = Camera::new();

        camera.set_position(
            Vec3::new(0.0, 0.0, 5.0)
        );

        let world_point = Vec4::new(
            0.0,
            0.0,
            0.0,
            1.0,
        );

        let camera_point =
            camera.view_matrix() * world_point;

        assert_close(camera_point.x, 0.0);
        assert_close(camera_point.y, 0.0);
        assert_close(camera_point.z, -5.0);
        assert_close(camera_point.w, 1.0);
    }

    #[test]
    fn camera_yaw_uses_inverse_rotation() {
        let mut camera = Camera::new();

        camera.set_yaw(
            Angle::degrees(90.0)
        );

        let world_point = Vec4::new(
            0.0,
            0.0,
            -1.0,
            1.0,
        );

        let camera_point =
            camera.view_matrix() * world_point;

        assert_close(camera_point.x, 1.0);
        assert_close(camera_point.y, 0.0);
        assert_close(camera_point.z, 0.0);
    }

    #[test]
    fn projection_uses_supplied_aspect_ratio() {
        let camera = Camera::new();

        let square =
            camera.projection_matrix(1.0);

        let widescreen =
            camera.projection_matrix(16.0 / 9.0);

        // Vertical projection is unchanged.
        assert_close(
            square.get(1, 1),
            widescreen.get(1, 1),
        );

        // Horizontal projection changes with aspect ratio.
        assert!(
            (square.get(0, 0)
                - widescreen.get(0, 0))
                .abs()
                > EPSILON
        );
    }
}