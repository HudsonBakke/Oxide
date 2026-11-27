//! Ten-second stereo positional demo. Optionally pass a WAV/MP3 path after --.
use oxide::{audio::{Audio, AudioListener, AudioSettings, AudioSource}, content::ContentManager, math::{Angle,Vec3}};
use std::{thread, time::Duration};
fn main() -> Result<(),Box<dyn std::error::Error>> {
    let audio=if let Some(path)=std::env::args().nth(1) { ContentManager::new().load::<Audio>(path)? }
        else { Audio::from_samples(&(0..48000).map(|i|(std::f32::consts::TAU*440.0*i as f32/48000.0).sin()*0.15).collect::<Vec<_>>(),48000)? };
    let listener=AudioListener::new();listener.activate();
    let settings=AudioSettings::new();settings.set_looping(true);settings.set_distance_range(1.0,20.0)?;
    let source=AudioSource::new(Vec3::new(0.0,0.0,-2.0),settings.clone(),Vec3::new(0.0,0.0,1.0))?;
    source.play(&audio)?;
    println!("Headphones: moving source. First 5 seconds omni, then a rotating cone. Live low-pass and modulation follow.");
    for frame in 0..200 {
        let t=frame as f32*0.05;source.set_position(Vec3::new(t.sin()*3.0,0.0,-2.0))?;
        source.set_direction(Vec3::new((t*2.0).sin(),0.0,(t*2.0).cos()))?;
        if frame==60 { settings.set_low_pass_cutoff(400.0)?;settings.set_pitch(0.75)?;println!("Low-pass 400 Hz, pitch 0.75"); }
        if frame==100 { source.play_directional(&audio,Angle::degrees(15.0),Angle::degrees(60.0),0.05)?; }
        if frame==130 {settings.set_low_pass_cutoff(24000.0)?;settings.set_modulation_frequency(6.0)?;settings.set_modulation_depth(0.8)?;println!("Modulation 6 Hz");}
        source.check_device()?;thread::sleep(Duration::from_millis(50));
    }
    source.pause();thread::sleep(Duration::from_millis(250));source.resume()?;thread::sleep(Duration::from_millis(250));source.stop();Ok(())
}
