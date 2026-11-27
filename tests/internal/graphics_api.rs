include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/graphics/graphics.rs"));

#[path = "graphics_api_checks.rs"]
mod checks;
