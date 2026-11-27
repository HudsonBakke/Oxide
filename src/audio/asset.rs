use std::{io::{Read, Write}, path::Path, sync::Arc, time::Duration};
use crate::content::{AssetType, ContentAsset, ContentAssetInternal, ContentError, ContentResult};
use super::{AudioResult, invalid};

const MAX_SAMPLES: usize = 64 * 1024 * 1024;

/// Immutable decoded mono sound. Cloning shares sample storage; playback owns a clone.
///
/// Load WAV or MP3 through ContentManager. Multichannel input is averaged to mono
/// for positional playback. Assets are fully decoded and cached, rather than streamed.
#[derive(Debug, Clone)]
pub struct Audio { pub(crate) samples: Arc<[f32]>, sample_rate: u32 }
impl Audio {
    /// Copies normalized mono samples at 8,000..192,000 Hz. Requires nonempty,
    /// finite samples in `[-1,1]`, with at most 64 million samples (256 MiB).
    pub fn from_samples(samples: &[f32], sample_rate: u32) -> AudioResult<Self> {
        if !(8_000..=192_000).contains(&sample_rate) || samples.is_empty() || samples.len() > MAX_SAMPLES {
            return Err(invalid("audio needs 1..67108864 mono samples at 8000..192000 Hz"));
        }
        if samples.iter().any(|v| !v.is_finite() || v.abs() > 1.0) { return Err(invalid("samples must be finite and in [-1,1]")); }
        Ok(Self { samples: Arc::from(samples), sample_rate })
    }
    /// Number of mono samples per second.
    pub fn sample_rate(&self) -> u32 { self.sample_rate }
    /// Number of decoded mono samples.
    pub fn sample_count(&self) -> usize { self.samples.len() }
    /// Duration before pitch changes or looping.
    pub fn duration(&self) -> Duration { Duration::from_secs_f64(self.samples.len() as f64 / self.sample_rate as f64) }
}
impl ContentAsset for Audio {}
impl ContentAssetInternal for Audio {
    fn asset_type_name() -> &'static str { "Audio" }
    fn supports_extension(extension: &str) -> bool { matches!(extension, "wav" | "mp3") }
    fn compiler_version() -> u32 { 1 }
    fn compiled_asset_type() -> AssetType { AssetType::Audio }
    fn import(path: &Path, subobject: &str) -> ContentResult<Self> {
        if !subobject.is_empty() { return Err(ContentError::SubObjectNotSupported { path: path.into(), asset_type: "Audio", subobject: subobject.into() }); }
        // COM apartments belong to a thread. Decode on a short-lived MTA thread
        // so callers in a GUI/managed STA need no COM initialization or changes.
        let source = path.to_path_buf();
        std::thread::spawn(move || decode(&source)).join()
            .map_err(|_| ContentError::ImportFailed { path: path.into(), message: "audio decoder thread panicked".into() })?
            .map_err(|message| ContentError::ImportFailed { path: path.into(), message })
    }
    fn write_compiled(&self, writer: &mut impl Write) -> ContentResult<()> {
        writer.write_all(&self.sample_rate.to_le_bytes())?;
        writer.write_all(&(self.samples.len() as u64).to_le_bytes())?;
        for sample in self.samples.iter() { writer.write_all(&sample.to_le_bytes())?; }
        Ok(())
    }
    fn read_compiled(reader: &mut impl Read, size: u64) -> ContentResult<Self> {
        let mut rate = [0;4]; let mut count = [0;8];
        reader.read_exact(&mut rate)?; reader.read_exact(&mut count)?;
        let count = u64::from_le_bytes(count);
        if count == 0 || count > MAX_SAMPLES as u64 || 12 + count * 4 != size {
            return Err(ContentError::InvalidCache("invalid audio payload length".into()));
        }
        let mut samples = Vec::new();
        samples.try_reserve_exact(count as usize).map_err(|_| ContentError::InvalidCache("audio allocation failed".into()))?;
        for _ in 0..count { let mut bytes = [0;4]; reader.read_exact(&mut bytes)?; samples.push(f32::from_le_bytes(bytes)); }
        Self::from_samples(&samples, u32::from_le_bytes(rate)).map_err(|e| ContentError::InvalidCache(e.to_string()))
    }
}

fn decode(path: &Path) -> Result<Audio, String> {
    use std::os::windows::ffi::OsStrExt;
    use windows::{core::PCWSTR, Win32::{Media::MediaFoundation::*, System::Com::*}};
    struct Com;
    impl Drop for Com { fn drop(&mut self) { unsafe { CoUninitialize(); } } }
    struct Foundation;
    impl Drop for Foundation { fn drop(&mut self) { unsafe { let _ = MFShutdown(); } } }
    let result = (|| -> windows::core::Result<Audio> {
        unsafe {
            CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;
            let _com = Com;
            MFStartup(MF_VERSION, MFSTARTUP_FULL)?;
            let _foundation = Foundation;
            let path: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
            // ContentManager canonicalizes to verbatim Windows paths (\\?\).
            // A byte stream avoids URL resolution treating that prefix as a
            // network server, and keeps Unicode/long filesystem paths intact.
            let stream_file = MFCreateFile(MF_ACCESSMODE_READ, MF_OPENMODE_FAIL_IF_NOT_EXIST,
                MF_FILEFLAGS_NONE, PCWSTR(path.as_ptr()))?;
            let reader = MFCreateSourceReaderFromByteStream(&stream_file, None)?;
            let stream = MF_SOURCE_READER_FIRST_AUDIO_STREAM.0 as u32;
            reader.SetStreamSelection(MF_SOURCE_READER_ALL_STREAMS.0 as u32, false)?;
            reader.SetStreamSelection(stream, true)?;
            let format = MFCreateMediaType()?;
            format.SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Audio)?;
            format.SetGUID(&MF_MT_SUBTYPE, &MFAudioFormat_PCM)?;
            format.SetUINT32(&MF_MT_AUDIO_BITS_PER_SAMPLE, 16)?;
            reader.SetCurrentMediaType(stream, None, &format)?;
            let format = reader.GetCurrentMediaType(stream)?;
            let rate = format.GetUINT32(&MF_MT_AUDIO_SAMPLES_PER_SECOND)?;
            let channels = format.GetUINT32(&MF_MT_AUDIO_NUM_CHANNELS)? as usize;
            if channels == 0 || channels > 32 || format.GetUINT32(&MF_MT_AUDIO_BITS_PER_SAMPLE)? != 16 {
                return Err(windows::core::Error::new(windows::Win32::Foundation::E_INVALIDARG, "decoder did not provide 16-bit PCM audio"));
            }
            let mut samples = Vec::new();
            let mut empty_packets = 0;
            loop {
                let mut flags = 0; let mut sample = None;
                reader.ReadSample(stream, 0, None, Some(&mut flags), None, Some(&mut sample))?;
                if flags & MF_SOURCE_READERF_ERROR.0 as u32 != 0 {
                    return Err(windows::core::Error::new(windows::Win32::Foundation::E_FAIL, "audio decoder reported a stream error"));
                }
                if flags & MF_SOURCE_READERF_CURRENTMEDIATYPECHANGED.0 as u32 != 0 {
                    let current = reader.GetCurrentMediaType(stream)?;
                    if current.GetUINT32(&MF_MT_AUDIO_SAMPLES_PER_SECOND)? != rate || current.GetUINT32(&MF_MT_AUDIO_NUM_CHANNELS)? as usize != channels || current.GetUINT32(&MF_MT_AUDIO_BITS_PER_SAMPLE)? != 16 {
                        return Err(windows::core::Error::new(windows::Win32::Foundation::E_INVALIDARG, "midstream audio format changes are unsupported"));
                    }
                }
                if let Some(sample) = sample {
                    empty_packets = 0;
                    let buffer = sample.ConvertToContiguousBuffer()?;
                    let mut pointer = std::ptr::null_mut(); let mut length = 0;
                    buffer.Lock(&mut pointer, None, Some(&mut length))?;
                    // Always unlock before reporting malformed/oversized input.
                    let decoded = (|| -> windows::core::Result<()> {
                        if (length > 0 && pointer.is_null()) || length as usize % (channels * 2) != 0 || samples.len() + length as usize / (channels * 2) > MAX_SAMPLES {
                            return Err(windows::core::Error::new(windows::Win32::Foundation::E_INVALIDARG, "invalid or oversized decoded audio buffer"));
                        }
                        if length != 0 {
                            let bytes = std::slice::from_raw_parts(pointer, length as usize);
                            for frame in bytes.chunks_exact(channels * 2) {
                                let sum: f32 = frame.chunks_exact(2).map(|s| i16::from_le_bytes([s[0],s[1]]) as f32 / 32768.0).sum();
                                samples.push((sum / channels as f32).clamp(-1.0,1.0));
                            }
                        }
                        Ok(())
                    })();
                    let unlocked = buffer.Unlock(); decoded?; unlocked?;
                } else { empty_packets += 1; }
                if flags & MF_SOURCE_READERF_ENDOFSTREAM.0 as u32 != 0 { break; }
                if empty_packets > 1024 { return Err(windows::core::Error::new(windows::Win32::Foundation::E_FAIL, "audio decoder made no progress")); }
            }
            Audio::from_samples(&samples, rate).map_err(|e| windows::core::Error::new(windows::Win32::Foundation::E_INVALIDARG, e.to_string()))
        }
    })();
    result.map_err(|e| e.to_string())
}
