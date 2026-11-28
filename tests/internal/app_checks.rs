use super::*;
use std::{cell::Cell, error::Error};
pub(super) static STATE: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[test]
fn invalid_configuration_is_reported_before_creating_an_event_loop() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<AppError>();
    struct Unreachable;
    impl Game for Unreachable { fn initialize(&mut self) { panic!("must not initialize"); } }
    for config in [AppConfig {width:0,..Default::default()},AppConfig {height:0,..Default::default()},AppConfig {title:"bad\0title".into(),..Default::default()}] {
        let error=run(Unreachable,config).unwrap_err();
        assert_eq!(error.stage(),AppStage::Configuration);
        assert!(error.to_string().contains("dimensions"));assert!(error.source().is_some());
    }
}

#[test]
fn exit_and_callback_panics_stop_later_callbacks_and_cleanup_run_state() {
    let _lock=STATE.lock().unwrap();
    for phase in 0..3 {
        let session=RunSession::acquire().unwrap();
        let mut lifecycle=Lifecycle::default();let calls=Cell::new(0);
        for index in 0..3 { lifecycle.callback(|| {calls.set(calls.get()+1);if index==phase {exit();}}); }
        assert_eq!(calls.get(),phase+1);assert!(lifecycle.should_stop());
        lifecycle.finish().unwrap();drop(session);
        assert!(!EXIT_REQUESTED.load(Ordering::Acquire));assert_eq!(window_width(),0);
    }
    let session=RunSession::acquire().unwrap();
    assert_eq!(RunSession::acquire().err().unwrap().stage(),AppStage::AlreadyRunning);
    WINDOW_WIDTH.store(640,Ordering::Relaxed);WINDOW_FOCUSED.store(true,Ordering::Relaxed);
    set_window_name("queued");let payload=std::sync::Arc::new("original panic");
    let original=payload.clone();let mut lifecycle=Lifecycle::default();
    lifecycle.callback(|| std::panic::panic_any(payload));
    lifecycle.callback(|| panic!("must not run after first panic"));
    drop(session);
    assert_eq!(window_width(),0);assert!(!is_window_focused());assert!(commands().is_empty());
    let caught=catch_unwind(AssertUnwindSafe(|| lifecycle.finish())).unwrap_err();
    assert!(std::sync::Arc::ptr_eq(caught.downcast_ref::<std::sync::Arc<&str>>().unwrap(),&original));
}

#[test]
fn frame_and_resize_errors_keep_stage_cause_and_first_failure() {
    let _lock=STATE.lock().unwrap();let _session=RunSession::acquire().unwrap();
    for stage in [AppStage::GraphicsInitialization,AppStage::BeginFrame,AppStage::EndFrame,AppStage::Resize] {
        let mut lifecycle=Lifecycle::default();
        lifecycle.graphics(stage,Err(std::io::Error::other("simulated device removal").into()));
        lifecycle.callback(|| panic!("must stop on graphics errors"));
        lifecycle.fail(AppError::new(AppStage::EventLoop,std::io::Error::other("secondary failure")));
        let error=lifecycle.finish().unwrap_err();assert_eq!(error.stage(),stage);
        assert!(error.to_string().contains("simulated device removal"));
        assert!(error.source().unwrap().source().is_some());
    }
}

#[test]
fn event_loop_thread_failure_returns_an_error_instead_of_unwinding_platform_code() {
    let _lock=STATE.lock().unwrap();
    let (stage,message)=std::thread::spawn(|| {
        struct EmptyGame;
        impl Game for EmptyGame {}
        let error=run(EmptyGame,AppConfig::default()).unwrap_err();
        (error.stage(),error.to_string())
    }).join().expect("startup panic escaped app::run");
    assert_eq!(stage,AppStage::EventLoop);
    assert!(message.contains("event loop"));
    assert!(!RUNNING.load(Ordering::Acquire));
}
