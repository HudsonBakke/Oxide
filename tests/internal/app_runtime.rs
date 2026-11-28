include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/app/runtime.rs"));
#[path = "app_checks.rs"]
mod checks;

#[path = "window_checks.rs"]
mod window_checks;
