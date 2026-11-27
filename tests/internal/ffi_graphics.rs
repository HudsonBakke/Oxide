include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/ffi/graphics.rs"));

#[path = "ffi_graphics_checks.rs"]
mod checks;
