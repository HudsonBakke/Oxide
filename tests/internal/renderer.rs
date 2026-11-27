include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/graphics/backend/d3d12/renderer.rs"));

#[path = "renderer_checks.rs"]
mod checks;

#[path = "renderer_readback.rs"]
mod readback;
