use super::*;
use crate::math::{Angle,Rect};

#[test]
fn sprite_validation_rejects_invalid_effects_and_keeps_defaults_independent() {
    let defaults=Draw2DOptions::default();assert!(defaults.validate().is_ok());assert_eq!(defaults.tint,Color::WHITE);
    for value in [f32::NAN,f32::INFINITY,-1.0,1.01] {
        let mut invalid=defaults;invalid.tint.a=value;assert!(invalid.validate().is_err());
    }
    for source in [Rect::new(-1.0,0.0,1.0,1.0),Rect::new(0.0,0.0,0.0,1.0),Rect::new(0.0,0.0,1.0,-1.0)] {
        assert!(Draw2DOptions {source:Some(source),..defaults}.validate().is_err());
    }
    assert!(Draw2DOptions {rotation:Angle::radians(f32::NAN),..defaults}.validate().is_err());
    assert!(draw2d::validate_rect(Rect::new(0.0,0.0,0.0,1.0)).is_ok());
    assert!(draw2d::validate_rect(Rect::new(f32::INFINITY,0.0,1.0,1.0)).is_err());
}

#[test]
fn text_cache_shapes_unicode_measures_newlines_and_bounds_memory() {
    let mut cache=text::TextCache::default();let options=TextOptions::default();
    let first=cache.get("Oxide café Ελληνικά 日本語",&options).unwrap();
    assert!(first.size.x>10.0 && first.size.y>10.0);
    assert!(first.texture.pixels().chunks_exact(4).any(|p|p[3]>0));
    assert!(first.texture.pixels().chunks_exact(4).all(|p|p[..3]==[255,255,255]));
    assert!(std::sync::Arc::ptr_eq(&first,&cache.get("Oxide café Ελληνικά 日本語",&options).unwrap()));
    let one=cache.get("Oxide",&options).unwrap();let two=cache.get("Oxide\nOxide",&options).unwrap();
    assert!(two.size.y>one.size.y*1.9);
    let spaces=cache.get("Oxide   ",&options).unwrap();assert!(spaces.size.x>one.size.x);
    assert_eq!(cache.get("",&options).unwrap().size,crate::math::Vec2::default());
    assert!(cache.get("bad\0text",&options).is_err());
    assert!(cache.get("text",&TextOptions {font_size:f32::NAN,..options.clone()}).is_err());
    assert!(cache.get(&"W".repeat(1000),&options).is_err());
    for i in 0..130 {cache.get(&format!("Label {i}"),&options).unwrap();}
    assert!(!std::sync::Arc::ptr_eq(&first,&cache.get("Oxide café Ελληνικά 日本語",&options).unwrap()));
    // Evicted images remain alive while an outstanding caller retains them.
    assert!(first.texture.width()>0);
}
