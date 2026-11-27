use oxide::math::{Rect3D, Vec3};
fn rectangle(normal: Vec3) -> Rect3D {
    Rect3D::new(Vec3::new(-2.0,1.0,0.0),Vec3::new(2.0,1.0,0.0),Vec3::new(2.0,-1.0,0.0),Vec3::new(-2.0,-1.0,0.0),normal).unwrap()
}
#[test]
fn rectangle_normalizes_either_front_side_and_reports_world_dimensions() {
    let front=rectangle(Vec3::new(0.0,0.0,5.0));
    let back=rectangle(Vec3::new(0.0,0.0,-2.0));
    assert_eq!(front.normal(),Vec3::new(0.0,0.0,1.0));
    assert_eq!(back.normal(),Vec3::new(0.0,0.0,-1.0));
    assert_eq!((front.width(),front.height(),front.aspect_ratio()),(4.0,2.0,2.0));
    assert_eq!(std::mem::size_of::<Rect3D>(),60);
}
#[test]
fn rectangle_rejects_degenerate_skewed_nonplanar_and_invalid_normals() {
    let r=rectangle(Vec3::new(0.0,0.0,1.0));
    for (tl,tr,br,bl,n) in [
        (r.top_left(),r.top_left(),r.bottom_right(),r.bottom_left(),r.normal()),
        (r.top_left(),r.top_right(),r.bottom_right()+Vec3::new(0.5,0.0,0.0),r.bottom_left(),r.normal()),
        (r.top_left(),r.top_right(),r.bottom_right()+Vec3::new(0.0,0.0,0.5),r.bottom_left(),r.normal()),
        (r.top_left(),r.top_right(),r.bottom_right(),r.bottom_left(),Vec3::default()),
        (r.top_left(),r.top_right(),r.bottom_right(),r.bottom_left(),Vec3::new(1.0,0.0,1.0)),
        (Vec3::new(f32::NAN,0.0,0.0),r.top_right(),r.bottom_right(),r.bottom_left(),r.normal()),
    ] { assert!(Rect3D::new(tl,tr,br,bl,n).is_err()); }
    // A rotated rectangle is valid; axis alignment is not required.
    let u=Vec3::new(1.0,1.0,0.0); let v=Vec3::new(0.0,0.0,2.0);
    assert!(Rect3D::new(Vec3::default(),u,u+v,v,u.cross(&v)).is_ok());
}
