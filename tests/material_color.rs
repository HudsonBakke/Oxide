use std::{fs, time::{SystemTime, UNIX_EPOCH}};
use oxide::{content::ContentManager, graphics::Material};

#[test]
fn gltf_color_factor_stays_linear_and_survives_material_cache_roundtrip() {
    let root = std::path::PathBuf::from(format!(
        "target/test-content-cache/material-color-{}-{}",
        std::process::id(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
    ));
    fs::create_dir_all(&root).unwrap();
    image::ImageBuffer::from_pixel(1, 1, image::Rgba([128_u8, 128, 128, 255]))
        .save(root.join("color.png")).unwrap();
    let source = root.join("scene.gltf");
    fs::write(&source, r#"{
        "asset": {"version": "2.0"},
        "images": [{"uri": "color.png"}],
        "textures": [{"source": 0}],
        "materials": [{"pbrMetallicRoughness": {
            "baseColorTexture": {"index": 0},
            "baseColorFactor": [0.5, 0.25, 0.75, 0.5]
        }}]
    }"#).unwrap();
    let cache = root.join("cache");
    for _ in 0..2 {
        let material = ContentManager::with_cache_directory(&cache)
            .load::<Material>(&source).unwrap();
        let color = material.base_color();
        assert_eq!((color.r, color.g, color.b, color.a), (0.5, 0.25, 0.75, 0.5));
        assert_eq!(material.base_texture().width(), 1);
    }
    fs::remove_dir_all(root).unwrap();
}
