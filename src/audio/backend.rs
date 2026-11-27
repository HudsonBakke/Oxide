use std::{collections::VecDeque, sync::{Arc, Mutex, OnceLock, Weak, atomic::{AtomicBool, Ordering}, mpsc}, thread::{self, JoinHandle}, time::Duration};
use windows::{core::{Interface, HRESULT, PCWSTR}, Win32::{Media::Audio::{XAudio2::*, WAVEFORMATEX, AudioCategory_GameEffects}, System::Com::*}};
use super::{AudioError, AudioResult, listener, lock, mixer::{BLOCK_FRAMES, SAMPLE_RATE, mix_source}, source::SourceState};

// The SDK's XAudio2Create inline wrapper is absent from windows-rs. Import the
// Windows 10+ system entry point directly; all voice methods use generated bindings.
#[link(name = "xaudio2_9")]
unsafe extern "system" { fn XAudio2Create(result: *mut *mut std::ffi::c_void, flags: u32, processor: u32) -> HRESULT; }

struct ComApartment;
impl ComApartment {
    fn new() -> windows::core::Result<Self> { unsafe { CoInitializeEx(None, COINIT_MULTITHREADED).ok()?; } Ok(Self) }
}
impl Drop for ComApartment { fn drop(&mut self) { unsafe { CoUninitialize(); } } }

struct Output {
    engine: IXAudio2,
    master: Option<IXAudio2MasteringVoice>, voice: Option<IXAudio2SourceVoice>,
    pending: VecDeque<Box<[f32]>>, free: Vec<Box<[f32]>>,
}
impl Output {
    fn new() -> windows::core::Result<Self> {
        let mut pointer = std::ptr::null_mut();
        unsafe { XAudio2Create(&mut pointer, 0, XAUDIO2_DEFAULT_PROCESSOR).ok()?; }
        let engine: IXAudio2 = unsafe { Interface::from_raw(pointer) };
        let mut output = Self { engine, master: None, voice: None, pending: VecDeque::new(),
            free: (0..3).map(|_| vec![0.0; BLOCK_FRAMES * 2].into_boxed_slice()).collect() };
        unsafe {
            output.engine.CreateMasteringVoice(&mut output.master, 2, SAMPLE_RATE, 0, PCWSTR::null(), None, AudioCategory_GameEffects)?;
            let format = WAVEFORMATEX { wFormatTag: 3, nChannels: 2, nSamplesPerSec: SAMPLE_RATE,
                nAvgBytesPerSec: SAMPLE_RATE * 8, nBlockAlign: 8, wBitsPerSample: 32, cbSize: 0 };
            output.engine.CreateSourceVoice(&mut output.voice, &format, 0, 1.0, None, None, None)?;
            output.voice.as_ref().unwrap().Start(0, XAUDIO2_COMMIT_NOW)?;
        }
        Ok(output)
    }
    fn next_buffer(&mut self) -> Option<Box<[f32]>> {
        let mut state = XAUDIO2_VOICE_STATE::default();
        unsafe { self.voice.as_ref().unwrap().GetState(&mut state, XAUDIO2_VOICE_NOSAMPLESPLAYED); }
        // Completed buffers are returned in FIFO order. Never touch memory still
        // referenced by XAudio2; polling avoids allocating native callbacks.
        while self.pending.len() > state.BuffersQueued as usize { self.free.push(self.pending.pop_front().unwrap()); }
        if state.BuffersQueued < 3 { self.free.pop() } else { None }
    }
    fn submit(&mut self, mut samples: Box<[f32]>) -> windows::core::Result<()> {
        for value in samples.iter_mut() { *value = value.clamp(-1.0,1.0); }
        let buffer = XAUDIO2_BUFFER { AudioBytes: (samples.len()*4) as u32,
            pAudioData: samples.as_ptr() as *const u8, ..Default::default() };
        unsafe { self.voice.as_ref().unwrap().SubmitSourceBuffer(&buffer, None)?; }
        self.pending.push_back(samples);
        Ok(())
    }
}
impl Drop for Output {
    fn drop(&mut self) {
        unsafe {
            self.engine.StopEngine();
            if let Some(voice) = self.voice.take() { voice.DestroyVoice(); }
            if let Some(master) = self.master.take() { master.DestroyVoice(); }
        }
        // DestroyVoice returns only after the engine has stopped reading buffers.
    }
}

struct Shared { sources: Mutex<Vec<Weak<Mutex<SourceState>>>>, shutdown: AtomicBool, error: Mutex<Option<String>> }
pub(crate) struct Engine { shared: Arc<Shared>, worker: Mutex<Option<JoinHandle<()>>> }
static ENGINE: OnceLock<Mutex<Weak<Engine>>> = OnceLock::new();
impl Engine {
    pub(crate) fn shared() -> AudioResult<Arc<Self>> {
        let mut slot = lock(ENGINE.get_or_init(|| Mutex::new(Weak::new())));
        if let Some(engine) = slot.upgrade() { engine.check()?; return Ok(engine); }
        let shared = Arc::new(Shared { sources: Mutex::new(Vec::new()), shutdown: AtomicBool::new(false), error: Mutex::new(None) });
        let thread_shared = shared.clone();
        let (sender, receiver) = mpsc::sync_channel(1);
        let worker = thread::Builder::new().name("oxide-audio".into()).spawn(move || {
            let run = || -> windows::core::Result<()> {
                let _com = ComApartment::new()?;
                let mut output = Output::new()?;
                let _ = sender.send(Ok::<(), String>(()));
                while !thread_shared.shutdown.load(Ordering::Acquire) {
                    while let Some(mut buffer) = output.next_buffer() {
                        buffer.fill(0.0);
                        let listener = listener::snapshot();
                        let sources: Vec<_> = {
                            let mut list = lock(&thread_shared.sources);
                            list.retain(|s| s.strong_count() > 0);
                            list.iter().filter_map(Weak::upgrade).collect()
                        };
                        for source in sources { mix_source(&mut lock(&source), &listener, &mut buffer); }
                        output.submit(buffer)?;
                    }
                    thread::sleep(Duration::from_millis(2));
                }
                Ok(())
            };
            if let Err(error) = run() {
                let message = error.to_string();
                *lock(&thread_shared.error) = Some(message.clone());
                let _ = sender.send(Err(message));
                for source in lock(&thread_shared.sources).iter().filter_map(Weak::upgrade) { lock(&source).stop(); }
            }
        }).map_err(|e| AudioError::Device(e.to_string()))?;
        match receiver.recv() {
            Ok(Ok(())) => {
                let engine = Arc::new(Self { shared, worker: Mutex::new(Some(worker)) });
                *slot = Arc::downgrade(&engine); Ok(engine)
            }
            error => { let _ = worker.join(); Err(AudioError::Device(match error { Ok(Err(message)) => message, _ => "audio worker terminated during initialization".into() })) }
        }
    }
    pub(crate) fn check(&self) -> AudioResult<()> {
        if let Some(message) = lock(&self.shared.error).as_ref() { Err(AudioError::Device(message.clone())) } else { Ok(()) }
    }
    pub(crate) fn register(&self, source: &Arc<Mutex<SourceState>>) -> AudioResult<()> {
        self.check()?; lock(&self.shared.sources).push(Arc::downgrade(source)); Ok(())
    }
}
impl Drop for Engine {
    fn drop(&mut self) {
        self.shared.shutdown.store(true, Ordering::Release);
        if let Some(worker) = lock(&self.worker).take() { let _ = worker.join(); }
    }
}
