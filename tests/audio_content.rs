use oxide::{audio::Audio, content::{ContentError, ContentManager}};
use std::{fs, path::{Path,PathBuf}, time::{SystemTime,UNIX_EPOCH}};
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self { let path=std::env::temp_dir().join(format!("oxide-audio-{}-{}",std::process::id(),SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()));fs::create_dir_all(&path).unwrap();Self(path) }
    fn cache(&self) -> PathBuf { self.0.join("cache") }
}
impl Drop for Fixture { fn drop(&mut self) { let _=fs::remove_dir_all(&self.0); } }
fn wav(path: &Path, frames: usize) {
    let mut bytes=Vec::new();bytes.extend(b"RIFF");bytes.extend(&(36+frames as u32*4).to_le_bytes());bytes.extend(b"WAVEfmt ");
    bytes.extend(&16u32.to_le_bytes());bytes.extend(&1u16.to_le_bytes());bytes.extend(&2u16.to_le_bytes());
    bytes.extend(&44100u32.to_le_bytes());bytes.extend(&(44100u32*4).to_le_bytes());bytes.extend(&4u16.to_le_bytes());bytes.extend(&16u16.to_le_bytes());
    bytes.extend(b"data");bytes.extend(&(frames as u32*4).to_le_bytes());
    for _ in 0..frames {bytes.extend(&16384i16.to_le_bytes());bytes.extend(&0i16.to_le_bytes());}
    fs::write(path,bytes).unwrap();
}
#[test]
fn wav_import_cache_reuse_invalidation_and_corrupt_cache_recovery() {
    let fixture=Fixture::new();let path=fixture.0.join("tone.wav");wav(&path,100);
    let mut content=ContentManager::with_cache_directory(fixture.cache());
    let audio=content.load::<Audio>(&path).unwrap();assert_eq!(audio.sample_rate(),44100);assert_eq!(audio.sample_count(),100);
    assert!((audio.duration().as_secs_f64()-100.0/44100.0).abs()<1e-8);
    let cache=fs::read_dir(fixture.cache()).unwrap().next().unwrap().unwrap().path();
    let modified=fs::metadata(&cache).unwrap().modified().unwrap();
    assert_eq!(content.load::<Audio>(&path).unwrap().sample_count(),100);
    assert_eq!(fs::metadata(&cache).unwrap().modified().unwrap(),modified);
    wav(&path,200);assert_eq!(content.load::<Audio>(&path).unwrap().sample_count(),200);
    fs::write(&cache,b"broken cache").unwrap();assert_eq!(content.load::<Audio>(&path).unwrap().sample_count(),200);
    assert!(matches!(content.load_sub::<Audio>(&path,"track"),Err(ContentError::SubObjectNotSupported{..})));
}
#[test]
fn mp3_decodes_and_invalid_audio_reports_content_errors_without_an_output_device() {
    let fixture=Fixture::new();let path=fixture.0.join("tone.mp3");
    let hex: String=include_str!("fixtures/tone_mp3.hex").split_whitespace().collect();
    let bytes:Vec<u8>=(0..hex.len()).step_by(2).map(|i|u8::from_str_radix(&hex[i..i+2],16).unwrap()).collect();fs::write(&path,bytes).unwrap();
    let mut content=ContentManager::with_cache_directory(fixture.cache());let audio=content.load::<Audio>(&path).unwrap();
    assert_eq!(audio.sample_rate(),44100);assert!(audio.duration().as_secs_f64()>0.15);
    let bad=fixture.0.join("bad.mp3");fs::write(&bad,b"not audio").unwrap();
    assert!(matches!(content.load::<Audio>(&bad),Err(ContentError::ImportFailed{..})));
}
