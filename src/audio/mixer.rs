use super::{listener::ListenerData, settings::SettingsValues, source::{SourceState, PlaybackState}};

pub(crate) const SAMPLE_RATE: u32 = 48_000;
pub(crate) const BLOCK_FRAMES: usize = 480;

pub(crate) fn spatial_gain(source: &SourceState, listener: &ListenerData, settings: &SettingsValues) -> [f32;2] {
    let offset = source.position - listener.position;
    let distance = offset.length();
    let attenuation = if distance >= settings.max_distance { 0.0 }
        else if distance <= settings.reference_distance { 1.0 }
        else {
            (settings.reference_distance / distance).powf(settings.rolloff)
                * (settings.max_distance - distance) / (settings.max_distance - settings.reference_distance)
        };
    let mut cone_gain = 1.0;
    if let Some(cone) = source.cone {
        if distance > 1.0e-6 {
            let angle = source.direction.dot(&(offset * (-1.0 / distance))).clamp(-1.0,1.0).acos();
            cone_gain = if angle <= cone.inner { 1.0 } else if angle >= cone.outer { cone.outer_volume }
                else { 1.0 + (cone.outer_volume - 1.0) * (angle - cone.inner) / (cone.outer - cone.inner) };
        }
    }
    let pan = if distance <= 1.0e-6 { 0.0 }
        else { (offset / distance).dot(&listener.forward.cross(&listener.up)).clamp(-1.0,1.0) };
    let gain = settings.volume * attenuation * cone_gain;
    [((1.0-pan)*0.5).sqrt()*gain, ((1.0+pan)*0.5).sqrt()*gain]
}

/// Mixes one source into an interleaved stereo block. Source controls are locked
/// once per block; no user code, decoding, or filesystem work runs in this path.
pub(crate) fn mix_source(source: &mut SourceState, listener: &ListenerData, output: &mut [f32]) {
    if source.state != PlaybackState::Playing { return; }
    let Some(audio) = source.audio.as_ref() else { return; };
    let settings = source.settings.snapshot();
    let target = spatial_gain(source, listener, &settings);
    let target_filter = if settings.low_pass_cutoff >= SAMPLE_RATE as f32 * 0.5 { 1.0 }
        else { 1.0 - (-std::f32::consts::TAU * settings.low_pass_cutoff / SAMPLE_RATE as f32).exp() };
    let frames = output.len() / 2;
    if frames == 0 { return; }
    let old_gain = source.gain;
    let old_pitch = source.pitch;
    let old_filter = source.filter;
    for (index, frame) in output.chunks_exact_mut(2).enumerate() {
        if source.cursor >= audio.samples.len() as f64 {
            if settings.looping != 0 { source.cursor %= audio.samples.len() as f64; }
            else { source.state = PlaybackState::Stopped; break; }
        }
        let t = (index + 1) as f32 / frames as f32;
        let first = source.cursor as usize;
        let next = if first + 1 < audio.samples.len() { first + 1 } else if settings.looping != 0 { 0 } else { first };
        let fraction = source.cursor.fract() as f32;
        let sample = audio.samples[first] + (audio.samples[next] - audio.samples[first]) * fraction;
        source.filtered += (sample - source.filtered) * (old_filter + (target_filter - old_filter) * t);
        let modulation = if settings.modulation_frequency == 0.0 || settings.modulation_depth == 0.0 { 1.0 }
            else { 1.0 - settings.modulation_depth * (0.5 - 0.5 * (source.phase as f32).cos()) };
        let value = source.filtered * modulation;
        for channel in 0..2 { frame[channel] += value * (old_gain[channel] + (target[channel] - old_gain[channel]) * t); }
        source.cursor += audio.sample_rate() as f64 / SAMPLE_RATE as f64 * (old_pitch + (settings.pitch - old_pitch) * t) as f64;
        source.phase = (source.phase + std::f64::consts::TAU * settings.modulation_frequency as f64 / SAMPLE_RATE as f64) % std::f64::consts::TAU;
    }
    source.gain = target; source.pitch = settings.pitch; source.filter = target_filter;
    if settings.looping == 0 && source.cursor >= audio.samples.len() as f64 { source.state = PlaybackState::Stopped; }
    if source.state == PlaybackState::Stopped { source.audio = None; source.cursor = 0.0; source.gain = [0.0;2]; }
}
