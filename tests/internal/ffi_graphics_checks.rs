use super::*;

#[test]
fn shadow_defaults_cross_the_c_boundary_and_reject_null_output() {
    let mut settings = ShadowSettings { resolution: 0, ..Default::default() };
    assert_eq!(oxide_directional_shadow_settings_default(&mut settings), OxideStatus::Ok);
    assert_eq!(settings, ShadowSettings::default());
    assert_eq!(oxide_directional_shadow_settings_default(std::ptr::null_mut()), OxideStatus::InvalidArgument);
}
