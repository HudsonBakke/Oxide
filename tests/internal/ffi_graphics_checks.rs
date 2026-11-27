use super::*;

#[test]
fn shadow_defaults_cross_the_c_boundary_and_reject_null_output() {
    let mut settings = ShadowSettings { resolution: 0, ..Default::default() };
    assert_eq!(oxide_directional_shadow_settings_default(&mut settings), OxideStatus::Ok);
    assert_eq!(settings, ShadowSettings::default());
    assert_eq!(oxide_directional_shadow_settings_default(std::ptr::null_mut()), OxideStatus::InvalidArgument);
}

#[test]
fn rect3d_native_constructor_and_bind_reject_invalid_values_and_null_handles() {
    use crate::math::{Rect3D,Vec3};
    use super::super::camera::oxide_rect3d_create;
    let tl=Vec3::new(-1.0,1.0,0.0);let tr=Vec3::new(1.0,1.0,0.0);let br=Vec3::new(1.0,-1.0,0.0);let bl=Vec3::new(-1.0,-1.0,0.0);let n=Vec3::new(0.0,0.0,4.0);
    let mut rectangle=Rect3D::new(tl,tr,br,bl,n).unwrap();
    assert_eq!(oxide_rect3d_create(tl,tr,br,bl,n,&mut rectangle),OxideStatus::Ok);
    assert_eq!(rectangle.normal(),Vec3::new(0.0,0.0,1.0));
    assert_eq!(oxide_rect3d_create(tl,tr,br,bl,n,std::ptr::null_mut()),OxideStatus::InvalidArgument);
    assert_eq!(oxide_graphics_bind_camera_3d(OxideGraphicsHandle(std::ptr::null_mut()),OxideCameraHandle(0),rectangle),OxideStatus::InvalidArgument);
    let zero:Rect3D=unsafe {std::mem::zeroed()};
    assert_eq!(oxide_graphics_bind_camera_3d(OxideGraphicsHandle(std::ptr::null_mut()),OxideCameraHandle(0),zero),OxideStatus::InvalidArgument);
}
