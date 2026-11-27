use super::*;
fn shared_contract<L: LightSource>(mut light: L) {
    assert!(!light.casts_shadows());
    light.set_casts_shadows(true);
    assert!(light.casts_shadows());
    assert_eq!(*light.shadow_settings(), ShadowSettings::default());
    let custom = ShadowSettings { resolution: 256, ..Default::default() };
    light.set_shadow_settings(custom).unwrap();
    assert_eq!(*light.shadow_settings(), custom);
    assert!(light.set_shadow_settings(ShadowSettings { far_plane: 0.0, ..custom }).is_err());
    assert_eq!(*light.shadow_settings(), custom);
}
#[test]
fn light_types_share_shadow_defaults_and_configuration() {
    shared_contract(PointLight::new(Vec3::default(), Color::WHITE, 1.0).unwrap());
    shared_contract(DirectionalLight::new(Vec3::new(0.0, -1.0, 0.0), Color::WHITE, 1.0).unwrap());
    shared_contract(SpotLight::new(Vec3::default(), Vec3::new(0.0, -1.0, 0.0), Angle::degrees(30.0), Color::WHITE, 1.0).unwrap());
    assert_eq!(std::mem::size_of::<PointLight>(), 76);
    assert_eq!(std::mem::size_of::<DirectionalLight>(), 76);
    assert_eq!(std::mem::size_of::<SpotLight>(), 92);
}
#[test]
fn light_interface_supports_heterogeneous_dynamic_dispatch() {
    let mut lights: Vec<Box<dyn LightSource>> = vec![
        Box::new(PointLight::new(Vec3::default(), Color::WHITE, 1.0).unwrap()),
        Box::new(DirectionalLight::new(Vec3::new(0.0, -1.0, 0.0), Color::WHITE, 1.0).unwrap()),
        Box::new(SpotLight::new(Vec3::default(), Vec3::new(0.0, -1.0, 0.0), Angle::degrees(30.0), Color::WHITE, 1.0).unwrap()),
    ];
    for light in &mut lights {
        light.set_casts_shadows(true);
        light.set_intensity(2.0).unwrap();
        assert!(light.casts_shadows());
        assert_eq!(light.intensity(), 2.0);
        assert_eq!(light.shadow_settings().resolution, 1024);
    }
}
