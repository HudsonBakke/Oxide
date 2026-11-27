//! Internal integration coverage compiled against the production source modules.
#![allow(dead_code, unused_imports)]
#[path = "../src/app/mod.rs"]
pub mod app;
#[path = "../src/collision/mod.rs"]
pub mod collision;
#[path = "../src/content/mod.rs"]
pub mod content;
#[path = "internal/graphics.rs"]
pub mod graphics;
#[path = "../src/input/mod.rs"]
pub mod input;
#[path = "../src/math/mod.rs"]
pub mod math;
#[path = "../src/error.rs"]
mod error;
#[path = "internal/ffi.rs"]
mod ffi;
#[path = "../src/platform/mod.rs"]
mod platform;
pub use error::{OxideError, OxideResult};
