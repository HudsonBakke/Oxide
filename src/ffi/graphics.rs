use crate::{
    graphics::{
        DirectionalLight,
        Graphics,
        PointLight,
        SpotLight,
    },
    math::Rect,
};

use super::{
    camera::with_camera,

    error::{
        ffi_boundary,
        set_last_error,
    },

    OxideCameraHandle,
    OxideGraphicsHandle,
    OxideStatus,
};

fn graphics_from_handle<'a>(
    handle: OxideGraphicsHandle,
) -> Option<&'a mut Graphics> {
    if handle.0.is_null() {
        return None;
    }

    Some(
        unsafe {
            &mut *(
                handle.0
                    as *mut Graphics
            )
        }
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_graphics_full_rect(
    graphics: OxideGraphicsHandle,
    result: *mut Rect,
) -> OxideStatus {
    ffi_boundary(
        || {
            if result.is_null() {
                set_last_error(
                    "result pointer cannot be null"
                );

                return OxideStatus::InvalidArgument;
            }

            let Some(graphics) =
                graphics_from_handle(
                    graphics
                )
            else {
                set_last_error(
                    "graphics handle is null"
                );

                return OxideStatus::InvalidArgument;
            };

            unsafe {
                *result =
                    graphics.full_rect();
            }

            OxideStatus::Ok
        }
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_graphics_clear_lights(
    graphics: OxideGraphicsHandle,
) -> OxideStatus {
    ffi_boundary(
        || {
            let Some(graphics) =
                graphics_from_handle(
                    graphics
                )
            else {
                set_last_error(
                    "graphics handle is null"
                );

                return OxideStatus::InvalidArgument;
            };

            graphics.clear_lights();

            OxideStatus::Ok
        }
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_graphics_bind_camera(
    graphics: OxideGraphicsHandle,
    camera: OxideCameraHandle,
    destination: Rect,
) -> OxideStatus {
    ffi_boundary(
        || {
            let Some(graphics) =
                graphics_from_handle(
                    graphics
                )
            else {
                set_last_error(
                    "graphics handle is null"
                );

                return OxideStatus::InvalidArgument;
            };

            let Some(result) =
                with_camera(
                    camera,
                    |camera| {
                        graphics.bind_camera(
                            camera,
                            destination,
                        )
                    },
                )
            else {
                set_last_error(
                    "camera handle is invalid"
                );

                return OxideStatus::InvalidArgument;
            };

            match result {
                Ok(()) => {
                    OxideStatus::Ok
                }

                Err(error) => {
                    set_last_error(
                        error.to_string()
                    );

                    OxideStatus::GraphicsError
                }
            }
        }
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_graphics_add_point_light(
    graphics: OxideGraphicsHandle,
    light: PointLight,
) -> OxideStatus {
    ffi_boundary(
        || {
            let Some(graphics) =
                graphics_from_handle(
                    graphics
                )
            else {
                set_last_error(
                    "graphics handle is null"
                );

                return OxideStatus::InvalidArgument;
            };

            graphics.add_light(
                &light
            );

            OxideStatus::Ok
        }
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_graphics_add_spot_light(
    graphics: OxideGraphicsHandle,
    light: SpotLight,
) -> OxideStatus {
    ffi_boundary(
        || {
            let Some(graphics) =
                graphics_from_handle(
                    graphics
                )
            else {
                set_last_error(
                    "graphics handle is null"
                );

                return OxideStatus::InvalidArgument;
            };

            graphics.add_light(
                &light
            );

            OxideStatus::Ok
        }
    )
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_graphics_add_directional_light(
    graphics: OxideGraphicsHandle,
    light: DirectionalLight,
) -> OxideStatus {
    ffi_boundary(
        || {
            let Some(graphics) =
                graphics_from_handle(
                    graphics
                )
            else {
                set_last_error(
                    "graphics handle is null"
                );

                return OxideStatus::InvalidArgument;
            };

            graphics.add_light(
                &light
            );

            OxideStatus::Ok
        }
    )
}