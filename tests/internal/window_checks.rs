use super::*;
use super::checks::STATE;
use crate::{input::{Keyboard, Keys, Mouse, MouseButton}, math::Vec2};
use winit::{dpi::PhysicalPosition,event::{ElementState,MouseButton as Button,MouseScrollDelta},keyboard::{KeyCode,PhysicalKey}};

fn focused_frame() -> FrameState {
    let mut frame=FrameState::default();frame.size(640,480);frame.focus(true);frame
}
fn press(frame:&FrameState) {
    frame.keyboard(PhysicalKey::Code(KeyCode::KeyW),ElementState::Pressed,false,false);
    frame.mouse_button(Button::Left,ElementState::Pressed);
    frame.mouse_motion((12.0,-4.0));frame.mouse_wheel(MouseScrollDelta::LineDelta(0.0,2.0));
}

#[test]
fn focus_loss_releases_all_input_including_between_update_presses_and_c_abi_snapshots() {
    let _lock=STATE.lock().unwrap();let _session=RunSession::acquire().unwrap();
    let mut frame=focused_frame();press(&frame);
    frame.cursor_moved(PhysicalPosition::new(40.0,50.0));
    assert!(Keyboard::get_state().is_key_pressed(Keys::W));
    assert!(Mouse::get_state().is_button_pressed(MouseButton::Left));
    frame.focus(false);
    let keys=Keyboard::get_state();let mouse=Mouse::get_state();
    assert!(keys.is_key_up(Keys::W));assert!(keys.is_key_released(Keys::W));
    assert!(mouse.is_button_up(MouseButton::Left));assert!(mouse.is_button_released(MouseButton::Left));
    assert_eq!(mouse.delta(),Vec2::default());assert_eq!(mouse.scroll_delta(),Vec2::default());
    assert_eq!(mouse.position(),Vec2::new(40.0,50.0));
    let mut native_keys=crate::ffi::input::OxideKeyboardState::default();
    let mut native_mouse=crate::ffi::input::OxideMouseState::default();
    assert_eq!(crate::ffi::input::oxide_keyboard_get_state(&mut native_keys),crate::ffi::OxideStatus::Ok);
    assert_eq!(crate::ffi::input::oxide_mouse_get_state(&mut native_mouse),crate::ffi::OxideStatus::Ok);
    assert_eq!(native_keys.current,[0,0]);assert_ne!(native_keys.previous[0]&(1<<(Keys::W as usize)),0);
    assert_eq!(native_mouse.current_buttons,0);assert_ne!(native_mouse.previous_buttons&1,0);
    // Background device events and repeated focus notifications cannot revive input.
    press(&frame);frame.focus(false);
    assert!(Keyboard::get_state().is_key_released(Keys::W));
    frame.focus(true);
    frame.keyboard(PhysicalKey::Code(KeyCode::KeyW),ElementState::Pressed,true,false);
    frame.keyboard(PhysicalKey::Code(KeyCode::KeyW),ElementState::Pressed,false,true);
    assert!(Keyboard::get_state().is_key_up(Keys::W));assert_eq!(Mouse::get_state().delta(),Vec2::default());
    advance_keyboard_frame();advance_mouse_frame();
    assert!(!Keyboard::get_state().is_key_released(Keys::W));assert!(!Mouse::get_state().is_button_released(MouseButton::Left));
    press(&frame);assert!(Keyboard::get_state().is_key_pressed(Keys::W));
}

#[test]
fn nonrenderable_states_gate_frames_clear_input_and_survive_redundant_transitions() {
    let _lock=STATE.lock().unwrap();let _session=RunSession::acquire().unwrap();
    for kind in 0..5 {
        let mut frame=focused_frame();press(&frame);
        for _ in 0..2 {match kind {0=>frame.size(0,480),1=>frame.size(640,0),2=>frame.minimized(true),3=>frame.occluded(true),_=>frame.suspended(true)}}
        assert!(!frame.active());assert!(!frame.accepts_input());
        frame.complete_frame(); // A skipped frame must not consume pending releases.
        assert!(Keyboard::get_state().is_key_released(Keys::W));assert!(Mouse::get_state().is_button_released(MouseButton::Left));
        press(&frame);assert!(Keyboard::get_state().is_key_up(Keys::W));assert_eq!(Mouse::get_state().delta(),Vec2::default());
        frame.size(800,600);frame.minimized(false);frame.occluded(false);frame.suspended(false);
        assert!(frame.active());assert!(frame.accepts_input());
        frame.complete_frame();
        assert!(!Keyboard::get_state().is_key_released(Keys::W));
    }
    let mut frame=focused_frame();frame.focus(false);
    assert!(frame.active(),"focus loss alone must not pause gameplay");assert!(!frame.accepts_input());
}

#[test]
fn simulation_time_excludes_suspension_and_bounds_long_stalls() {
    let start=Instant::now();let mut clock=FrameClock::default();
    let first=clock.tick(start);assert_eq!(first.delta_seconds(),0.0);assert_eq!(first.total_seconds(),0.0);
    let normal=clock.tick(start+Duration::from_millis(16));assert_eq!(normal.delta_seconds(),0.016);
    clock.pause();let restored=clock.tick(start+Duration::from_secs(3600));
    assert_eq!(restored.delta_seconds(),0.0);assert_eq!(restored.total_seconds(),0.016);
    let stalled=clock.tick(start+Duration::from_secs(3610));assert_eq!(stalled.delta_seconds(),0.1);
    assert!((stalled.total_seconds()-0.116).abs()<1e-9);
    let next=clock.tick(start+Duration::from_secs(3610)+Duration::from_millis(8));assert_eq!(next.delta_seconds(),0.008);
}

#[test]
fn resize_and_nested_pause_reasons_do_not_resume_early_or_reset_total_time() {
    let _lock=STATE.lock().unwrap();let _session=RunSession::acquire().unwrap();
    let start=Instant::now();let mut frame=focused_frame();frame.clock.tick(start);
    frame.clock.tick(start+Duration::from_millis(20));
    frame.size(800,600);assert_eq!(frame.clock.tick(start+Duration::from_secs(20)).delta_seconds(),0.0);
    frame.minimized(true);frame.suspended(true);frame.occluded(true);
    frame.minimized(false);assert!(!frame.active());
    frame.suspended(false);assert!(!frame.active());
    frame.occluded(false);assert!(frame.active());
    let restored=frame.clock.tick(start+Duration::from_secs(3600));
    assert_eq!(restored.delta_seconds(),0.0);assert_eq!(restored.total_seconds(),0.02);
    frame.suspended(false);frame.occluded(false);frame.size(800,600);
    assert_eq!(frame.clock.tick(start+Duration::from_secs(3600)+Duration::from_millis(12)).delta_seconds(),0.012);
}

#[test]
fn cursor_preferences_release_in_background_and_restore_without_losing_user_choices() {
    let policy=CursorPolicy {grabbed:true,visible:false,..Default::default()};
    assert_eq!(policy.desired(true),(true,false));assert_eq!(policy.desired(false),(false,true));
    assert_eq!(policy.desired(true),(true,false));
    let custom=CursorPolicy {grabbed:false,visible:false,..Default::default()};
    assert_eq!(custom.desired(true),(false,false));assert_eq!(custom.desired(false),(false,true));
}

#[test]
fn run_teardown_resets_input_levels_edges_motion_and_scroll() {
    let _lock=STATE.lock().unwrap();let session=RunSession::acquire().unwrap();
    let frame=focused_frame();press(&frame);advance_keyboard_frame();advance_mouse_frame();press(&frame);
    drop(session);
    let keyboard=Keyboard::get_state();let mouse=Mouse::get_state();
    assert!(keyboard.is_key_up(Keys::W));assert!(!keyboard.is_key_released(Keys::W));
    assert!(mouse.is_button_up(MouseButton::Left));assert!(!mouse.is_button_released(MouseButton::Left));
    assert_eq!(mouse.delta(),Vec2::default());assert_eq!(mouse.scroll_delta(),Vec2::default());assert_eq!(mouse.position(),Vec2::default());
}
