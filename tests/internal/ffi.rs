#[path = "ffi_audio.rs"]
mod audio;
#[path = "../../src/ffi/app.rs"]
mod app;
#[path = "../../src/ffi/app_control.rs"]
mod app_control;
#[path = "../../src/ffi/attachment.rs"]
mod attachment;
#[path = "../../src/ffi/camera.rs"]
mod camera;
#[path = "../../src/ffi/collision.rs"]
mod collision;
#[path = "../../src/ffi/content.rs"]
mod content;
#[path = "../../src/ffi/error.rs"]
mod error;
#[path = "ffi_graphics.rs"]
mod graphics;
#[path = "../../src/ffi/input.rs"]
mod input;
#[path = "../../src/ffi/registry.rs"]
mod registry;
#[path = "../../src/ffi/resources.rs"]
mod resources;
#[path = "../../src/ffi/types.rs"]
mod types;

pub use error::OxideStatus;

pub use types::{
    OxideAppConfig, OxideCameraHandle, OxideContentManagerHandle, OxideGameCallbacks,
    OxideGraphicsHandle, OxideMaterialHandle, OxideMeshHandle, OxideTextureHandle,
};
