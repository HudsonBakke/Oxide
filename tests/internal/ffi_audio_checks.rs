use super::*;
#[test]
fn audio_abi_checks_layout_validation_nulls_and_owned_handles_without_output() {
    assert_eq!(std::mem::size_of::<SettingsValues>(),36);
    let mut settings_handle=0;
    assert_eq!(oxide_audio_settings_create(&mut settings_handle),OxideStatus::Ok);
    let mut values=SettingsValues::default();
    assert_eq!(oxide_audio_settings_set_volume(settings_handle,f32::NAN),OxideStatus::InvalidArgument);
    assert_eq!(oxide_audio_settings_get(settings_handle,&mut values),OxideStatus::Ok);assert_eq!(values.volume,1.0);
    values.volume=0.5;values.pitch=-1.0;
    assert_eq!(oxide_audio_settings_set(settings_handle,values),OxideStatus::InvalidArgument);
    assert_eq!(oxide_audio_settings_get(settings_handle,&mut values),OxideStatus::Ok);assert_eq!(values.volume,1.0);
    assert_eq!(oxide_audio_settings_set_looping(settings_handle,2),OxideStatus::InvalidArgument);
    let mut source=999;
    assert_eq!(oxide_audio_source_create(Vec3::new(f32::NAN,0.0,0.0),settings_handle,Vec3::new(0.0,0.0,-1.0),&mut source),OxideStatus::InvalidArgument);assert_eq!(source,0);
    assert_eq!(oxide_audio_source_get_state(0,std::ptr::null_mut()),OxideStatus::InvalidArgument);
    let mut clip=0;assert_eq!(oxide_audio_create_samples([0.0,0.5].as_ptr(),2,48000,&mut clip),OxideStatus::Ok);
    let (mut rate,mut count,mut duration)=(0,0,0.0);assert_eq!(oxide_audio_metadata(clip,&mut rate,&mut count,&mut duration),OxideStatus::Ok);
    assert_eq!((rate,count),(48000,2));assert!(duration>0.0);
    assert_eq!(oxide_audio_destroy(clip),OxideStatus::Ok);assert_eq!(oxide_audio_destroy(clip),OxideStatus::InvalidArgument);
    assert_eq!(oxide_audio_settings_destroy(settings_handle),OxideStatus::Ok);
}
