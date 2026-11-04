use std::{
    path::Path,
    sync::atomic::{
        AtomicU64,
        Ordering,
    },
};

use super::Color;

static NEXT_TEXTURE_ID: AtomicU64 =
    AtomicU64::new(1);

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub(crate) struct TextureId(u64);

fn next_texture_id() -> TextureId {
    TextureId(
        NEXT_TEXTURE_ID.fetch_add(
            1,
            Ordering::Relaxed,
        )
    )
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
    pub fn load<P: AsRef<Path>>(
        path: P,
    ) -> Result<Self, image::ImageError> {
        let image = image::open(path)?
            .to_rgba8();

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
            (
                value.clamp(0.0, 1.0)
                * 255.0
            ).round() as u8
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
}

impl Clone for Texture {
    fn clone(&self) -> Self {
        Self {
            id: next_texture_id(),

            width: self.width,
            height: self.height,

            pixels: self.pixels.clone(),
        }
    }
}