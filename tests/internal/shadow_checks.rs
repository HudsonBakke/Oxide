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
