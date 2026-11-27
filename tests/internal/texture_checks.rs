use super::*;
use crate::math::Vec3;

fn sample(texture: &Texture) -> [f32; 3] {
    sample_direction(texture, Vec3::new(1.0, 0.0, 0.0))
}

#[test]
fn environment_sampling_decodes_srgb_before_filtering() {
    let gray = Texture::rgba(1, 1, vec![128, 128, 128, 255]).unwrap();
    assert!((sample(&gray)[0] - 0.21586).abs() < 0.00001);
    let contrast = Texture::rgba(2, 1, vec![0, 0, 0, 255, 255, 255, 255, 255]).unwrap();
    // Bilinear interpolation averages linear black and white, not encoded gray.
    assert!((sample(&contrast)[0] - 0.5).abs() < 0.00001);
}

#[test]
fn solid_colors_and_hdr_radiance_are_not_srgb_decoded() {
    let solid = Texture::solid(Color::rgb(0.5, 0.5, 0.5));
    assert!((sample(&solid)[0] - 128.0 / 255.0).abs() < 0.00001);
    let hdr = Texture::from_hdr_rgba(1, 1, vec![4.0, 2.0, 1.0, 1.0]);
    assert_eq!(sample(&hdr), [4.0, 2.0, 1.0]);
}

#[test]
fn constant_hdr_environment_preserves_radiance_and_irradiance_energy() {
    let texture = Texture::from_hdr_rgba(1, 1, vec![4.0, 2.0, 1.0, 1.0]);
    let normal = Vec3::new(0.0, 1.0, 0.0);
    let irradiance = integrate_irradiance(&texture, normal, 96);
    let specular = integrate_ggx(&texture, normal, 0.7, 64);
    for channel in 0..3 {
        let radiance = [4.0, 2.0, 1.0][channel];
        assert!((irradiance[channel] - std::f32::consts::PI * radiance).abs() < 0.0001);
        assert!((specular[channel] - radiance).abs() < 0.0001);
    }
}

#[test]
fn texture_cache_preserves_linear_srgb_and_hdr_encodings() {
    for texture in [
        Texture::solid(Color::rgb(0.5, 0.25, 0.75)),
        Texture::rgba(1, 1, vec![128, 64, 192, 255]).unwrap(),
        Texture::from_hdr_rgba(1, 1, vec![8.0, 2.0, 0.5, 1.0]),
    ] {
        let mut bytes = Vec::new();
        texture.write_compiled(&mut bytes).unwrap();
        let restored = Texture::read_compiled(&mut bytes.as_slice(), bytes.len() as u64).unwrap();
        assert_eq!(texture.srgb_encoded(), restored.srgb_encoded());
        assert_eq!(sample(&texture), sample(&restored));
    }
}

#[test]
fn environment_prefilter_bounds_resolution_without_clamping_hdr() {
    let texture = Texture::from_hdr_rgba(640, 1, [4.0, 2.0, 1.0, 1.0].repeat(640));
    let filtered = texture.environment_prefilter();
    assert_eq!((texture.width(), texture.height()), (640, 1));
    assert_eq!(filtered.specular.len(), 5);
    assert_eq!((filtered.specular[0].0, filtered.specular[0].1), (512, 1));
    for (width, height, pixels) in filtered.specular {
        assert!(width <= 512 && height <= 512);
        for pixel in pixels.chunks_exact(4) {
            assert!((pixel[0] - 4.0).abs() < 0.0001);
            assert!((pixel[1] - 2.0).abs() < 0.0001);
        }
    }
    let tiny = Texture::solid(Color::WHITE).environment_prefilter();
    assert_eq!(tiny.specular.len(), 1);
    assert_eq!((tiny.specular[0].0, tiny.specular[0].1), (1, 1));
}
