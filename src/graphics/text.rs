//! Bounded CPU label cache. Rasterization stays in a private COM apartment.

use std::{
    collections::VecDeque,
    sync::{Arc, mpsc},
    thread,
};

use windows::{
    core::{Error, PCWSTR, Result, w},
    Win32::{
        Foundation::{E_FAIL, E_INVALIDARG},
        Graphics::{Direct2D::{Common::*, *}, DirectWrite::*, Dxgi::Common::*, Imaging::*},
        System::Com::*,
    },
};

use crate::math::Vec2;
use super::{TextOptions, Texture};

const MAX_LABELS: usize = 128;
const MAX_PIXEL_BYTES: usize = 16 * 1024 * 1024;

type CacheKey = (String, String, u32);
type Request = (String, String, f32, mpsc::SyncSender<Result<TextImage>>);

pub(crate) struct TextImage {
    pub(crate) texture: Texture,
    pub(crate) size: Vec2,
    pub(crate) offset: Vec2,
}

#[derive(Default)]
pub(crate) struct TextCache {
    entries: VecDeque<(CacheKey, Arc<TextImage>)>,
    bytes: usize,
    sender: Option<mpsc::Sender<Request>>,
    worker: Option<thread::JoinHandle<()>>,
}

impl TextCache {
    pub(crate) fn get(&mut self, text: &str, options: &TextOptions) -> Result<Arc<TextImage>> {
        options.validate(text).map_err(|e| Error::new(E_INVALIDARG, e.to_string()))?;
        let key = (text.to_owned(), options.font_family.clone(), options.font_size.to_bits());
        if let Some(index) = self.entries.iter().position(|(k, _)| k == &key) {
            let item = self.entries.remove(index).unwrap();
            let image = item.1.clone();
            self.entries.push_back(item);
            return Ok(image);
        }
        self.start_worker()?;
        let (reply, result) = mpsc::sync_channel(1);
        self.sender.as_ref().unwrap()
            .send((text.into(), options.font_family.clone(), options.font_size, reply))
            .map_err(|_| Error::new(E_FAIL, "text worker stopped"))?;
        let image = Arc::new(result.recv()
            .map_err(|_| Error::new(E_FAIL, "text worker stopped"))??);
        let bytes = image.texture.pixels().len();
        while self.entries.len() >= MAX_LABELS || self.bytes + bytes > MAX_PIXEL_BYTES {
            let Some((_, old)) = self.entries.pop_front() else { break; };
            self.bytes -= old.texture.pixels().len();
        }
        self.bytes += bytes;
        self.entries.push_back((key, image.clone()));
        Ok(image)
    }

    fn start_worker(&mut self) -> Result<()> {
        if self.sender.is_some() {
            return Ok(());
        }
        let (sender, receiver) = mpsc::channel::<Request>();
        self.worker = Some(thread::Builder::new()
            .name("Oxide text".into())
            .spawn(move || {
                let initialized = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED).ok() };
                for (text, family, size, reply) in receiver {
                    let result = match &initialized {
                        Ok(()) => unsafe { rasterize(&text, &family, size) },
                        Err(error) => Err(error.clone()),
                    };
                    let _ = reply.send(result);
                }
                if initialized.is_ok() {
                    unsafe { CoUninitialize() };
                }
            })
            .map_err(|e| Error::new(E_FAIL, e.to_string()))?);
        self.sender = Some(sender);
        Ok(())
    }
}

impl Drop for TextCache {
    fn drop(&mut self) {
        self.sender.take();
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

// All COM interfaces are local to the worker and released before CoUninitialize.
unsafe fn rasterize(text: &str, family: &str, size: f32) -> Result<TextImage> {
    unsafe {
        if text.is_empty() {
            return Ok(TextImage {
                texture: Texture::rgba(1, 1, vec![255, 255, 255, 0]).unwrap(),
                size: Vec2::default(),
                offset: Vec2::default(),
            });
        }
        let write: IDWriteFactory = DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)?;
        let family: Vec<u16> = family.encode_utf16().chain(Some(0)).collect();
        let format = write.CreateTextFormat(
            PCWSTR(family.as_ptr()), None, DWRITE_FONT_WEIGHT_NORMAL,
            DWRITE_FONT_STYLE_NORMAL, DWRITE_FONT_STRETCH_NORMAL, size, w!("en-us"),
        )?;
        format.SetWordWrapping(DWRITE_WORD_WRAPPING_NO_WRAP)?;
        let utf16: Vec<u16> = text.encode_utf16().collect();
        let layout = write.CreateTextLayout(&utf16, &format, 2048.0, 2048.0)?;
        let mut metrics = DWRITE_TEXT_METRICS::default();
        layout.GetMetrics(&mut metrics)?;
        let overhang = layout.GetOverhangMetrics()?;
        let left = 2.0 + overhang.left.max(0.0);
        let top = 2.0 + overhang.top.max(0.0);
        let width = (metrics.widthIncludingTrailingWhitespace + left
            + overhang.right.max(0.0) + 2.0).ceil().max(1.0) as u32;
        let height = (metrics.height + top + overhang.bottom.max(0.0) + 2.0)
            .ceil().max(1.0) as u32;
        if width > 2048 || height > 2048 {
            return Err(Error::new(E_INVALIDARG,
                "text label exceeds 2048 pixels; split it into smaller labels"));
        }
        let wic: IWICImagingFactory = CoCreateInstance(
            &CLSID_WICImagingFactory, None, CLSCTX_INPROC_SERVER,
        )?;
        let bitmap = wic.CreateBitmap(
            width, height, &GUID_WICPixelFormat32bppPBGRA, WICBitmapCacheOnLoad,
        )?;
        let d2d: ID2D1Factory = D2D1CreateFactory(D2D1_FACTORY_TYPE_SINGLE_THREADED, None)?;
        let target = d2d.CreateWicBitmapRenderTarget(&bitmap, &D2D1_RENDER_TARGET_PROPERTIES {
            r#type: D2D1_RENDER_TARGET_TYPE_SOFTWARE,
            pixelFormat: D2D1_PIXEL_FORMAT {
                format: DXGI_FORMAT_B8G8R8A8_UNORM,
                alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
            },
            dpiX: 96.0,
            dpiY: 96.0,
            ..Default::default()
        })?;
        let brush = target.CreateSolidColorBrush(
            &D2D1_COLOR_F { r: 1.0, g: 1.0, b: 1.0, a: 1.0 }, None,
        )?;
        target.SetTextAntialiasMode(D2D1_TEXT_ANTIALIAS_MODE_GRAYSCALE);
        target.BeginDraw();
        target.Clear(Some(&D2D1_COLOR_F::default()));
        target.DrawTextLayout(
            windows_numerics::Vector2 { X: left, Y: top },
            &layout, &brush, D2D1_DRAW_TEXT_OPTIONS_NONE,
        );
        target.EndDraw(None, None)?;
        let mut pixels = vec![0u8; width as usize * height as usize * 4];
        bitmap.CopyPixels(std::ptr::null(), width * 4, &mut pixels)?;
        // D2D returns premultiplied white. Preserve coverage as alpha and restore
        // straight white RGB so the sprite blend multiplies alpha exactly once.
        for pixel in pixels.chunks_exact_mut(4) {
            pixel[0] = 255;
            pixel[1] = 255;
            pixel[2] = 255;
        }
        Ok(TextImage {
            texture: Texture::rgba(width, height, pixels).unwrap(),
            size: Vec2::new(metrics.widthIncludingTrailingWhitespace, metrics.height),
            offset: Vec2::new(-left, -top),
        })
    }
}
