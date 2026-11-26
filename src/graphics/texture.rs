use std::{
    io::{Read, Write},
    path::Path,
    sync::atomic::{AtomicU64, Ordering},
};

use super::Color;

use crate::content::{AssetType, ContentAsset, ContentAssetInternal, ContentError, ContentResult};

static NEXT_TEXTURE_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct TextureId(u64);

fn next_texture_id() -> TextureId {
    TextureId(NEXT_TEXTURE_ID.fetch_add(1, Ordering::Relaxed))
}

/// A 2D texture containing RGBA image data.
#[derive(Debug)]
pub struct Texture {
    id: TextureId,

    width: u32,
    height: u32,

    pixels: Vec<u8>,
}

impl Texture {
    /// Loads a texture from an image file.
    pub(crate) fn load<P: AsRef<Path>>(path: P) -> Result<Self, image::ImageError> {
        let image = image::open(path)?.to_rgba8();

        let width = image.width();
        let height = image.height();

        Ok(Self {
            id: next_texture_id(),
            width,
            height,
            pixels: image.into_raw(),
        })
    }

    /// Creates a 1x1 solid-color texture.
    pub fn solid(color: Color) -> Self {
        fn channel(value: f32) -> u8 {
            (value.clamp(0.0, 1.0) * 255.0).round() as u8
        }

        Self {
            id: next_texture_id(),

            width: 1,
            height: 1,

            pixels: vec![
                channel(color.r),
                channel(color.g),
                channel(color.b),
                channel(color.a),
            ],
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub(crate) fn id(&self) -> TextureId {
        self.id
    }

    pub(crate) fn pixels(&self) -> &[u8] {
        &self.pixels
    }

    /// Creates a texture directly from RGBA8 pixel data.
    pub fn rgba(
        width: u32,
        height: u32,
        pixels: Vec<u8>,
    ) -> crate::OxideResult<Self> {
        let expected = (width as usize)
            .checked_mul(height as usize)
            .and_then(|value| value.checked_mul(4))
            .ok_or(crate::OxideError::InvalidArgument {
                argument: "pixels",
                message: "texture dimensions overflow",
            })?;

        if pixels.len() != expected {
            return Err(crate::OxideError::InvalidArgument {
                argument: "pixels",
                message: "RGBA pixel count does not match texture dimensions",
            });
        }

        Ok(Self::from_rgba(width, height, pixels))
    }


    pub(crate) fn environment_prefilter(&self) -> EnvironmentPrefilter {
        const SPECULAR_LEVELS: u32 = 5;
        const SPECULAR_SAMPLES: u32 = 64;
        const IRRADIANCE_SAMPLES: u32 = 96;

        let mut specular = Vec::with_capacity(SPECULAR_LEVELS as usize);

        for level in 0..SPECULAR_LEVELS {
            let roughness = level as f32 / (SPECULAR_LEVELS - 1) as f32;
            let width = (self.width >> level).max(1);
            let height = (self.height >> level).max(1);
            let mut pixels = vec![0_u8; width as usize * height as usize * 4];

            for y in 0..height {
                for x in 0..width {
                    let direction = equirectangular_direction(x, y, width, height);
                    let color = if level == 0 {
                        sample_direction(self, direction)
                    } else {
                        integrate_ggx(self, direction, roughness, SPECULAR_SAMPLES)
                    };
                    write_rgb(&mut pixels, x, y, width, color);
                }
            }

            specular.push((width, height, pixels));
        }

        let irradiance_width = self.width.min(64).max(1);
        let irradiance_height = (irradiance_width / 2).max(1);
        let mut irradiance =
            vec![0_u8; irradiance_width as usize * irradiance_height as usize * 4];

        for y in 0..irradiance_height {
            for x in 0..irradiance_width {
                let normal = equirectangular_direction(
                    x, y, irradiance_width, irradiance_height
                );
                let color = integrate_irradiance(self, normal, IRRADIANCE_SAMPLES);
                write_rgb(&mut irradiance, x, y, irradiance_width, color);
            }
        }

        EnvironmentPrefilter {
            specular,
            irradiance: (irradiance_width, irradiance_height, irradiance),
        }
    }

    pub(crate) fn from_rgba(width: u32, height: u32, pixels: Vec<u8>) -> Self {
        Self {
            id: next_texture_id(),
            width,
            height,
            pixels,
        }
    }
}


pub(crate) struct EnvironmentPrefilter {
    pub(crate) specular: Vec<(u32, u32, Vec<u8>)>,
    pub(crate) irradiance: (u32, u32, Vec<u8>),
}

fn radical_inverse_vdc(mut bits: u32) -> f32 {
    bits = (bits << 16) | (bits >> 16);
    bits = ((bits & 0x5555_5555) << 1) | ((bits & 0xAAAA_AAAA) >> 1);
    bits = ((bits & 0x3333_3333) << 2) | ((bits & 0xCCCC_CCCC) >> 2);
    bits = ((bits & 0x0F0F_0F0F) << 4) | ((bits & 0xF0F0_F0F0) >> 4);
    bits = ((bits & 0x00FF_00FF) << 8) | ((bits & 0xFF00_FF00) >> 8);
    bits as f32 * 2.3283064365386963e-10
}

fn hammersley(i: u32, count: u32) -> (f32, f32) {
    (i as f32 / count as f32, radical_inverse_vdc(i))
}

fn equirectangular_direction(x: u32, y: u32, width: u32, height: u32) -> crate::math::Vec3 {
    let u = (x as f32 + 0.5) / width as f32;
    let v = (y as f32 + 0.5) / height as f32;
    let phi = (u - 0.5) * std::f32::consts::TAU;
    let theta = v * std::f32::consts::PI;
    crate::math::Vec3::new(
        theta.sin() * phi.cos(),
        theta.cos(),
        theta.sin() * phi.sin(),
    )
}

fn basis(normal: crate::math::Vec3) -> (crate::math::Vec3, crate::math::Vec3) {
    let up = if normal.y.abs() < 0.999 {
        crate::math::Vec3::new(0.0, 1.0, 0.0)
    } else {
        crate::math::Vec3::new(1.0, 0.0, 0.0)
    };
    let tangent = up.cross(&normal).normalized();
    let bitangent = normal.cross(&tangent).normalized();
    (tangent, bitangent)
}

fn sample_direction(texture: &Texture, direction: crate::math::Vec3) -> [f32; 3] {
    let d = direction.normalized();
    let u = (d.z.atan2(d.x) / std::f32::consts::TAU + 0.5).rem_euclid(1.0);
    let v = d.y.clamp(-1.0, 1.0).acos() / std::f32::consts::PI;
    let fx = u * texture.width as f32 - 0.5;
    let fy = v * texture.height as f32 - 0.5;
    let x0 = fx.floor() as i32;
    let y0 = fy.floor() as i32;
    let tx = fx - fx.floor();
    let ty = fy - fy.floor();

    let fetch = |x: i32, y: i32| {
        let x = x.rem_euclid(texture.width as i32) as u32;
        let y = y.clamp(0, texture.height as i32 - 1) as u32;
        let index = ((y * texture.width + x) * 4) as usize;
        [
            texture.pixels[index] as f32 / 255.0,
            texture.pixels[index + 1] as f32 / 255.0,
            texture.pixels[index + 2] as f32 / 255.0,
        ]
    };

    let a = fetch(x0, y0);
    let b = fetch(x0 + 1, y0);
    let c = fetch(x0, y0 + 1);
    let d = fetch(x0 + 1, y0 + 1);
    let mut result = [0.0; 3];
    for channel in 0..3 {
        let top = a[channel] * (1.0 - tx) + b[channel] * tx;
        let bottom = c[channel] * (1.0 - tx) + d[channel] * tx;
        result[channel] = top * (1.0 - ty) + bottom * ty;
    }
    result
}

fn integrate_irradiance(
    texture: &Texture,
    normal: crate::math::Vec3,
    samples: u32,
) -> [f32; 3] {
    let (tangent, bitangent) = basis(normal);
    let mut sum = [0.0_f32; 3];

    // Cosine-weighted hemisphere sampling. Because p(L)=N.L/pi,
    // the Monte Carlo irradiance estimator reduces to pi * average(Li).
    for i in 0..samples {
        let (u1, u2) = hammersley(i, samples);
        let r = u1.sqrt();
        let phi = std::f32::consts::TAU * u2;
        let local = crate::math::Vec3::new(
            r * phi.cos(),
            r * phi.sin(),
            (1.0 - u1).sqrt(),
        );
        let direction =
            tangent * local.x + bitangent * local.y + normal * local.z;
        let sample = sample_direction(texture, direction);
        for channel in 0..3 {
            sum[channel] += sample[channel];
        }
    }

    let scale = std::f32::consts::PI / samples as f32;
    [sum[0] * scale, sum[1] * scale, sum[2] * scale]
}

fn integrate_ggx(
    texture: &Texture,
    normal: crate::math::Vec3,
    roughness: f32,
    samples: u32,
) -> [f32; 3] {
    let view = normal;
    let (tangent, bitangent) = basis(normal);
    let alpha = (roughness * roughness).max(0.001);
    let alpha2 = alpha * alpha;
    let mut sum = [0.0_f32; 3];
    let mut weight = 0.0_f32;

    for i in 0..samples {
        let (u1, u2) = hammersley(i, samples);
        let phi = std::f32::consts::TAU * u1;
        let cos_theta =
            ((1.0 - u2) / (1.0 + (alpha2 - 1.0) * u2)).sqrt();
        let sin_theta = (1.0 - cos_theta * cos_theta).max(0.0).sqrt();
        let half_local = crate::math::Vec3::new(
            sin_theta * phi.cos(),
            sin_theta * phi.sin(),
            cos_theta,
        );
        let half_vector = (
            tangent * half_local.x
            + bitangent * half_local.y
            + normal * half_local.z
        ).normalized();
        let light = (half_vector * (2.0 * view.dot(&half_vector)) - view).normalized();
        let n_dot_l = normal.dot(&light).max(0.0);
        if n_dot_l > 0.0 {
            let sample = sample_direction(texture, light);
            for channel in 0..3 {
                sum[channel] += sample[channel] * n_dot_l;
            }
            weight += n_dot_l;
        }
    }

    if weight <= 1.0e-6 {
        return sample_direction(texture, normal);
    }
    [sum[0] / weight, sum[1] / weight, sum[2] / weight]
}

fn write_rgb(pixels: &mut [u8], x: u32, y: u32, width: u32, color: [f32; 3]) {
    let index = ((y * width + x) * 4) as usize;
    for channel in 0..3 {
        pixels[index + channel] =
            (color[channel].clamp(0.0, 1.0) * 255.0).round() as u8;
    }
    pixels[index + 3] = 255;
}

impl Clone for Texture {
    fn clone(&self) -> Self {
        // A clone is another handle to the same immutable texture contents,
        // so it must preserve resource identity for GPU caching.
        Self {
            id: self.id,
            width: self.width,
            height: self.height,
            pixels: self.pixels.clone(),
        }
    }
}

impl ContentAssetInternal for Texture {
    fn compiler_version() -> u32 {
        2
    }

    fn asset_type_name() -> &'static str {
        "Texture"
    }

    fn supports_extension(extension: &str) -> bool {
        matches!(
            extension,
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "tif" | "tiff" | "webp" | "gltf" | "glb"
        )
    }

    fn import(path: &Path, subobject: &str) -> ContentResult<Self> {
        let extension = path
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();

        match extension.as_str() {
            "gltf" | "glb" => crate::content::import_gltf_texture(path, subobject),

            _ => {
                if !subobject.is_empty() {
                    return Err(ContentError::SubObjectNotSupported {
                        path: path.to_path_buf(),

                        asset_type: Self::asset_type_name(),

                        subobject: subobject.to_string(),
                    });
                }

                Self::load(path).map_err(|error| ContentError::ImportFailed {
                    path: path.to_path_buf(),

                    message: error.to_string(),
                })
            }
        }
    }

    fn compiled_asset_type() -> AssetType {
        AssetType::Texture
    }

    fn write_compiled(&self, writer: &mut impl Write) -> ContentResult<()> {
        writer.write_all(&self.width.to_le_bytes())?;

        writer.write_all(&self.height.to_le_bytes())?;

        let pixel_count = u64::try_from(self.pixels.len())
            .map_err(|_| ContentError::InvalidCache("texture pixel data is too large".into()))?;

        writer.write_all(&pixel_count.to_le_bytes())?;

        writer.write_all(&self.pixels)?;

        Ok(())
    }

    fn read_compiled(reader: &mut impl Read, payload_size: u64) -> ContentResult<Self> {
        const HEADER_SIZE: u64 = 4 + 4 + 8;

        if payload_size < HEADER_SIZE {
            return Err(ContentError::InvalidCache(
                "texture payload is too small".into(),
            ));
        }

        let mut u32_bytes = [0u8; 4];

        reader.read_exact(&mut u32_bytes)?;

        let width = u32::from_le_bytes(u32_bytes);

        reader.read_exact(&mut u32_bytes)?;

        let height = u32::from_le_bytes(u32_bytes);

        let mut u64_bytes = [0u8; 8];

        reader.read_exact(&mut u64_bytes)?;

        let pixel_count = u64::from_le_bytes(u64_bytes);

        let expected_pixels = u64::from(width)
            .checked_mul(u64::from(height))
            .and_then(|value| value.checked_mul(4))
            .ok_or_else(|| ContentError::InvalidCache("texture dimensions overflow".into()))?;

        if pixel_count != expected_pixels {
            return Err(ContentError::InvalidCache(
                "texture pixel count does not match dimensions".into(),
            ));
        }

        if payload_size != HEADER_SIZE + pixel_count {
            return Err(ContentError::InvalidCache(
                "texture payload size is inconsistent".into(),
            ));
        }

        let pixel_count = usize::try_from(pixel_count).map_err(|_| {
            ContentError::InvalidCache("texture is too large for this platform".into())
        })?;

        let mut pixels = vec![0u8; pixel_count];

        reader.read_exact(&mut pixels)?;

        Ok(Self {
            id: next_texture_id(),
            width,
            height,
            pixels,
        })
    }
}

impl ContentAsset for Texture {}
