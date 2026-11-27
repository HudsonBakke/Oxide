use std::sync::{Arc, Mutex};
use super::{AudioResult, invalid, lock};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct SettingsValues {
    pub volume: f32,
    pub pitch: f32,
    pub reference_distance: f32,
    pub max_distance: f32,
    pub rolloff: f32,
    pub low_pass_cutoff: f32,
    pub modulation_frequency: f32,
    pub modulation_depth: f32,
    pub looping: u32,
}

impl Default for SettingsValues {
    fn default() -> Self {
        Self { volume: 1.0, pitch: 1.0, reference_distance: 1.0, max_distance: 50.0,
            rolloff: 1.0, low_pass_cutoff: 24_000.0, modulation_frequency: 0.0,
            modulation_depth: 0.0, looping: 0 }
    }
}
impl SettingsValues {
    pub(crate) fn validate(&self) -> AudioResult<()> {
        for (name, value, min, max) in [
            ("volume", self.volume, 0.0, 16.0), ("pitch", self.pitch, 0.0625, 16.0),
            ("reference_distance", self.reference_distance, 0.0001, 1.0e9),
            ("max_distance", self.max_distance, 0.0002, 1.0e12),
            ("rolloff", self.rolloff, 0.0, 16.0),
            ("low_pass_cutoff", self.low_pass_cutoff, 20.0, 24_000.0),
            ("modulation_frequency", self.modulation_frequency, 0.0, 100.0),
            ("modulation_depth", self.modulation_depth, 0.0, 1.0),
        ] {
            if !value.is_finite() || !(min..=max).contains(&value) {
                return Err(invalid(format!("{name} must be finite and in [{min}, {max}]")));
            }
        }
        if self.max_distance <= self.reference_distance { return Err(invalid("max_distance must exceed reference_distance")); }
        if self.looping > 1 { return Err(invalid("looping must be zero or one")); }
        Ok(())
    }
}

/// Shared, thread-safe playback controls. Clones refer to the same live settings.
///
/// Updating these values affects every source using this instance without
/// restarting playback. Changes are picked up by the mixer independently of Draw.
#[derive(Debug, Clone)]
pub struct AudioSettings { pub(crate) values: Arc<Mutex<SettingsValues>> }

impl Default for AudioSettings {
    fn default() -> Self { Self { values: Arc::new(Mutex::new(SettingsValues::default())) } }
}
impl AudioSettings {
    /// Creates unity volume/pitch, no effects, no looping, and a 1..50-unit distance range.
    pub fn new() -> Self { Self::default() }
    pub(crate) fn snapshot(&self) -> SettingsValues { *lock(&self.values) }
    pub(crate) fn replace(&self, values: SettingsValues) -> AudioResult<()> {
        values.validate()?;
        *lock(&self.values) = values;
        Ok(())
    }
    fn update(&self, change: impl FnOnce(&mut SettingsValues)) -> AudioResult<()> {
        let mut current = lock(&self.values);
        let mut next = *current;
        change(&mut next);
        next.validate()?;
        *current = next;
        Ok(())
    }
    /// Changes both distance bounds atomically; requires 0 < reference < maximum.
    pub fn set_distance_range(&self, reference: f32, maximum: f32) -> AudioResult<()> {
        self.update(|v| { v.reference_distance = reference; v.max_distance = maximum; })
    }
    /// Whether the sound wraps to its beginning; changing this affects active playback.
    pub fn looping(&self) -> bool { self.snapshot().looping != 0 }
    /// Enables or disables looping without restarting the sound.
    pub fn set_looping(&self, value: bool) { lock(&self.values).looping = value as u32; }
}

impl AudioSettings {
    /// Linear gain in [0,16]; one preserves amplitude.
    pub fn volume(&self) -> f32 { self.snapshot().volume }
    /// Changes volume; invalid values leave the current settings unchanged.
    pub fn set_volume(&self, value: f32) -> AudioResult<()> { self.update(|v| v.volume = value) }
    /// Playback-rate ratio in [1/16,16]; one preserves pitch and duration.
    pub fn pitch(&self) -> f32 { self.snapshot().pitch }
    /// Changes pitch; invalid values leave the current settings unchanged.
    pub fn set_pitch(&self, value: f32) -> AudioResult<()> { self.update(|v| v.pitch = value) }
    /// World-space distance at which attenuation begins.
    pub fn reference_distance(&self) -> f32 { self.snapshot().reference_distance }
    /// Changes reference_distance; invalid values leave the current settings unchanged.
    pub fn set_reference_distance(&self, value: f32) -> AudioResult<()> { self.update(|v| v.reference_distance = value) }
    /// World-space cutoff distance, strictly greater than reference_distance.
    pub fn max_distance(&self) -> f32 { self.snapshot().max_distance }
    /// Changes max_distance; invalid values leave the current settings unchanged.
    pub fn set_max_distance(&self, value: f32) -> AudioResult<()> { self.update(|v| v.max_distance = value) }
    /// Inverse-distance exponent in [0,16], combined with a fade to max_distance.
    pub fn rolloff(&self) -> f32 { self.snapshot().rolloff }
    /// Changes rolloff; invalid values leave the current settings unchanged.
    pub fn set_rolloff(&self, value: f32) -> AudioResult<()> { self.update(|v| v.rolloff = value) }
    /// Low-pass cutoff in Hz, [20,24000]; 24000 bypasses filtering.
    pub fn low_pass_cutoff(&self) -> f32 { self.snapshot().low_pass_cutoff }
    /// Changes low_pass_cutoff; invalid values leave the current settings unchanged.
    pub fn set_low_pass_cutoff(&self, value: f32) -> AudioResult<()> { self.update(|v| v.low_pass_cutoff = value) }
    /// Amplitude-modulation rate in Hz, [0,100]; zero disables modulation.
    pub fn modulation_frequency(&self) -> f32 { self.snapshot().modulation_frequency }
    /// Changes modulation_frequency; invalid values leave the current settings unchanged.
    pub fn set_modulation_frequency(&self, value: f32) -> AudioResult<()> { self.update(|v| v.modulation_frequency = value) }
    /// Amplitude-modulation depth in [0,1]; zero disables modulation.
    pub fn modulation_depth(&self) -> f32 { self.snapshot().modulation_depth }
    /// Changes modulation_depth; invalid values leave the current settings unchanged.
    pub fn set_modulation_depth(&self, value: f32) -> AudioResult<()> { self.update(|v| v.modulation_depth = value) }
}
