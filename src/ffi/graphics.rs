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

#[repr(C)]
#[derive(Clone,Copy)]
pub struct OxideDraw2DOptions {
    pub tint: crate::graphics::Color,
    pub source: Rect,
    pub rotation: crate::math::Angle,
    pub origin: crate::math::Vec2,
    pub clip: Rect,
    pub flags: u32,
}
impl OxideDraw2DOptions {
    fn decoded(self)->Result<crate::graphics::Draw2DOptions,String> {
        if self.flags & !15 != 0 {return Err("unknown 2D option flags".into());}
        let options=crate::graphics::Draw2DOptions {
            tint:self.tint,source:if self.flags&1!=0 {Some(self.source)} else {None},rotation:self.rotation,origin:self.origin,
            clip:if self.flags&2!=0 {Some(self.clip)} else {None},flip_horizontally:self.flags&4!=0,flip_vertically:self.flags&8!=0,
        };
        options.validate().map_err(|e|e.to_string())?;Ok(options)
    }
}
fn draw2d_invalid(message:impl ToString)->OxideStatus {set_last_error(message.to_string());OxideStatus::InvalidArgument}
fn draw2d_status<T>(result:crate::graphics::GraphicsResult<T>,output:impl FnOnce(T))->OxideStatus {
    match result {Ok(value)=>{output(value);OxideStatus::Ok},Err(e)=>{set_last_error(e.to_string());OxideStatus::GraphicsError}}
}
#[unsafe(no_mangle)]
pub extern "C" fn oxide_draw_2d_options_default(result:*mut OxideDraw2DOptions)->OxideStatus {
    ffi_boundary(|| {if result.is_null(){return draw2d_invalid("result is null");}
        unsafe {*result=OxideDraw2DOptions {tint:crate::graphics::Color::WHITE,source:Rect::default(),rotation:crate::math::Angle::radians(0.0),origin:crate::math::Vec2::default(),clip:Rect::default(),flags:0};}OxideStatus::Ok
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn oxide_graphics_draw_2d(graphics:OxideGraphicsHandle,texture:OxideTextureHandle,target:Rect)->OxideStatus {
    let options=OxideDraw2DOptions {tint:crate::graphics::Color::WHITE,source:Rect::default(),rotation:crate::math::Angle::radians(0.0),origin:crate::math::Vec2::default(),clip:Rect::default(),flags:0};
    oxide_graphics_draw_2d_with_options(graphics,texture,target,options)
}
#[unsafe(no_mangle)]
pub extern "C" fn oxide_graphics_draw_2d_with_options(graphics:OxideGraphicsHandle,texture:OxideTextureHandle,target:Rect,options:OxideDraw2DOptions)->OxideStatus {
    ffi_boundary(|| {
        let options=match options.decoded(){Ok(v)=>v,Err(e)=>return draw2d_invalid(e)};
        if let Err(e)=crate::graphics::draw2d::validate_rect(target){return draw2d_invalid(e);}
        let Some(graphics)=graphics_from_handle(graphics) else {return draw2d_invalid("graphics is null");};
        let Some(result)=with_texture(texture,|texture| {
            if let Some(source)=options.source {if source.x+source.width>texture.width() as f32 || source.y+source.height>texture.height() as f32 {return Err("source exceeds texture dimensions".to_string());}}
            Ok(graphics.draw_2d_with_options(texture,target,options))
        }) else {return draw2d_invalid("texture handle is invalid");};
        match result {Ok(result)=>draw2d_status(result,|_|{}),Err(e)=>draw2d_invalid(e)}
    })
}
// UTF-8 strings are borrowed only for this call. Lengths exclude terminators.
unsafe fn label_options(text:*const u8,length:usize,family:*const u8,family_length:usize,size:f32,draw:OxideDraw2DOptions)->Result<(String,crate::graphics::TextOptions),String> {
    if (text.is_null()&&length!=0)||(family.is_null()&&family_length!=0)||length>65536||family_length>256 {return Err("invalid text or font pointer/length".into());}
    let text=if length==0 {""} else {std::str::from_utf8(unsafe {std::slice::from_raw_parts(text,length)}).map_err(|_|"text is not UTF-8")?};
    let family=if family_length==0 {"Segoe UI"} else {std::str::from_utf8(unsafe {std::slice::from_raw_parts(family,family_length)}).map_err(|_|"font is not UTF-8")?};
    let options=crate::graphics::TextOptions {font_family:family.into(),font_size:size,draw:draw.decoded()?};
    options.validate(text).map_err(|e|e.to_string())?;Ok((text.into(),options))
}
#[unsafe(no_mangle)]
pub extern "C" fn oxide_graphics_draw_text(graphics:OxideGraphicsHandle,text:*const u8,length:usize,position:crate::math::Vec2,family:*const u8,family_length:usize,size:f32,draw:OxideDraw2DOptions)->OxideStatus {
    ffi_boundary(|| {
        let (text,options)=match unsafe {label_options(text,length,family,family_length,size,draw)} {Ok(v)=>v,Err(e)=>return draw2d_invalid(e)};
        if let Err(e)=crate::graphics::draw2d::validate_rect(Rect::new(position.x,position.y,0.0,0.0)){return draw2d_invalid(e);}
        let Some(graphics)=graphics_from_handle(graphics) else {return draw2d_invalid("graphics is null");};
        draw2d_status(graphics.draw_text_with_options(&text,position,&options),|_|{})
    })
}
#[unsafe(no_mangle)]
pub extern "C" fn oxide_graphics_measure_text(graphics:OxideGraphicsHandle,text:*const u8,length:usize,family:*const u8,family_length:usize,size:f32,draw:OxideDraw2DOptions,result:*mut crate::math::Vec2)->OxideStatus {
    ffi_boundary(|| {
        let (text,options)=match unsafe {label_options(text,length,family,family_length,size,draw)} {Ok(v)=>v,Err(e)=>return draw2d_invalid(e)};
        if result.is_null(){return draw2d_invalid("result is null");}
        let Some(graphics)=graphics_from_handle(graphics) else {return draw2d_invalid("graphics is null");};
        draw2d_status(graphics.measure_text_with_options(&text,&options),|value|unsafe {*result=value})
    })
}
