use std::ffi::CStr;

use crate::{
    app::{
        self,
        AppConfig,
        Game,
        GameTime,
    },
    graphics::Graphics,
};

use super::{
    OxideGameCallbacks,
    OxideGraphicsHandle,
    OxideAppConfig,
};

struct ForeignGame {
    callbacks:
        OxideGameCallbacks,
}

impl Game for ForeignGame {
    fn initialize(&mut self) {
        let Some(callback) =
            self.callbacks.initialize
        else {
            return;
        };

        unsafe {
            callback(
                self.callbacks.user_data
            );
        }
    }

    fn update(
        &mut self,
        time: &GameTime,
    ) {
        let Some(callback) =
            self.callbacks.update
        else {
            return;
        };

        unsafe {
            callback(
                self.callbacks.user_data,
                *time,
            );
        }
    }

    fn draw(
        &mut self,
        graphics: &mut Graphics,
    ) {
        let Some(callback) =
            self.callbacks.draw
        else {
            return;
        };

        let graphics_handle =
            OxideGraphicsHandle(
                graphics
                    as *mut Graphics
                    as *mut std::ffi::c_void
            );

        unsafe {
            callback(
                self.callbacks.user_data,
                graphics_handle,
            );
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn oxide_run(
    config:
        OxideAppConfig,

    callbacks:
        OxideGameCallbacks,
) {
    let title =
        if config.title.is_null() {
            String::from("Oxide")
        } else {
            unsafe {
                CStr::from_ptr(
                    config.title
                )
            }
            .to_string_lossy()
            .into_owned()
        };

    let config =
        AppConfig {
            title,
            width:
                config.width,
            height:
                config.height,
        };

    let game =
        ForeignGame {
            callbacks,
        };

    app::run(
        game,
        config,
    );
}