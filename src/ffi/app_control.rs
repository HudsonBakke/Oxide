use std::ffi::CStr;

use crate::app;

use super::{
    OxideStatus,
    OxideTextureHandle,
    error::{
        ffi_boundary,
        set_last_error,
    },
    resources::with_texture,
};

#[unsafe(no_mangle)]
pub extern "C" fn oxide_app_exit() -> OxideStatus {
    ffi_boundary(|| {
        app::exit();
        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn oxide_app_set_window_name(
    name: *const std::ffi::c_char,
) -> OxideStatus {
    ffi_boundary(|| {
        if name.is_null() {
            set_last_error("window name cannot be null");
            return OxideStatus::InvalidArgument;
        }

        let name = unsafe { CStr::from_ptr(name) };

        let Ok(name) = name.to_str() else {
            set_last_error("window name must be valid UTF-8");
            return OxideStatus::InvalidArgument;
        };

        app::set_window_name(name);
        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_app_set_window_logo(
    texture: OxideTextureHandle,
) -> OxideStatus {
    ffi_boundary(|| {
        let Some(()) = with_texture(texture, |texture| {
            app::set_window_logo(texture);
        }) else {
            set_last_error("texture handle is invalid");
            return OxideStatus::InvalidArgument;
        };

        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_app_set_preferred_backbuffer_size(
    width: u32,
    height: u32,
) -> OxideStatus {
    ffi_boundary(|| {
        if width == 0 || height == 0 {
            set_last_error("backbuffer dimensions must be greater than zero");
            return OxideStatus::InvalidArgument;
        }

        app::set_preferred_backbuffer_size(width, height);
        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_app_window_width() -> u32 {
    app::window_width()
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_app_window_height() -> u32 {
    app::window_height()
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_app_is_window_focused() -> u8 {
    u8::from(app::is_window_focused())
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_app_set_fullscreen(
    fullscreen: u8,
) -> OxideStatus {
    ffi_boundary(|| {
        app::set_fullscreen(fullscreen != 0);
        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_app_is_fullscreen() -> u8 {
    u8::from(app::is_fullscreen())
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_app_set_cursor_visible(
    visible: u8,
) -> OxideStatus {
    ffi_boundary(|| {
        app::set_cursor_visible(visible != 0);
        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_app_set_cursor_grabbed(
    grabbed: u8,
) -> OxideStatus {
    ffi_boundary(|| {
        app::set_cursor_grabbed(grabbed != 0);
        OxideStatus::Ok
    })
}
