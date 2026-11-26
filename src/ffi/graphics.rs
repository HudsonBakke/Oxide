use crate::{
    graphics::{DirectionalLight, Environment, Graphics, PointLight, SpotLight},
    math::{Rect, Transform},
};

use super::{
    OxideCameraHandle, OxideGraphicsHandle, OxideMaterialHandle, OxideMeshHandle, OxideStatus, OxideTextureHandle,
    camera::with_camera,
    error::{ffi_boundary, set_last_error},
    resources::{with_material, with_mesh, with_texture},
};

fn graphics_from_handle<'a>(handle: OxideGraphicsHandle) -> Option<&'a mut Graphics> {
    if handle.0.is_null() {
        return None;
    }

    Some(unsafe { &mut *(handle.0 as *mut Graphics) })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_graphics_draw_mesh(
    graphics: OxideGraphicsHandle,
    mesh: OxideMeshHandle,
    material: OxideMaterialHandle,
    transform: Transform,
) -> OxideStatus {
    ffi_boundary(|| {
        let Some(graphics) = graphics_from_handle(graphics) else {
            set_last_error("graphics handle is null");

            return OxideStatus::InvalidArgument;
        };

        let Some(result) = with_mesh(mesh, |mesh| {
            with_material(material, |material| {
                graphics.draw_mesh(mesh, material, &transform)
            })
        }) else {
            set_last_error("mesh handle is invalid");

            return OxideStatus::InvalidArgument;
        };

        let Some(result) = result else {
            set_last_error("material handle is invalid");

            return OxideStatus::InvalidArgument;
        };

        match result {
            Ok(()) => OxideStatus::Ok,

            Err(error) => {
                set_last_error(error.to_string());

                OxideStatus::GraphicsError
            }
        }
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_graphics_full_rect(
    graphics: OxideGraphicsHandle,
    result: *mut Rect,
) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() {
            set_last_error("result pointer cannot be null");

            return OxideStatus::InvalidArgument;
        }

        let Some(graphics) = graphics_from_handle(graphics) else {
            set_last_error("graphics handle is null");

            return OxideStatus::InvalidArgument;
        };

        unsafe {
            *result = graphics.full_rect();
        }

        OxideStatus::Ok
    })
}



#[unsafe(no_mangle)]
pub extern "C" fn oxide_graphics_set_environment_explicit(
    graphics: OxideGraphicsHandle,
    ambient_color: crate::graphics::Color,
    ambient_intensity: f32,
    environment_texture: OxideTextureHandle,
    has_environment_texture: bool,
    environment_intensity: f32,
    background_color: crate::graphics::Color,
) -> OxideStatus {
    ffi_boundary(|| {
        let Some(graphics) = graphics_from_handle(graphics) else {
            set_last_error("graphics handle is null");
            return OxideStatus::InvalidArgument;
        };

        let texture = if has_environment_texture {
            let Some(texture) = with_texture(environment_texture, |texture| texture.clone()) else {
                set_last_error("environment texture handle is invalid");
                return OxideStatus::InvalidArgument;
            };
            Some(texture)
        } else {
            None
        };

        let environment = match Environment::new(
            ambient_color,
            ambient_intensity,
            texture,
            environment_intensity,
            background_color,
        ) {
            Ok(environment) => environment,
            Err(error) => {
                set_last_error(error.to_string());
                return OxideStatus::InvalidArgument;
            }
        };

        graphics.set_environment(environment);
        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_graphics_clear_environment(
    graphics: OxideGraphicsHandle,
) -> OxideStatus {
    ffi_boundary(|| {
        let Some(graphics) = graphics_from_handle(graphics) else {
            set_last_error("graphics handle is null");
            return OxideStatus::InvalidArgument;
        };

        graphics.clear_environment();
        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_graphics_clear_lights(graphics: OxideGraphicsHandle) -> OxideStatus {
    ffi_boundary(|| {
        let Some(graphics) = graphics_from_handle(graphics) else {
            set_last_error("graphics handle is null");

            return OxideStatus::InvalidArgument;
        };

        graphics.clear_lights();

        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_graphics_bind_camera(
    graphics: OxideGraphicsHandle,
    camera: OxideCameraHandle,
    destination: Rect,
) -> OxideStatus {
    ffi_boundary(|| {
        let Some(graphics) = graphics_from_handle(graphics) else {
            set_last_error("graphics handle is null");

            return OxideStatus::InvalidArgument;
        };

        let Some(result) = with_camera(camera, |camera| graphics.bind_camera(camera, destination))
        else {
            set_last_error("camera handle is invalid");

            return OxideStatus::InvalidArgument;
        };

        match result {
            Ok(()) => OxideStatus::Ok,

            Err(error) => {
                set_last_error(error.to_string());

                OxideStatus::GraphicsError
            }
        }
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_graphics_add_point_light(
    graphics: OxideGraphicsHandle,
    light: PointLight,
) -> OxideStatus {
    ffi_boundary(|| {
        let Some(graphics) = graphics_from_handle(graphics) else {
            set_last_error("graphics handle is null");

            return OxideStatus::InvalidArgument;
        };

        graphics.add_light(&light);

        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_graphics_add_spot_light(
    graphics: OxideGraphicsHandle,
    light: SpotLight,
) -> OxideStatus {
    ffi_boundary(|| {
        let Some(graphics) = graphics_from_handle(graphics) else {
            set_last_error("graphics handle is null");

            return OxideStatus::InvalidArgument;
        };

        graphics.add_light(&light);

        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_graphics_add_directional_light(
    graphics: OxideGraphicsHandle,
    light: DirectionalLight,
) -> OxideStatus {
    ffi_boundary(|| {
        let Some(graphics) = graphics_from_handle(graphics) else {
            set_last_error("graphics handle is null");

            return OxideStatus::InvalidArgument;
        };

        graphics.add_light(&light);

        OxideStatus::Ok
    })
}
