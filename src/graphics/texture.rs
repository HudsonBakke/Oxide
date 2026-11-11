use std::{
    io::{
        Read,
        Write,
    },
    path::Path,
    sync::atomic::{
        AtomicU64,
        Ordering,
    },
};

use super::Color;

use crate::content::{
    AssetType,
    ContentAsset,
    ContentError,
    ContentResult,
};

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

    pub(crate) fn from_rgba(
        width: u32,
        height: u32,
        pixels: Vec<u8>,
    ) -> Self {
        Self {
            id: next_texture_id(),
            width,
            height,
            pixels,
        }
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

impl ContentAsset for Texture {
    fn compiler_version() -> u32 {
        2
    }

    fn asset_type_name() -> &'static str {
        "Texture"
    }

    fn supports_extension(
        extension: &str,
    ) -> bool {
        matches!(
            extension,
            "png"
                | "jpg"
                | "jpeg"
                | "gif"
                | "bmp"
                | "tif"
                | "tiff"
                | "webp"
                | "gltf"
                | "glb"
        )
    }

    fn import(
        path: &Path,
        subobject: &str,
    ) -> ContentResult<Self> {
        let extension =
            path.extension()
                .and_then(|value| value.to_str())
                .unwrap_or("")
                .to_ascii_lowercase();

        match extension.as_str() {
            "gltf" | "glb" => {
                crate::content::import_gltf_texture(
                    path,
                    subobject,
                )
            }

            _ => {
                if !subobject.is_empty() {
                    return Err(
                        ContentError::SubObjectNotSupported {
                            path:
                                path.to_path_buf(),

                            asset_type:
                                Self::asset_type_name(),

                            subobject:
                                subobject.to_string(),
                        }
                    );
                }

                Self::load(path)
                    .map_err(
                        |error| {
                            ContentError::ImportFailed {
                                path:
                                    path.to_path_buf(),

                                message:
                                    error.to_string(),
                            }
                        }
                    )
            }
        }
    }

    fn compiled_asset_type()
        -> AssetType
    {
        AssetType::Texture
    }

    fn write_compiled(
        &self,
        writer: &mut impl Write,
    ) -> ContentResult<()> {
        writer.write_all(
            &self.width.to_le_bytes()
        )?;

        writer.write_all(
            &self.height.to_le_bytes()
        )?;

        let pixel_count =
            u64::try_from(
                self.pixels.len()
            )
            .map_err(
                |_| {
                    ContentError::InvalidCache(
                        "texture pixel data is too large"
                            .into()
                    )
                }
            )?;

        writer.write_all(
            &pixel_count.to_le_bytes()
        )?;

        writer.write_all(
            &self.pixels
        )?;

        Ok(())
    }

    fn read_compiled(
        reader: &mut impl Read,
        payload_size: u64,
    ) -> ContentResult<Self> {
        const HEADER_SIZE: u64 =
            4 + 4 + 8;

        if payload_size < HEADER_SIZE {
            return Err(
                ContentError::InvalidCache(
                    "texture payload is too small"
                        .into()
                )
            );
        }

        let mut u32_bytes =
            [0u8; 4];

        reader.read_exact(
            &mut u32_bytes
        )?;

        let width =
            u32::from_le_bytes(
                u32_bytes
            );

        reader.read_exact(
            &mut u32_bytes
        )?;

        let height =
            u32::from_le_bytes(
                u32_bytes
            );

        let mut u64_bytes =
            [0u8; 8];

        reader.read_exact(
            &mut u64_bytes
        )?;

        let pixel_count =
            u64::from_le_bytes(
                u64_bytes
            );

        let expected_pixels =
            u64::from(width)
                .checked_mul(
                    u64::from(height)
                )
                .and_then(
                    |value| {
                        value.checked_mul(4)
                    }
                )
                .ok_or_else(
                    || {
                        ContentError::InvalidCache(
                            "texture dimensions overflow"
                                .into()
                        )
                    }
                )?;

        if pixel_count
            != expected_pixels
        {
            return Err(
                ContentError::InvalidCache(
                    "texture pixel count does not match dimensions"
                        .into()
                )
            );
        }

        if payload_size
            != HEADER_SIZE + pixel_count
        {
            return Err(
                ContentError::InvalidCache(
                    "texture payload size is inconsistent"
                        .into()
                )
            );
        }

        let pixel_count =
            usize::try_from(
                pixel_count
            )
            .map_err(
                |_| {
                    ContentError::InvalidCache(
                        "texture is too large for this platform"
                            .into()
                    )
                }
            )?;

        let mut pixels =
            vec![
                0u8;
                pixel_count
            ];

        reader.read_exact(
            &mut pixels
        )?;

        Ok(
            Self {
                id: next_texture_id(),
                width,
                height,
                pixels,
            }
        )
    }
}