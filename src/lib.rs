pub mod app;
pub mod math;
pub mod graphics;
pub mod collision;
pub mod content;
pub mod input;

mod platform;
mod error;
mod ffi;

pub use error::{
    OxideError,
    OxideResult,
};