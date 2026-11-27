use std::sync::{Arc, Mutex, OnceLock};
use crate::{audio::{Audio, AudioError, AudioListener, AudioResult, AudioSettings, AudioSource, SettingsValues}, math::{Angle, Vec3}};
use super::{error::{ffi_boundary, set_last_error}, registry::Registry, OxideStatus};

fn lock<T>(mutex: &Mutex<T>) -> std::sync::MutexGuard<'_, T> { mutex.lock().unwrap_or_else(|p| p.into_inner()) }
fn status(result: AudioResult<()>) -> OxideStatus {
    match result { Ok(()) => OxideStatus::Ok, Err(error) => { let code = match error { AudioError::InvalidArgument(_) => OxideStatus::InvalidArgument, AudioError::Device(_) => OxideStatus::AudioError }; set_last_error(error.to_string()); code } }
}
fn invalid() -> OxideStatus { set_last_error("audio handle is invalid or output pointer is null"); OxideStatus::InvalidArgument }

fn clips() -> &'static Mutex<Registry<Audio>> {
    static REGISTRY: OnceLock<Mutex<Registry<Audio>>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(Registry::new()))
}

fn settings() -> &'static Mutex<Registry<AudioSettings>> {
    static REGISTRY: OnceLock<Mutex<Registry<AudioSettings>>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(Registry::new()))
}

fn sources() -> &'static Mutex<Registry<Arc<AudioSource>>> {
    static REGISTRY: OnceLock<Mutex<Registry<Arc<AudioSource>>>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(Registry::new()))
}

fn listeners() -> &'static Mutex<Registry<AudioListener>> {
    static REGISTRY: OnceLock<Mutex<Registry<AudioListener>>> = OnceLock::new();
    REGISTRY.get_or_init(|| Mutex::new(Registry::new()))
}
pub(crate) fn insert_audio(audio: Audio) -> u64 { lock(clips()).insert(audio) }

#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_destroy(handle: u64) -> OxideStatus {
    ffi_boundary(|| { let removed = lock(clips()).remove(handle); if removed.is_some() { OxideStatus::Ok } else { invalid() } })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_settings_destroy(handle: u64) -> OxideStatus {
    ffi_boundary(|| { let removed = lock(settings()).remove(handle); if removed.is_some() { OxideStatus::Ok } else { invalid() } })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_source_destroy(handle: u64) -> OxideStatus {
    ffi_boundary(|| { let removed = lock(sources()).remove(handle); if removed.is_some() { OxideStatus::Ok } else { invalid() } })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_listener_destroy(handle: u64) -> OxideStatus {
    ffi_boundary(|| { let removed = lock(listeners()).remove(handle); if removed.is_some() { OxideStatus::Ok } else { invalid() } })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_create_samples(samples: *const f32, count: usize, rate: u32, result: *mut u64) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() || samples.is_null() || count == 0 || count > 64*1024*1024 { return invalid(); }
        unsafe { *result=0; }
        match Audio::from_samples(unsafe { std::slice::from_raw_parts(samples,count) },rate) {
            Ok(audio) => { unsafe { *result=insert_audio(audio); } OxideStatus::Ok }, Err(e) => status(Err(e))
        }
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_metadata(handle: u64, rate: *mut u32, count: *mut u64, duration: *mut f64) -> OxideStatus {
    ffi_boundary(|| {
        if rate.is_null() || count.is_null() || duration.is_null() { return invalid(); }
        let Some(audio)=lock(clips()).get(handle).cloned() else { return invalid(); };
        unsafe { *rate=audio.sample_rate(); *count=audio.sample_count() as u64; *duration=audio.duration().as_secs_f64(); } OxideStatus::Ok
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_settings_create(result: *mut u64) -> OxideStatus {
    ffi_boundary(|| { if result.is_null() { return invalid(); } unsafe { *result=lock(settings()).insert(AudioSettings::new()); } OxideStatus::Ok })
}
#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_settings_get(handle: u64, result: *mut SettingsValues) -> OxideStatus {
    ffi_boundary(|| { if result.is_null() { return invalid(); } let Some(value)=lock(settings()).get(handle).cloned() else { return invalid(); }; unsafe { *result=value.snapshot(); } OxideStatus::Ok })
}
#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_settings_set(handle: u64, values: SettingsValues) -> OxideStatus {
    ffi_boundary(|| { let Some(value)=lock(settings()).get(handle).cloned() else { return invalid(); }; status(value.replace(values)) })
}
#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_settings_set_distance_range(handle: u64, reference: f32, maximum: f32) -> OxideStatus {
    ffi_boundary(|| { let Some(value)=lock(settings()).get(handle).cloned() else { return invalid(); }; status(value.set_distance_range(reference,maximum)) })
}
#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_settings_set_looping(handle: u64, looping: u32) -> OxideStatus {
    ffi_boundary(|| { let Some(value)=lock(settings()).get(handle).cloned() else { return invalid(); }; if looping>1 { return status(Err(AudioError::InvalidArgument("looping must be zero or one".into()))); } value.set_looping(looping!=0); OxideStatus::Ok })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_settings_set_volume(handle: u64, value: f32) -> OxideStatus {
    ffi_boundary(|| { let Some(settings)=lock(settings()).get(handle).cloned() else { return invalid(); }; status(settings.set_volume(value)) })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_settings_set_pitch(handle: u64, value: f32) -> OxideStatus {
    ffi_boundary(|| { let Some(settings)=lock(settings()).get(handle).cloned() else { return invalid(); }; status(settings.set_pitch(value)) })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_settings_set_reference_distance(handle: u64, value: f32) -> OxideStatus {
    ffi_boundary(|| { let Some(settings)=lock(settings()).get(handle).cloned() else { return invalid(); }; status(settings.set_reference_distance(value)) })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_settings_set_max_distance(handle: u64, value: f32) -> OxideStatus {
    ffi_boundary(|| { let Some(settings)=lock(settings()).get(handle).cloned() else { return invalid(); }; status(settings.set_max_distance(value)) })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_settings_set_rolloff(handle: u64, value: f32) -> OxideStatus {
    ffi_boundary(|| { let Some(settings)=lock(settings()).get(handle).cloned() else { return invalid(); }; status(settings.set_rolloff(value)) })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_settings_set_low_pass_cutoff(handle: u64, value: f32) -> OxideStatus {
    ffi_boundary(|| { let Some(settings)=lock(settings()).get(handle).cloned() else { return invalid(); }; status(settings.set_low_pass_cutoff(value)) })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_settings_set_modulation_frequency(handle: u64, value: f32) -> OxideStatus {
    ffi_boundary(|| { let Some(settings)=lock(settings()).get(handle).cloned() else { return invalid(); }; status(settings.set_modulation_frequency(value)) })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_settings_set_modulation_depth(handle: u64, value: f32) -> OxideStatus {
    ffi_boundary(|| { let Some(settings)=lock(settings()).get(handle).cloned() else { return invalid(); }; status(settings.set_modulation_depth(value)) })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_source_create(position: Vec3, settings_handle: u64, direction: Vec3, result: *mut u64) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() { return invalid(); } unsafe { *result=0; }
        let Some(settings)=lock(settings()).get(settings_handle).cloned() else { return invalid(); };
        match AudioSource::new(position,settings,direction) {
            Ok(source) => { unsafe { *result=lock(sources()).insert(Arc::new(source)); } OxideStatus::Ok }, Err(e) => status(Err(e))
        }
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_source_play(handle: u64, clip: u64) -> OxideStatus {
    ffi_boundary(|| { let Some(source)=lock(sources()).get(handle).cloned() else { return invalid(); }; let Some(audio)=lock(clips()).get(clip).cloned() else { return invalid(); }; status(source.play(&audio)) })
}
#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_source_play_directional(handle: u64, clip: u64, inner: Angle, outer: Angle, outer_volume: f32) -> OxideStatus {
    ffi_boundary(|| { let Some(source)=lock(sources()).get(handle).cloned() else { return invalid(); }; let Some(audio)=lock(clips()).get(clip).cloned() else { return invalid(); }; status(source.play_directional(&audio,inner,outer,outer_volume)) })
}
#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_source_set_cone(handle: u64, inner: Angle, outer: Angle, outer_volume: f32) -> OxideStatus {
    ffi_boundary(|| { let Some(source)=lock(sources()).get(handle).cloned() else { return invalid(); }; status(source.set_cone(inner,outer,outer_volume)) })
}
#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_source_set_settings(handle: u64, settings_handle: u64) -> OxideStatus {
    ffi_boundary(|| { let Some(source)=lock(sources()).get(handle).cloned() else { return invalid(); }; let Some(settings)=lock(settings()).get(settings_handle).cloned() else { return invalid(); }; source.set_settings(settings); OxideStatus::Ok })
}
#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_source_get_settings(handle: u64, result: *mut u64) -> OxideStatus {
    ffi_boundary(|| { if result.is_null() { return invalid(); } let Some(source)=lock(sources()).get(handle).cloned() else { return invalid(); }; unsafe { *result=lock(settings()).insert(source.settings()); } OxideStatus::Ok })
}
#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_source_get_state(handle: u64, result: *mut u32) -> OxideStatus {
    ffi_boundary(|| { if result.is_null() { return invalid(); } let Some(source)=lock(sources()).get(handle).cloned() else { return invalid(); }; match source.state() { Ok(state) => { unsafe { *result=state as u32; } OxideStatus::Ok }, Err(e) => status(Err(e)) } })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_source_pause(handle: u64) -> OxideStatus {
    ffi_boundary(|| { let Some(source)=lock(sources()).get(handle).cloned() else { return invalid(); }; source.pause(); OxideStatus::Ok })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_source_resume(handle: u64) -> OxideStatus {
    ffi_boundary(|| { let Some(source)=lock(sources()).get(handle).cloned() else { return invalid(); }; status(source.resume()) })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_source_stop(handle: u64) -> OxideStatus {
    ffi_boundary(|| { let Some(source)=lock(sources()).get(handle).cloned() else { return invalid(); }; source.stop(); OxideStatus::Ok })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_listener_create(result: *mut u64) -> OxideStatus {
    ffi_boundary(|| { if result.is_null() { return invalid(); } unsafe { *result=lock(listeners()).insert(AudioListener::new()); } OxideStatus::Ok })
}
#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_listener_activate(handle: u64) -> OxideStatus {
    ffi_boundary(|| { let Some(listener)=lock(listeners()).get(handle).cloned() else { return invalid(); }; listener.activate(); OxideStatus::Ok })
}
#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_listener_set_orientation(handle: u64, forward: Vec3, up: Vec3) -> OxideStatus {
    ffi_boundary(|| { let Some(listener)=lock(listeners()).get(handle).cloned() else { return invalid(); }; status(listener.set_orientation(forward,up)) })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_source_get_position(handle: u64, result: *mut Vec3) -> OxideStatus {
    ffi_boundary(|| { if result.is_null() { return invalid(); } let Some(source)=lock(sources()).get(handle).cloned() else { return invalid(); }; unsafe { *result=source.position(); } OxideStatus::Ok })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_source_set_position(handle: u64, value: Vec3) -> OxideStatus {
    ffi_boundary(|| { let Some(source)=lock(sources()).get(handle).cloned() else { return invalid(); }; status(source.set_position(value)) })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_source_get_direction(handle: u64, result: *mut Vec3) -> OxideStatus {
    ffi_boundary(|| { if result.is_null() { return invalid(); } let Some(source)=lock(sources()).get(handle).cloned() else { return invalid(); }; unsafe { *result=source.direction(); } OxideStatus::Ok })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_source_set_direction(handle: u64, value: Vec3) -> OxideStatus {
    ffi_boundary(|| { let Some(source)=lock(sources()).get(handle).cloned() else { return invalid(); }; status(source.set_direction(value)) })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_listener_get_position(handle: u64, result: *mut Vec3) -> OxideStatus {
    ffi_boundary(|| { if result.is_null() { return invalid(); } let Some(listener)=lock(listeners()).get(handle).cloned() else { return invalid(); }; unsafe { *result=listener.position(); } OxideStatus::Ok })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_listener_set_position(handle: u64, value: Vec3) -> OxideStatus {
    ffi_boundary(|| { let Some(listener)=lock(listeners()).get(handle).cloned() else { return invalid(); }; status(listener.set_position(value)) })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_listener_get_direction(handle: u64, result: *mut Vec3) -> OxideStatus {
    ffi_boundary(|| { if result.is_null() { return invalid(); } let Some(listener)=lock(listeners()).get(handle).cloned() else { return invalid(); }; unsafe { *result=listener.direction(); } OxideStatus::Ok })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_audio_listener_get_up(handle: u64, result: *mut Vec3) -> OxideStatus {
    ffi_boundary(|| { if result.is_null() { return invalid(); } let Some(listener)=lock(listeners()).get(handle).cloned() else { return invalid(); }; unsafe { *result=listener.up(); } OxideStatus::Ok })
}
