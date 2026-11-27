use crate::{
    graphics::{ShadowSettings, DirectionalLight, Environment, Graphics, PointLight, SpotLight},
    math::{Rect, Rect3D, Transform},
};

use super::{
    OxideCameraHandle, OxideGraphicsHandle, OxideMaterialHandle, OxideMeshHandle, OxideStatus,
    OxideTextureHandle,
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
pub extern "C" fn oxide_graphics_get_exposure(
    graphics: OxideGraphicsHandle,
    result: *mut f32,
) -> OxideStatus {
    ffi_boundary(|| {
        let Some(graphics) = graphics_from_handle(graphics) else {
            set_last_error("graphics handle is null");
            return OxideStatus::InvalidArgument;
        };
        if result.is_null() {
            set_last_error("result pointer cannot be null");
            return OxideStatus::InvalidArgument;
        }
        unsafe {
            *result = graphics.exposure();
        }
        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_graphics_set_exposure(
    graphics: OxideGraphicsHandle,
    exposure: f32,
) -> OxideStatus {
    ffi_boundary(|| {
        let Some(graphics) = graphics_from_handle(graphics) else {
            set_last_error("graphics handle is null");
            return OxideStatus::InvalidArgument;
        };
        match graphics.set_exposure(exposure) {
            Ok(()) => OxideStatus::Ok,
            Err(error) => {
                set_last_error(error.to_string());
                OxideStatus::InvalidArgument
            }
        }
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_graphics_draw_mesh(
    graphics: OxideGraphicsHandle,
    mesh: OxideMeshHandle,
    material: OxideMaterialHandle,
    transform: Transform,
) -> OxideStatus {
    oxide_graphics_draw_mesh_with_transparency(graphics, mesh, material, transform, 0.0)
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_graphics_draw_mesh_with_transparency(
    graphics: OxideGraphicsHandle,
    mesh: OxideMeshHandle,
    material: OxideMaterialHandle,
    transform: Transform,
    transparency: f32,
) -> OxideStatus {
    ffi_boundary(|| {
        if let Err(error) = crate::graphics::shadow::validate_transparency(transparency) {
            set_last_error(error.to_string());
            return OxideStatus::InvalidArgument;
        }
        let Some(graphics) = graphics_from_handle(graphics) else {
            set_last_error("graphics handle is null");

            return OxideStatus::InvalidArgument;
        };

        let Some(result) = with_mesh(mesh, |mesh| {
            with_material(material, |material| {
                graphics.draw_mesh_with_transparency(mesh, material, &transform, transparency)
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
    background_texture: OxideTextureHandle,
    has_background_texture: bool,
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

        let background = if has_background_texture {
            let Some(texture) = with_texture(background_texture, |texture| texture.clone()) else {
                set_last_error("background texture handle is invalid");
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
            background,
            background_color,
        ) {
            Ok(environment) => environment,
            Err(error) => {
                set_last_error(error.to_string());
                return OxideStatus::InvalidArgument;
            }
        };

        graphics.set_environment(&environment);
        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_graphics_clear_environment(graphics: OxideGraphicsHandle) -> OxideStatus {
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
        if light.casts_shadows() {
            if let Err(error) = light.shadow_settings().validate() {
                set_last_error(error.to_string());
                return OxideStatus::InvalidArgument;
            }
        }
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
        if light.casts_shadows() {
            if let Err(error) = light.shadow_settings().validate() {
                set_last_error(error.to_string());
                return OxideStatus::InvalidArgument;
            }
        }
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
        if light.casts_shadows() {
            if let Err(error) = light.shadow_settings().validate() {
                set_last_error(error.to_string());
                return OxideStatus::InvalidArgument;
            }
        }
        let Some(graphics) = graphics_from_handle(graphics) else {
            set_last_error("graphics handle is null");

            return OxideStatus::InvalidArgument;
        };

        graphics.add_light(&light);

        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_directional_shadow_settings_default(result: *mut ShadowSettings) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() {
            set_last_error("result pointer cannot be null");
            return OxideStatus::InvalidArgument;
        }
        unsafe { *result = ShadowSettings::default(); }
        OxideStatus::Ok
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_shadow_settings_default(result: *mut ShadowSettings) -> OxideStatus {
    oxide_directional_shadow_settings_default(result)
}



#[unsafe(no_mangle)]
pub extern "C" fn oxide_point_light_init(position: crate::math::Vec3, color: crate::graphics::Color, intensity: f32, result: *mut PointLight) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() { set_last_error("result pointer cannot be null"); return OxideStatus::InvalidArgument; }
        match PointLight::new(position, color, intensity) {
            Ok(light) => { unsafe { *result = light; } OxideStatus::Ok }
            Err(error) => { set_last_error(error.to_string()); OxideStatus::InvalidArgument }
        }
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_spot_light_init(position: crate::math::Vec3, direction: crate::math::Vec3, angle: crate::math::Angle, color: crate::graphics::Color, intensity: f32, result: *mut SpotLight) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() { set_last_error("result pointer cannot be null"); return OxideStatus::InvalidArgument; }
        match SpotLight::new(position, direction, angle, color, intensity) {
            Ok(light) => { unsafe { *result = light; } OxideStatus::Ok }
            Err(error) => { set_last_error(error.to_string()); OxideStatus::InvalidArgument }
        }
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_directional_light_init(direction: crate::math::Vec3, color: crate::graphics::Color, intensity: f32, result: *mut DirectionalLight) -> OxideStatus {
    ffi_boundary(|| {
        if result.is_null() { set_last_error("result pointer cannot be null"); return OxideStatus::InvalidArgument; }
        match DirectionalLight::new(direction, color, intensity) {
            Ok(light) => { unsafe { *result = light; } OxideStatus::Ok }
            Err(error) => { set_last_error(error.to_string()); OxideStatus::InvalidArgument }
        }
    })
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_graphics_bind_camera_3d(graphics: OxideGraphicsHandle, camera: OxideCameraHandle, destination: Rect3D) -> OxideStatus {
    oxide_graphics_bind_camera_3d_with_resolution(graphics, camera, destination, 0, 0)
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_graphics_bind_camera_3d_with_resolution(graphics: OxideGraphicsHandle, camera: OxideCameraHandle, destination: Rect3D, width: u32, height: u32) -> OxideStatus {
    ffi_boundary(|| {
        let destination = match destination.validated() {
            Ok(rectangle) => rectangle,
            Err(error) => { set_last_error(error.to_string()); return OxideStatus::InvalidArgument; }
        };
        let Some(graphics) = graphics_from_handle(graphics) else { set_last_error("graphics handle is null"); return OxideStatus::InvalidArgument; };
        let Some(result) = with_camera(camera, |camera| graphics.bind_camera_3d_with_resolution(camera, destination, width, height))
        else { set_last_error("camera handle is invalid"); return OxideStatus::InvalidArgument; };
        match result { Ok(()) => OxideStatus::Ok, Err(error) => { set_last_error(error.to_string()); OxideStatus::GraphicsError } }
    })
}
