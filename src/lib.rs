//! A programmatic 3D engine with a Rust API and C-compatible language bindings.
//!
//! Start with [app::Game], load assets through [content::ContentManager], and
//! submit camera, lighting, and mesh commands through [graphics::Graphics].
//! The current renderer requires 64-bit Windows and Direct3D 12.

pub mod app;
pub mod collision;
pub mod content;
pub mod graphics;
pub mod input;
pub mod math;

mod error;
mod ffi;
mod platform;

pub use error::{OxideError, OxideResult};
