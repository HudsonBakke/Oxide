use std::sync::{Arc, Mutex};
use crate::math::{Angle, Vec3};
use super::{Audio, AudioResult, AudioSettings, backend::Engine, direction_valid, invalid, lock, position_valid};

/// Lifecycle of the one playback owned by an AudioSource.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackState {
    /// No sound is active, or nonlooping playback has finished.
    Stopped = 0,
    /// Samples are advancing and being mixed, even outside audible range.
    Playing = 1,
    /// Samples are retained, with the playback cursor held in place.
    Paused = 2,
}
#[derive(Debug, Clone, Copy)]
pub(crate) struct Cone { pub inner: f32, pub outer: f32, pub outer_volume: f32 }
impl Cone {
    pub(crate) fn new(inner: Angle, outer: Angle, outer_volume: f32) -> AudioResult<Self> {
        let inner = inner.as_radians(); let outer = outer.as_radians();
        if !inner.is_finite() || !outer.is_finite() || inner < 0.0 || outer > std::f32::consts::PI || inner > outer || !outer_volume.is_finite() || !(0.0..=1.0).contains(&outer_volume) {
            return Err(invalid("cone half-angles need 0 <= inner <= outer <= pi; outer_volume must be in [0,1]"));
        }
        Ok(Self { inner, outer, outer_volume })
    }
}
pub(crate) struct SourceState {
    pub position: Vec3, pub direction: Vec3, pub settings: AudioSettings,
    pub audio: Option<Audio>, pub state: PlaybackState, pub cone: Option<Cone>,
    pub cursor: f64, pub gain: [f32;2], pub pitch: f32, pub filtered: f32,
    pub filter: f32, pub phase: f64,
}
impl SourceState {
    pub(crate) fn new(position: Vec3, settings: AudioSettings, direction: Vec3) -> Self {
        Self { position, direction, settings, audio: None, state: PlaybackState::Stopped,
            cone: None, cursor: 0.0, gain: [0.0;2], pitch: 1.0, filtered: 0.0, filter: 1.0, phase: 0.0 }
    }
    pub(crate) fn play(&mut self, audio: &Audio, cone: Option<Cone>) {
        self.audio = Some(audio.clone()); self.cone = cone; self.cursor = 0.0;
        self.gain = [0.0;2]; self.pitch = self.settings.pitch(); self.filtered = 0.0; self.phase = 0.0;
        self.state = PlaybackState::Playing;
    }
    pub(crate) fn stop(&mut self) { self.audio = None; self.state = PlaybackState::Stopped; self.cursor = 0.0; self.gain = [0.0;2]; }
}

/// A positional emitter with one playback, shared live settings, and a normalized direction.
///
/// Play replaces the previous sound; create multiple sources to overlap sounds.
/// Dropping a source stops it. The source retains its clip and settings, so their
/// original owners may be dropped while it plays. No scene geometry is inspected.
pub struct AudioSource { pub(crate) data: Arc<Mutex<SourceState>>, engine: Arc<Engine> }
impl AudioSource {
    /// Creates an emitter and lazily opens the default Windows output device.
    /// Returns an error if position/direction are invalid or no device is available.
    pub fn new(position: Vec3, settings: AudioSettings, direction: Vec3) -> AudioResult<Self> {
        position_valid(position)?; let direction = direction_valid(direction)?;
        let engine = Engine::shared()?;
        let data = Arc::new(Mutex::new(SourceState::new(position, settings, direction)));
        engine.register(&data)?;
        Ok(Self { data, engine })
    }
    /// Shares the source's settings; changing the returned instance changes active playback.
    pub fn settings(&self) -> AudioSettings { lock(&self.data).settings.clone() }
    /// Replaces settings for active and future playback; the supplied instance is shared.
    pub fn set_settings(&self, settings: AudioSettings) { lock(&self.data).settings = settings; }
    /// Current world-space emitter position.
    pub fn position(&self) -> Vec3 { lock(&self.data).position }
    /// Changes position without restarting playback.
    pub fn set_position(&self, position: Vec3) -> AudioResult<()> { position_valid(position)?; lock(&self.data).position = position; Ok(()) }
    /// Current normalized emission direction; used only by directional playback.
    pub fn direction(&self) -> Vec3 { lock(&self.data).direction }
    /// Normalizes a finite nonzero vector and changes direction during playback.
    pub fn set_direction(&self, direction: Vec3) -> AudioResult<()> { let direction = direction_valid(direction)?; lock(&self.data).direction = direction; Ok(()) }
    /// Replaces the current sound with omnidirectional, distance-attenuated playback.
    pub fn play(&self, audio: &Audio) -> AudioResult<()> { self.engine.check()?; lock(&self.data).play(audio, None); Ok(()) }
    /// Replaces the current sound with cone emission. Angles are half-angles;
    /// gain interpolates from one inside the inner cone to outer_volume outside
    /// the outer cone. Distance attenuation applies in addition to cone attenuation.
    pub fn play_directional(&self, audio: &Audio, inner: Angle, outer: Angle, outer_volume: f32) -> AudioResult<()> {
        let cone = Cone::new(inner, outer, outer_volume)?;
        self.engine.check()?; lock(&self.data).play(audio, Some(cone)); Ok(())
    }
    /// Updates an active directional cone without restarting. Also selects the
    /// cone for a currently paused directional sound; Play resets to omnidirectional.
    pub fn set_cone(&self, inner: Angle, outer: Angle, outer_volume: f32) -> AudioResult<()> {
        let cone = Cone::new(inner, outer, outer_volume)?; lock(&self.data).cone = Some(cone); Ok(())
    }
    /// Pauses a playing sound at its current cursor; otherwise does nothing.
    pub fn pause(&self) { let mut state = lock(&self.data); if state.state == PlaybackState::Playing { state.state = PlaybackState::Paused; state.gain = [0.0;2]; } }
    /// Resumes a paused sound, or does nothing if stopped/already playing.
    pub fn resume(&self) -> AudioResult<()> { self.engine.check()?; let mut state = lock(&self.data); if state.state == PlaybackState::Paused { state.state = PlaybackState::Playing; } Ok(()) }
    /// Stops playback, releases its retained clip, and resets the cursor.
    pub fn stop(&self) { lock(&self.data).stop(); }
    /// Returns playback state, reporting asynchronous device failures.
    pub fn state(&self) -> AudioResult<PlaybackState> { self.engine.check()?; Ok(lock(&self.data).state) }
    /// Checks for initialization/output failures without changing playback.
    pub fn check_device(&self) -> AudioResult<()> { self.engine.check() }
}
