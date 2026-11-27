use super::*;
use super::super::{source::{Cone, SourceState}, Audio, AudioListener, AudioSettings};
use crate::{ content::{ContentAssetInternal, ContentError}, math::{Angle, Vec3}};

fn source(samples: &[f32], rate: u32) -> SourceState {
    let mut source = SourceState::new(Vec3::default(), AudioSettings::new(), Vec3::new(0.0,0.0,-1.0));
    source.play(&Audio::from_samples(samples,rate).unwrap(),None); source
}
fn listener() -> ListenerData { ListenerData::default() }
fn energy(samples: &[f32]) -> f32 { samples.iter().map(|s| s*s).sum::<f32>() / samples.len() as f32 }

#[test]
fn spatial_audio_respects_distance_pan_listener_rotation_and_cone_edges() {
    let mut source = source(&[1.0;1000],48_000);
    let settings = source.settings.snapshot();
    let center = spatial_gain(&source,&listener(),&settings);
    assert!((center[0]-std::f32::consts::FRAC_1_SQRT_2).abs()<1e-6);
    source.position = Vec3::new(1.0,0.0,0.0);
    let right = spatial_gain(&source,&listener(),&settings);
    assert_eq!(right,[0.0,1.0]);
    let reversed = ListenerData { forward: Vec3::new(0.0,0.0,1.0), ..listener() };
    assert_eq!(spatial_gain(&source,&reversed,&settings),[1.0,0.0]);
    source.position = Vec3::new(settings.max_distance,0.0,0.0);
    assert_eq!(spatial_gain(&source,&listener(),&settings),[0.0;2]);
    source.position = Vec3::new(2.0,0.0,0.0);
    assert!(spatial_gain(&source,&listener(),&settings)[1]<0.5);
    source.position = Vec3::default(); source.cone = Some(Cone::new(Angle::degrees(20.0),Angle::degrees(40.0),0.2).unwrap());
    for (angle,expected) in [(0.0,1.0),(30.0,0.6),(90.0,0.2)] {
        let radians = (angle as f32).to_radians();
        let listener = ListenerData { position: Vec3::new(radians.sin(),0.0,-radians.cos()), ..listener() };
        let gain = spatial_gain(&source,&listener,&settings);
        assert!(((gain[0]*gain[0]+gain[1]*gain[1]).sqrt()-expected).abs()<1e-5);
    }
}

#[test]
fn mixer_advances_resamples_loops_and_releases_completed_clips() {
    let mut source = source(&[0.25;100],24_000);
    let mut output=[0.0;100];
    mix_source(&mut source,&listener(),&mut output);
    assert_eq!(source.cursor,25.0);
    source.settings.set_pitch(2.0).unwrap();
    mix_source(&mut source,&listener(),&mut output);
    assert!(source.cursor>62.0 && source.cursor<64.0);
    source.state = PlaybackState::Paused; let cursor=source.cursor;
    mix_source(&mut source,&listener(),&mut output); assert_eq!(source.cursor,cursor);
    source.state = PlaybackState::Playing;
    source.settings.set_looping(true);
    for _ in 0..10 { mix_source(&mut source,&listener(),&mut output); }
    assert_eq!(source.state,PlaybackState::Playing); assert!(source.audio.is_some());
    source.settings.set_looping(false);
    for _ in 0..10 { mix_source(&mut source,&listener(),&mut output); }
    assert_eq!(source.state,PlaybackState::Stopped); assert!(source.audio.is_none());
    let replacement=Audio::from_samples(&[0.5;20],48_000).unwrap();
    source.play(&replacement,None); drop(replacement);
    assert_eq!(source.cursor,0.0); assert_eq!(source.audio.as_ref().unwrap().sample_count(),20);
    source.stop(); assert!(source.audio.is_none());
}

#[test]
fn shared_settings_apply_live_and_reject_invalid_changes_atomically() {
    let mut source=source(&[0.5;4000],48_000);
    let shared=source.settings.clone(); let mut output=[0.0;960];
    mix_source(&mut source,&listener(),&mut output);
    shared.set_volume(0.0).unwrap(); output.fill(0.0);
    mix_source(&mut source,&listener(),&mut output);
    assert_eq!(source.gain,[0.0;2]); assert!(output[2].abs()>output[958].abs());
    let old=shared.snapshot();
    for value in [f32::NAN,f32::INFINITY,-1.0,17.0] { assert!(shared.set_volume(value).is_err()); }
    assert_eq!(shared.volume(),old.volume);
    assert!(shared.set_distance_range(10.0,2.0).is_err());
    assert_eq!(shared.reference_distance(),old.reference_distance);
    assert!(Cone::new(Angle::degrees(60.0),Angle::degrees(20.0),0.0).is_err());
    let listener=AudioListener::new();
    assert!(listener.set_orientation(Vec3::new(0.0,1.0,0.0),Vec3::new(0.0,2.0,0.0)).is_err());
    assert_eq!(listener.direction(),Vec3::new(0.0,0.0,-1.0));
}

#[test]
fn low_pass_and_amplitude_modulation_change_the_mixed_signal() {
    let samples: Vec<_>=(0..9600).map(|i| if i%2==0 {0.5} else {-0.5}).collect();
    let mut dry=source(&samples,48_000); let mut wet=source(&samples,48_000);
    wet.settings.set_low_pass_cutoff(100.0).unwrap();
    let mut a=vec![0.0;9600]; let mut b=vec![0.0;9600];
    mix_source(&mut dry,&listener(),&mut a); mix_source(&mut wet,&listener(),&mut b);
    a.fill(0.0); b.fill(0.0);
    mix_source(&mut dry,&listener(),&mut a); mix_source(&mut wet,&listener(),&mut b);
    assert!(energy(&b[4800..])<energy(&a[4800..])*0.01);
    let mut plain=source(&[0.5;9600],48_000); let mut modulated=source(&[0.5;9600],48_000);
    modulated.settings.set_modulation_frequency(20.0).unwrap(); modulated.settings.set_modulation_depth(1.0).unwrap();
    a.fill(0.0); b.fill(0.0); mix_source(&mut plain,&listener(),&mut a); mix_source(&mut modulated,&listener(),&mut b);
    a.fill(0.0); b.fill(0.0); mix_source(&mut plain,&listener(),&mut a); mix_source(&mut modulated,&listener(),&mut b);
    assert!(energy(&b)<energy(&a)*0.6);
}

#[test]
fn audio_cache_roundtrips_and_rejects_nonfinite_truncated_or_oversized_payloads() {
    let audio=Audio::from_samples(&[0.0,0.5,-1.0],44_100).unwrap();
    let mut bytes=Vec::new(); audio.write_compiled(&mut bytes).unwrap();
    let restored=Audio::read_compiled(&mut bytes.as_slice(),bytes.len() as u64).unwrap();
    assert_eq!(restored.sample_rate(),44_100); assert_eq!(restored.samples.as_ref(),audio.samples.as_ref());
    assert!(Audio::read_compiled(&mut bytes[..bytes.len()-1].as_ref(),bytes.len() as u64).is_err());
    bytes[12..16].copy_from_slice(&f32::NAN.to_le_bytes());
    assert!(matches!(Audio::read_compiled(&mut bytes.as_slice(),bytes.len() as u64),Err(ContentError::InvalidCache(_))));
    assert!(Audio::from_samples(&[],48_000).is_err()); assert!(Audio::from_samples(&[1.1],48_000).is_err());
}
