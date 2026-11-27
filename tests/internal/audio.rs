#[path = "../../src/audio/asset.rs"]
mod asset;
#[path = "../../src/audio/backend.rs"]
mod backend;
#[path = "../../src/audio/listener.rs"]
mod listener;
#[path = "audio_mixer.rs"]
mod mixer;
#[path = "../../src/audio/settings.rs"]
mod settings;
#[path = "../../src/audio/source.rs"]
mod source;
pub use asset::Audio;
pub use listener::AudioListener;
pub use settings::AudioSettings;
pub(crate) use settings::SettingsValues;
pub use source::{AudioSource, PlaybackState};

use std::sync::{Mutex, MutexGuard};

/// Failure to validate an audio operation or initialize/use the output device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AudioError {
    /// A value is nonfinite, out of range, or otherwise invalid.
    InvalidArgument(String),
    /// The Windows audio device could not be opened or stopped working.
    Device(String),
}

impl std::fmt::Display for AudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidArgument(message) => write!(f, "invalid audio argument: {message}"),
            Self::Device(message) => write!(f, "audio device error: {message}"),
        }
    }
}
impl std::error::Error for AudioError {}
/// Result of validating audio state or operating the playback device.
pub type AudioResult<T> = Result<T, AudioError>;

fn lock<T>(value: &Mutex<T>) -> MutexGuard<'_, T> {
    value.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn invalid(message: impl Into<String>) -> AudioError { AudioError::InvalidArgument(message.into()) }

fn position_valid(value: crate::math::Vec3) -> AudioResult<()> {
    if [value.x, value.y, value.z].iter().all(|v| v.is_finite() && v.abs() <= 1.0e12) { Ok(()) }
    else { Err(invalid("position components must be finite and within +/-1e12 world units")) }
}

fn direction_valid(value: crate::math::Vec3) -> AudioResult<crate::math::Vec3> {
    position_valid(value)?;
    if value.length() < 1.0e-6 { return Err(invalid("direction must be nonzero")); }
    Ok(value.normalized())
}
