use std::ffi::{
    c_char,
    c_void
};

use crate::{
    app::GameTime,
};

/// Opaque handle representing the Graphics object
/// valid during a draw callback.
///
/// Foreign code must never dereference this value directly.
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct OxideGraphicsHandle(
    pub *mut c_void
);

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct OxideCameraHandle(
    pub u64
);

/// Callback table supplied by foreign-language game code.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OxideGameCallbacks {
    /// Opaque value owned by the caller.
    ///
    /// Oxide stores this value and passes it back unchanged
    /// whenever a game callback is invoked.
    pub user_data:
        *mut c_void,

    pub initialize:
        Option<
            unsafe extern "C" fn(
                user_data: *mut c_void,
            )
        >,

    pub update:
        Option<
            unsafe extern "C" fn(
                user_data: *mut c_void,
                time: GameTime,
            )
        >,

    pub draw:
        Option<
            unsafe extern "C" fn(
                user_data: *mut c_void,
                graphics:
                    OxideGraphicsHandle,
            )
        >,
}

/// ABI-safe application configuration.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct OxideAppConfig {
    /// Null-terminated UTF-8 window title.
    pub title:
        *const c_char,

    pub width: u32,
    pub height: u32,
}