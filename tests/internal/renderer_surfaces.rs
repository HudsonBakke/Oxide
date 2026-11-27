use super::*;
use super::readback::{quad, transform, headless_renderer, start_headless_frame, finish_headless_frame_samples};
use crate::graphics::{Color, Environment};

fn rectangle(normal: f32) -> Rect3D {
    Rect3D::new(Vec3::new(-1.5,1.5,1.0),Vec3::new(1.5,1.5,1.0),Vec3::new(1.5,-1.5,1.0),Vec3::new(-1.5,-1.5,1.0),Vec3::new(0.0,0.0,normal)).unwrap()
}
fn emissive(color: Color) -> Material {
    let mut material=Material::solid(Color::BLACK);material.set_emissive_color(color);material.set_emissive_strength(1.0).unwrap();material
}
fn assert_color(actual: [f32;3], expected: [f32;3]) {
    assert!(actual.into_iter().zip(expected).all(|(a,b)|(a-b).abs()<0.004),"camera surface pixel {actual:?} != {expected:?}");
}
#[test]
fn camera_surfaces_render_current_feeds_in_either_order_and_obey_depth_transparency_and_front_side() {
    let device=D3D12Device::warp_for_test().unwrap();
    let (mut renderer,depth)=headless_renderer(&device);
    let mesh=quad();let red=emissive(Color::RED);let blue=emissive(Color::BLUE);let green=emissive(Color::GREEN);
    let mut feed=Camera::new();feed.set_position(Vec3::new(8.0,0.0,5.0));
    let mut screen=Camera::new();screen.set_position(Vec3::new(0.0,0.0,5.0));
    for frame in 0..6 {
        start_headless_frame(&mut renderer,&device,&depth,frame==0);
        let submit_feed=|renderer: &mut D3D12Renderer| {
            renderer.bind_camera_3d(&device,&feed,rectangle(if frame==4 {-1.0} else {1.0}),64,64).unwrap();
            renderer.draw_mesh(&device,&mesh,&red,&transform(8.0,0.0,4.0,4.0),0.0).unwrap();
            // Feed-only geometry in front of the player's monitor must not leak
            // into the screen camera's command group.
            renderer.draw_mesh(&device,&mesh,&green,&transform(0.0,2.0,4.0,4.0),0.0).unwrap();
        };
        if frame%2==1 { submit_feed(&mut renderer); }
        renderer.bind_camera(&device,&screen,Rect::new(0.0,0.0,64.0,64.0)).unwrap();
        renderer.draw_mesh(&device,&mesh,&blue,&transform(0.0,0.0,4.0,4.0),0.0).unwrap();
        if frame==2 || frame==3 {
            renderer.draw_mesh(&device,&mesh,&green,&transform(0.0,2.0,4.0,4.0),if frame==3 {0.5} else {0.0}).unwrap();
        }
        if frame%2==0 { submit_feed(&mut renderer); }
        let descriptor=renderer.camera_surfaces[0].descriptor;
        let actual=finish_headless_frame_samples(&mut renderer,&device,&depth,&[(32,32)])[0];
        assert_color(actual,match frame {2=>[0.0,1.0,0.0],3=>[0.5,0.5,0.0],4=>[0.0,0.0,1.0],_=>[1.0,0.0,0.0]});
        assert_eq!(renderer.camera_surfaces.len(),1);
        assert_eq!(renderer.camera_surfaces[0].descriptor,descriptor);
    }
    start_headless_frame(&mut renderer,&device,&depth,false);
    finish_headless_frame_samples(&mut renderer,&device,&depth,&[(32,32)]);
    assert!(renderer.camera_surfaces.is_empty());
}

#[test]
fn camera_surface_images_preserve_uv_orientation_and_linear_hdr_in_multiple_screen_views() {
    let device=D3D12Device::warp_for_test().unwrap();let (mut renderer,depth)=headless_renderer(&device);
    start_headless_frame(&mut renderer,&device,&depth,true);
    let mesh=quad();let red=emissive(Color::RED);let green=emissive(Color::GREEN);let blue=emissive(Color::BLUE);
    let mut feed=Camera::new();feed.set_position(Vec3::new(0.0,0.0,5.0));
    renderer.bind_camera_3d(&device,&feed,rectangle(1.0),64,64).unwrap();
    // Distinct quadrants expose vertical flips and mirrored UVs.
    for (x,y,material) in [(-1.0,1.0,&red),(1.0,1.0,&green),(-1.0,-1.0,&blue)] {
        let mut placement=transform(x,0.0,2.0,2.0);placement.set_position(Vec3::new(x,y,0.0));
        renderer.draw_mesh(&device,&mesh,material,&placement,0.0).unwrap();
    }
    // Camera feeds omit monitor surfaces even when their geometry is in view.
    // Two identical screen viewports see the same current-frame image.
    let screen=feed;
    renderer.bind_camera(&device,&screen,Rect::new(0.0,0.0,32.0,64.0)).unwrap();
    renderer.draw_background(&device).unwrap();renderer.background_drawn=true;
    renderer.bind_camera(&device,&screen,Rect::new(32.0,0.0,32.0,64.0)).unwrap();
    let values=finish_headless_frame_samples(&mut renderer,&device,&depth,&[(8,24),(24,24),(8,40),(40,24),(56,24),(40,40)]);
    for (actual,expected) in values.into_iter().zip([[1.0,0.0,0.0],[0.0,1.0,0.0],[0.0,0.0,1.0],[1.0,0.0,0.0],[0.0,1.0,0.0],[0.0,0.0,1.0]]) { assert_color(actual,expected); }
}

#[test]
fn camera_surface_rebinds_resize_reuse_descriptors_and_validate_before_submission() {
    let device=D3D12Device::warp_for_test().unwrap();let (mut renderer,depth)=headless_renderer(&device);
    let mut camera=Camera::new();camera.set_position(Vec3::new(0.0,0.0,5.0));
    let surface=rectangle(1.0);
    for (frame,size) in [32,64,32].into_iter().enumerate() {
        start_headless_frame(&mut renderer,&device,&depth,frame==0);
        let before=renderer.commands.len();
        assert!(renderer.bind_camera_3d(&device,&camera,surface,0,64).is_err());
        assert!(renderer.bind_camera_3d(&device,&camera,surface,2049,64).is_err());
        assert_eq!(renderer.commands.len(),before);
        renderer.set_environment(Environment::new(Color::BLACK,0.0,None,0.0,None,Color::rgb(2.0,0.25,0.0)).unwrap());
        renderer.bind_camera_3d(&device,&camera,surface,size,size).unwrap();
        let descriptor=renderer.camera_surfaces[0].descriptor;
        // Rebinding the same rectangle reuses the target; a later background wins.
        renderer.draw_background(&device).unwrap();renderer.background_drawn=true;
        renderer.set_environment(Environment::new(Color::BLACK,0.0,None,0.0,None,Color::rgb(3.0,0.5,0.0)).unwrap());
        renderer.bind_camera_3d(&device,&camera,surface,size,size).unwrap();
        assert_eq!(renderer.surface_bindings.len(),1);
        assert!(renderer.bind_camera_3d(&device,&camera,surface,size+1,size).is_err());
        renderer.draw_background(&device).unwrap();renderer.background_drawn=true;
        renderer.set_environment(Environment::new(Color::BLACK,0.0,None,0.0,None,Color::BLACK).unwrap());
        renderer.bind_camera(&device,&camera,Rect::new(0.0,0.0,64.0,64.0)).unwrap();
        let color=finish_headless_frame_samples(&mut renderer,&device,&depth,&[(32,32)])[0];
        assert_color(color,[3.0,0.5,0.0]);
        assert_eq!(renderer.camera_surfaces[0].descriptor,descriptor);
        assert_eq!((renderer.camera_surfaces[0].color.width,renderer.camera_surfaces[0].color.height),(size,size));
    }
}
