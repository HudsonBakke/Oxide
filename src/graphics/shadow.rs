use crate::{
    OxideError, OxideResult,
    math::{Mat4, Vec3},
};

/// Optional shadow quality, directional coverage, and local-light clipping settings.
///
/// Lights provide defaults automatically. Directional lights use center/size/depth;
/// point and spot lights derive their projection from the light and near/far planes.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShadowSettings {
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
    /// Positive local-light near clipping distance in world units. Default: 0.1.
    pub near_plane: f32,
    /// Local-light far clipping distance, greater than near_plane. Default: 60.
    pub far_plane: f32,
}

impl Default for ShadowSettings {
    fn default() -> Self {
        Self {
            center: Vec3::new(0.0, 0.0, 0.0),
            size: 30.0,
            depth: 60.0,
            resolution: 1024,
            depth_bias: 0.0001,
            normal_bias: 0.01,
            near_plane: 0.1,
            far_plane: 60.0,
        }
    }
}

impl ShadowSettings {
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
            && self.normal_bias >= 0.0
            && self.near_plane.is_finite() && self.near_plane > 0.0
            && self.far_plane.is_finite() && self.far_plane > self.near_plane;
        if !valid {
            return Err(OxideError::InvalidArgument {
                argument: "shadow settings",
                message: "shadow coverage must be finite and positive, center finite, resolution a power of two in 128..=4096, and biases finite and non-negative",
            });
        }
        Ok(())
    }

    pub(crate) fn perspective_matrix(&self, position: Vec3, direction: Vec3, fov: f32) -> OxideResult<Mat4> {
        self.validate()?;
        if ![position.x, position.y, position.z, direction.x, direction.y, direction.z, fov].iter().all(|v| v.is_finite())
            || direction.length() <= 1.0e-6 || !direction.length().is_finite()
            || fov <= 0.0 || fov >= std::f32::consts::PI {
            return Err(OxideError::InvalidArgument { argument: "light", message: "shadow projection requires finite position, nonzero direction, and field of view in (0, pi)" });
        }
        let forward = direction.normalized();
        let reference = if forward.y.abs() > 0.99 { Vec3::new(0.0, 0.0, 1.0) } else { Vec3::new(0.0, 1.0, 0.0) };
        let right = forward.cross(&reference).normalized();
        let up = right.cross(&forward).normalized();
        let view = Mat4::from_array([
            [right.x, right.y, right.z, -right.dot(&position)],
            [up.x, up.y, up.z, -up.dot(&position)],
            [-forward.x, -forward.y, -forward.z, forward.dot(&position)],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        Ok(Mat4::perspective(fov, 1.0, self.near_plane, self.far_plane) * view)
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

pub(crate) fn validate_transparency(transparency: f32) -> OxideResult<()> {
    if !transparency.is_finite() || !(0.0..=1.0).contains(&transparency) {
        return Err(OxideError::InvalidArgument {
            argument: "transparency",
            message: "shadow transparency must be finite and between zero and one",
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(actual: Vec3, expected: Vec3) {
        assert!((actual - expected).length() < 0.0001, "{actual:?} != {expected:?}");
    }

    #[test]
    fn light_volume_maps_center_and_near_far_planes() {
        let settings = ShadowSettings { center: Vec3::new(3.0, -2.0, 7.0), ..Default::default() };
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
        let settings = ShadowSettings { size: 20.0, ..Default::default() };
        let matrix = settings.light_matrix(Vec3::new(0.0, 0.0, -1.0)).unwrap();
        close(matrix.transform_point(Vec3::new(10.0, -10.0, 0.0)), Vec3::new(1.0, -1.0, 0.5));
        let model = Mat4::translation(5.0, 0.0, 0.0) * Mat4::scale(2.0, 2.0, 2.0);
        close((matrix * model).transform_point(Vec3::new(1.0, 0.0, 0.0)), Vec3::new(0.7, 0.0, 0.5));
    }

    #[test]
    fn invalid_shadow_inputs_are_rejected() {
        let defaults = ShadowSettings::default();
        for resolution in [0, 127, 129, 8192] {
            assert!(ShadowSettings { resolution, ..defaults }.validate().is_err());
        }
        for value in [0.0, -1.0, f32::NAN, f32::INFINITY] {
            assert!(ShadowSettings { size: value, ..defaults }.validate().is_err());
            assert!(ShadowSettings { depth: value, ..defaults }.validate().is_err());
        }
        for value in [-1.0, f32::NAN, f32::INFINITY] {
            assert!(ShadowSettings { depth_bias: value, ..defaults }.validate().is_err());
            assert!(ShadowSettings { normal_bias: value, ..defaults }.validate().is_err());
        }
        assert!(ShadowSettings { center: Vec3::new(f32::NAN, 0.0, 0.0), ..defaults }.validate().is_err());
        for direction in [Vec3::new(0.0, 0.0, 0.0), Vec3::new(f32::NAN, 1.0, 0.0)] {
            assert!(defaults.light_matrix(direction).is_err());
        }
        assert!(ShadowSettings { size: f32::MIN_POSITIVE / 4.0, ..defaults }.light_matrix(Vec3::new(0.0, -1.0, 0.0)).is_err());
    }

    #[test]
    fn shadow_transparency_rejects_invalid_values() {
        for value in [0.0, 0.5, 1.0] {
            assert!(validate_transparency(value).is_ok());
        }
        for value in [-0.01, 1.01, f32::NAN, f32::INFINITY] {
            assert!(validate_transparency(value).is_err());
        }
    }

    #[test]
    fn shadow_settings_have_the_c_abi_layout_and_valid_defaults() {
        assert_eq!(std::mem::size_of::<ShadowSettings>(), 40);
        assert_eq!(std::mem::offset_of!(ShadowSettings, resolution), 20);
        assert_eq!(std::mem::offset_of!(ShadowSettings, normal_bias), 28);
        assert!(ShadowSettings::default().validate().is_ok());
    }
}

/// Compatibility name for directional coverage; prefer ShadowSettings for all lights.
pub type DirectionalShadowSettings = ShadowSettings;
