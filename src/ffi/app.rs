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
    error::{
        ffi_boundary,
        set_last_error,
    },
    OxideAppConfig,
    OxideGameCallbacks,
    OxideGraphicsHandle,
    OxideStatus,
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
    config: OxideAppConfig,
    callbacks: OxideGameCallbacks,
) -> OxideStatus {
    ffi_boundary(
        || {
            if config.width == 0 {
                set_last_error(
                    "application width must be greater than zero"
                );

                return OxideStatus::InvalidArgument;
            }

            if config.height == 0 {
                set_last_error(
                    "application height must be greater than zero"
                );

                return OxideStatus::InvalidArgument;
            }

            let title =
                if config.title.is_null() {
                    String::from(
                        "Oxide"
                    )
                } else {
                    let title =
                        unsafe {
                            CStr::from_ptr(
                                config.title
                            )
                        };

                    match title.to_str() {
                        Ok(title) => {
                            title.to_owned()
                        }

                        Err(_) => {
                            set_last_error(
                                "application title must be valid UTF-8"
                            );

                            return OxideStatus::InvalidArgument;
                        }
                    }
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

            OxideStatus::Ok
        }
    )
}