use crate::{
    OxideError, OxideResult,
    math::{Mat4, Vec3},
};

/// Orthographic coverage and receiver bias for one directional shadow map.
///
/// The volume is centered on `center`, with square cross-section perpendicular
/// to the light rays. Submit all relevant opaque casters, including those above
/// the visible scene. Positions outside this volume receive no shadow.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectionalShadowSettings {
    /// World-space center of the shadow volume; defaults to the origin.
    pub center: Vec3,
    /// Full width and height in world units; finite and positive. Default: 30.
    pub size: f32,
    /// Full extent along the light rays; finite and positive. Default: 60.
    pub depth: f32,
    /// Square map size in pixels, a power of two from 128 through 4096. Default: 1024.
    pub resolution: u32,
    /// Non-negative finite offset subtracted from comparison depth in [0, 1] units. Default: 0.0001.
    pub depth_bias: f32,
    /// Non-negative finite receiver offset along the geometric normal in world units. Default: 0.01.
    pub normal_bias: f32,
}

impl Default for DirectionalShadowSettings {
    fn default() -> Self {
        Self {
            center: Vec3::new(0.0, 0.0, 0.0),
            size: 30.0,
            depth: 60.0,
            resolution: 1024,
            depth_bias: 0.0001,
            normal_bias: 0.01,
        }
    }
}

impl DirectionalShadowSettings {
    /// Checks coverage, resolution, and bias, returning InvalidArgument on invalid values.
    pub fn validate(&self) -> OxideResult<()> {
        let valid = [self.center.x, self.center.y, self.center.z]
            .iter()
            .all(|v| v.is_finite())
            && self.size.is_finite()
            && self.size > 0.0
            && self.depth.is_finite()
            && self.depth > 0.0
            && (128..=4096).contains(&self.resolution)
            && self.resolution.is_power_of_two()
            && self.depth_bias.is_finite()
            && self.depth_bias >= 0.0
            && self.normal_bias.is_finite()
            && self.normal_bias >= 0.0;
        if !valid {
            return Err(OxideError::InvalidArgument {
                argument: "shadow settings",
                message: "shadow coverage must be finite and positive, center finite, resolution a power of two in 128..=4096, and biases finite and non-negative",
            });
        }
        Ok(())
    }

    pub(crate) fn light_matrix(&self, direction: Vec3) -> OxideResult<Mat4> {
        self.validate()?;
        let length = direction.length();
        if !length.is_finite() || length <= 1.0e-6 {
            return Err(OxideError::InvalidArgument {
                argument: "direction",
                message: "shadow light direction must be finite and nonzero",
            });
        }
        let forward = direction.normalized();
        let reference = if forward.y.abs() > 0.99 {
            Vec3::new(0.0, 0.0, 1.0)
        } else {
            Vec3::new(0.0, 1.0, 0.0)
        };
        // A right-handed light view looks down -Z; increasing depth follows
        // the light rays. The alternate reference handles vertical sunlight.
        let right = forward.cross(&reference).normalized();
        let up = right.cross(&forward).normalized();
        let xy = 2.0 / self.size;
        let z = 1.0 / self.depth;
        let matrix = Mat4::from_array([
            [
                right.x * xy, right.y * xy, right.z * xy,
                -right.dot(&self.center) * xy,
            ],
            [up.x * xy, up.y * xy, up.z * xy, -up.dot(&self.center) * xy],
            [
                forward.x * z, forward.y * z, forward.z * z,
                0.5 - forward.dot(&self.center) * z,
            ],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        if matrix.as_array().iter().flatten().any(|v| !v.is_finite()) {
            return Err(OxideError::InvalidArgument {
                argument: "shadow settings",
                message: "shadow volume cannot be represented by a finite projection",
            });
        }
        Ok(matrix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(actual: Vec3, expected: Vec3) {
        assert!((actual - expected).length() < 0.0001, "{actual:?} != {expected:?}");
    }

    #[test]
    fn light_volume_maps_center_and_near_far_planes() {
        let settings = DirectionalShadowSettings { center: Vec3::new(3.0, -2.0, 7.0), ..Default::default() };
        for direction in [Vec3::new(-0.5, -1.0, -0.3), Vec3::new(0.0, -1.0, 0.0), Vec3::new(0.0, 1.0, 0.0)] {
            let matrix = settings.light_matrix(direction).unwrap();
            let extent = direction.normalized() * (settings.depth * 0.5);
            close(matrix.transform_point(settings.center), Vec3::new(0.0, 0.0, 0.5));
            close(matrix.transform_point(settings.center - extent), Vec3::new(0.0, 0.0, 0.0));
            close(matrix.transform_point(settings.center + extent), Vec3::new(0.0, 0.0, 1.0));
        }
    }

    #[test]
    fn light_volume_preserves_square_coverage_and_model_transform() {
        let settings = DirectionalShadowSettings { size: 20.0, ..Default::default() };
        let matrix = settings.light_matrix(Vec3::new(0.0, 0.0, -1.0)).unwrap();
        close(matrix.transform_point(Vec3::new(10.0, -10.0, 0.0)), Vec3::new(1.0, -1.0, 0.5));
        let model = Mat4::translation(5.0, 0.0, 0.0) * Mat4::scale(2.0, 2.0, 2.0);
        close((matrix * model).transform_point(Vec3::new(1.0, 0.0, 0.0)), Vec3::new(0.7, 0.0, 0.5));
    }

    #[test]
    fn invalid_shadow_inputs_are_rejected() {
        let defaults = DirectionalShadowSettings::default();
        for resolution in [0, 127, 129, 8192] {
            assert!(DirectionalShadowSettings { resolution, ..defaults }.validate().is_err());
        }
        for value in [0.0, -1.0, f32::NAN, f32::INFINITY] {
            assert!(DirectionalShadowSettings { size: value, ..defaults }.validate().is_err());
            assert!(DirectionalShadowSettings { depth: value, ..defaults }.validate().is_err());
        }
        for value in [-1.0, f32::NAN, f32::INFINITY] {
            assert!(DirectionalShadowSettings { depth_bias: value, ..defaults }.validate().is_err());
            assert!(DirectionalShadowSettings { normal_bias: value, ..defaults }.validate().is_err());
        }
        assert!(DirectionalShadowSettings { center: Vec3::new(f32::NAN, 0.0, 0.0), ..defaults }.validate().is_err());
        for direction in [Vec3::new(0.0, 0.0, 0.0), Vec3::new(f32::NAN, 1.0, 0.0)] {
            assert!(defaults.light_matrix(direction).is_err());
        }
        assert!(DirectionalShadowSettings { size: f32::MIN_POSITIVE / 4.0, ..defaults }.light_matrix(Vec3::new(0.0, -1.0, 0.0)).is_err());
    }

    #[test]
    fn shadow_settings_have_the_c_abi_layout_and_valid_defaults() {
        assert_eq!(std::mem::size_of::<DirectionalShadowSettings>(), 32);
        assert_eq!(std::mem::offset_of!(DirectionalShadowSettings, resolution), 20);
        assert_eq!(std::mem::offset_of!(DirectionalShadowSettings, normal_bias), 28);
        assert!(DirectionalShadowSettings::default().validate().is_ok());
    }
}
