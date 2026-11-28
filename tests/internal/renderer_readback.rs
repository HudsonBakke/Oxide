//! Headless D3D12 render/readback regression checks, using WARP only in tests.
use super::*;
use windows::Win32::Graphics::Direct3D12::*;
use crate::{graphics::{Color, Vertex}, math::{Vec2, Vec3}};

pub(super) fn quad() -> Mesh {
    Mesh::new(
        [(-0.5, -0.5), (0.5, -0.5), (0.5, 0.5), (-0.5, 0.5)]
            .into_iter().map(|(x, y)| Vertex::new(Vec3::new(x, y, 0.0), Vec3::new(0.0, 0.0, 1.0), Vec2::new(x + 0.5, y + 0.5))).collect(),
        vec![0, 2, 1, 0, 3, 2],
    )
}

pub(super) fn transform(x: f32, z: f32, width: f32, height: f32) -> Transform {
    let mut result = Transform::new();
    result.set_position(Vec3::new(x, 0.0, z));
    result.set_scale(Vec3::new(width, height, 1.0));
    result
}

fn half_to_float(bits: u16) -> f32 {
    let sign = if bits & 0x8000 == 0 { 1.0 } else { -1.0 };
    let exponent = (bits >> 10) & 31;
    let mantissa = (bits & 1023) as f32;
    if exponent == 0 { sign * mantissa * 2.0_f32.powi(-24) }
    else { sign * (1.0 + mantissa / 1024.0) * 2.0_f32.powi(exponent as i32 - 15) }
}

fn copy_hdr(renderer: &D3D12Renderer, device: &D3D12Device) -> ID3D12Resource {
    let mut readback = None;
    unsafe {
        device.device().CreateCommittedResource(
            &D3D12_HEAP_PROPERTIES { Type: D3D12_HEAP_TYPE_READBACK, ..Default::default() },
            D3D12_HEAP_FLAG_NONE,
            &D3D12_RESOURCE_DESC {
                Dimension: D3D12_RESOURCE_DIMENSION_BUFFER, Width: 64 * 512,
                Height: 1, DepthOrArraySize: 1, MipLevels: 1,
                SampleDesc: windows::Win32::Graphics::Dxgi::Common::DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
                Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
                ..Default::default()
            }, D3D12_RESOURCE_STATE_COPY_DEST, None, &mut readback,
        ).unwrap();
    }
    let readback: ID3D12Resource = readback.unwrap();
    let target = &renderer.hdr_target.as_ref().unwrap().resource;
    transition_resource(&renderer.command_list, target, D3D12_RESOURCE_STATE_RENDER_TARGET, D3D12_RESOURCE_STATE_COPY_SOURCE);
    let mut source = D3D12_TEXTURE_COPY_LOCATION {
        pResource: std::mem::ManuallyDrop::new(Some(target.clone())),
        Type: D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX,
        Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 { SubresourceIndex: 0 },
    };
    let mut destination = D3D12_TEXTURE_COPY_LOCATION {
        pResource: std::mem::ManuallyDrop::new(Some(readback.clone())),
        Type: D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT,
        Anonymous: D3D12_TEXTURE_COPY_LOCATION_0 {
            PlacedFootprint: D3D12_PLACED_SUBRESOURCE_FOOTPRINT {
                Offset: 0,
                Footprint: D3D12_SUBRESOURCE_FOOTPRINT {
                    Format: DXGI_FORMAT_R16G16B16A16_FLOAT,
                    Width: 64, Height: 64, Depth: 1, RowPitch: 512,
                },
            },
        },
    };
    unsafe {
        renderer.command_list.CopyTextureRegion(&destination, 0, 0, 0, &source, None);
        std::mem::ManuallyDrop::drop(&mut source.pResource);
        std::mem::ManuallyDrop::drop(&mut destination.pResource);
    }
    transition_resource(&renderer.command_list, target, D3D12_RESOURCE_STATE_COPY_SOURCE, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE);
    readback
}

#[test]
fn queued_shadows_multiply_transmission_and_preserve_opaque_blockers() {
    for kind in 0..5 { check_light_transmission(kind); }
}

fn check_light_transmission(kind: usize) {
    let device = D3D12Device::warp_for_test().unwrap();
    let mut renderer = D3D12Renderer::new(&device).unwrap();
    let cpu = unsafe { renderer.texture_heap.GetCPUDescriptorHandleForHeapStart() };
    renderer.hdr_target = Some(HdrTarget::new(&device, 64, 64, cpu).unwrap());
    // This separate descriptor is not sampled by any shader.
    let mut unused = cpu;
    unused.ptr += (FIRST_TEXTURE_DESCRIPTOR - 1) as usize * renderer.texture_descriptor_size as usize;
    let depth = ShadowMap::new(&device, 64, unused).unwrap();
    let mut light = DirectionalLight::new(Vec3::new(-1.0, 0.0, -1.0), Color::WHITE, 1.0).unwrap();
    let settings = ShadowSettings { size: 12.0, depth: 16.0, resolution: 128, normal_bias: 0.0, depth_bias: 0.001, ..Default::default() };
    light.set_shadow_settings(settings).unwrap();
    light.set_casts_shadows(true);
    let mut point = PointLight::new(Vec3::new(7.0, 0.0, 6.0), Color::WHITE, if kind == 3 { 42.5 } else { 85.0 }).unwrap();
    let mut spot = SpotLight::new(Vec3::new(7.0, 0.0, 6.0), Vec3::new(-7.0, 0.0, -6.0), crate::math::Angle::degrees(if kind == 4 { 120.0 } else { 30.0 }), Color::WHITE, 85.0).unwrap();
    point.set_shadow_settings(settings).unwrap(); point.set_casts_shadows(true);
    spot.set_shadow_settings(settings).unwrap(); spot.set_casts_shadows(true);
    if kind == 3 { light.set_intensity(0.5).unwrap(); }
    let mut camera = Camera::new();
    camera.set_position(Vec3::new(0.0, 0.0, 5.0));
    let mut mesh = quad();
    let mut material = Material::solid(Color::WHITE);
    let mut samples = Vec::new();
    // Window planes sit outside the camera but on the light ray to the receiver.
    for frame in 0..8 {
        if frame == 7 {
            camera.set_position(Vec3::new(0.0, 0.0, 5.0));
            material.set_base_color(Color::WHITE);
            for vertex in mesh.vertices_mut() { vertex.position.x -= 100.0; }
        }
        renderer.reset_frame_state();
    renderer.frame_size = (64, 64);
        unsafe {
            renderer.command_allocators[0].Reset().unwrap();
            renderer.command_list.Reset(&renderer.command_allocators[0], None).unwrap();
            renderer.command_list.SetDescriptorHeaps(&[Some(renderer.texture_heap.clone())]);
            renderer.command_list.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
        }
        if frame == 0 {
            transition_resource(&renderer.command_list, &depth.resource, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE, D3D12_RESOURCE_STATE_DEPTH_WRITE);
        }
        let hdr = renderer.hdr_target.as_ref().unwrap();
        transition_resource(&renderer.command_list, &hdr.resource, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE, D3D12_RESOURCE_STATE_RENDER_TARGET);
        unsafe {
            renderer.command_list.ClearDepthStencilView(depth.dsv, D3D12_CLEAR_FLAG_DEPTH, 1.0, 0, None);
            renderer.command_list.ClearRenderTargetView(hdr.rtv, &[0.0; 4], None);
        }
        renderer.bind_camera(&device, &camera, Rect::new(0.0, 0.0, 64.0, 64.0)).unwrap();
        match kind {
            0 => renderer.add_directional_light(&light),
            1 => renderer.add_point_light(&point),
            3 => { renderer.add_directional_light(&light); renderer.add_point_light(&point); }
            _ => renderer.add_spot_light(&spot),
        }
        if frame == 7 {
            // Dropping CPU resources after submission must leave the GPU draw valid.
            let transient_mesh = quad();
            let transient_material = Material::solid(Color::WHITE);
            renderer.draw_mesh(&device, &transient_mesh, &transient_material, &transform(0.0, 0.0, 4.0, 4.0), 0.0).unwrap();
        } else {
        renderer.draw_mesh(&device, &mesh, &material, &transform(0.0, 0.0, 4.0, 4.0), 0.0).unwrap();
        }
        let count = renderer.commands.len();
        assert!(renderer.draw_mesh(&device, &mesh, &material, &Transform::new(), f32::NAN).is_err());
        assert_eq!(renderer.commands.len(), count);
        if frame == 6 {
            // Later geometry, material, lighting, and camera changes must not
            // alter the earlier white receiver draw at the center of the view.
            for vertex in mesh.vertices_mut() { vertex.position.x += 100.0; }
            material.set_base_color(Color::BLACK);
            renderer.clear_lights();
            camera.set_position(Vec3::new(100.0, 0.0, 5.0));
            renderer.bind_camera(&device, &camera, Rect::new(48.0, 0.0, 16.0, 64.0)).unwrap();
            renderer.draw_mesh(&device, &mesh, &material, &Transform::new(), 1.0).unwrap();
            assert!(renderer.mesh_cache.contains_key(&(mesh.id(), 0, 0)));
            assert!(renderer.mesh_cache.contains_key(&(mesh.id(), mesh.vertex_generation(), 0)));
        }
        if (1..=3).contains(&frame) || frame == 5 {
            renderer.draw_mesh(&device, &mesh, &material, &transform(2.0, 2.0, 2.0, 3.0), 0.5).unwrap();
        }
        if (2..=3).contains(&frame) {
            renderer.draw_mesh(&device, &mesh, &material, &transform(3.0, 3.0, 2.0, 3.0), 0.5).unwrap();
        }
        if frame == 3 {
            renderer.draw_mesh(&device, &mesh, &material, &transform(1.0, 1.0, 1.0, 3.0), 0.0).unwrap();
        }
        if frame == 4 {
            renderer.draw_mesh(&device, &mesh, &material, &transform(-2.0, -2.0, 2.0, 3.0), 0.5).unwrap();
        }
        if frame == 5 {
            // A second camera can submit the same caster without doubling its transmission.
            renderer.bind_camera(&device, &camera, Rect::new(48.0, 0.0, 16.0, 64.0)).unwrap();
            renderer.draw_mesh(&device, &mesh, &material, &transform(2.0, 2.0, 2.0, 3.0), 0.5).unwrap();
        }
        let shadow = renderer.render_shadows(&device).unwrap();
        let rtv = renderer.hdr_target.as_ref().unwrap().rtv;
        unsafe { renderer.command_list.OMSetRenderTargets(1, Some(&rtv), false, Some(&depth.dsv)); }
        renderer.render_scene(shadow, depth.dsv);
        unsafe { renderer.command_list.OMSetRenderTargets(0, None, false, None); }
        let readback = copy_hdr(&renderer, &device);
        unsafe {
            if let Ok(queue) = device.device().cast::<ID3D12InfoQueue>() {
                for index in 0..queue.GetNumStoredMessagesAllowedByRetrievalFilter() {
                    let mut length = 0;
                    queue.GetMessage(index, None, &mut length).unwrap();
                    let mut buffer = vec![0_u64; (length + 7) / 8];
                    let message = buffer.as_mut_ptr() as *mut D3D12_MESSAGE;
                    queue.GetMessage(index, Some(message), &mut length).unwrap();
                    let message = &*message;
                    if message.Severity == D3D12_MESSAGE_SEVERITY_ERROR || message.Severity == D3D12_MESSAGE_SEVERITY_CORRUPTION {
                        let description = std::ffi::CStr::from_ptr(message.pDescription as *const i8).to_string_lossy();
                        panic!("D3D12 validation: {description}");
                    }
                }
                queue.ClearStoredMessages();
            }
            renderer.command_list.Close().unwrap();
            let list: ID3D12CommandList = renderer.command_list.cast().unwrap();
            device.command_queue().ExecuteCommandLists(&[Some(list)]);
        }
        renderer.wait_for_gpu(&device).unwrap();
        let mut pointer = std::ptr::null_mut();
        unsafe {
            readback.Map(0, None, Some(&mut pointer)).unwrap();
            let red = std::ptr::read_unaligned((pointer as *const u8).add(32 * 512 + 32 * 8) as *const u16);
            samples.push(half_to_float(red));
            readback.Unmap(0, None);
        }
        renderer.finish_completed_frame(&device);
    }
    let full = samples[0];
    assert!(full > 0.05, "light {kind} receiver must be lit: {samples:?}");
    for (index, expected) in [(1, 0.5), (2, 0.25), (3, 0.0), (4, 1.0), (5, 0.5), (6, 1.0), (7, 1.0)] {
        assert!((samples[index] / full - expected).abs() < 0.025, "wrong transmission for light {kind} in frame {index}: {samples:?}");
    }
}


#[test]
fn visible_transparency_blends_in_depth_order_per_camera() {
    let device = D3D12Device::warp_for_test().unwrap();
    let mut renderer = D3D12Renderer::new(&device).unwrap();
    let cpu = unsafe { renderer.texture_heap.GetCPUDescriptorHandleForHeapStart() };
    renderer.hdr_target = Some(HdrTarget::new(&device, 64, 64, cpu).unwrap());
    let mut unused = cpu;
    unused.ptr += (FIRST_TEXTURE_DESCRIPTOR - 1) as usize * renderer.texture_descriptor_size as usize;
    let depth = ShadowMap::new(&device, 64, unused).unwrap();
    renderer.set_environment(crate::graphics::Environment::new(Color::BLACK, 0.0, None, 0.0, None, Color::BLACK).unwrap());
    let mut mesh = quad();
    // Duplicate the winding to let opposing cameras see the same test planes.
    // Back-face culling still selects just one copy for each camera.
    mesh = Mesh::new(mesh.vertices().to_vec(), vec![0, 2, 1, 0, 3, 2, 0, 1, 2, 0, 2, 3]);
    let colors = [Color::RED, Color::GREEN, Color::BLUE];
    let materials: Vec<_> = colors.into_iter().map(|color| {
        let mut material = Material::solid(Color::BLACK);
        material.set_emissive_color(color);
        material.set_emissive_strength(1.0).unwrap();
        material
    }).collect();
    let mut camera = Camera::new();
    camera.set_position(Vec3::new(0.0, 0.0, 5.0));
    for frame in 0..8 {
        renderer.reset_frame_state();
    renderer.frame_size = (64, 64);
        unsafe {
            renderer.command_allocators[0].Reset().unwrap();
            renderer.command_list.Reset(&renderer.command_allocators[0], None).unwrap();
            renderer.command_list.SetDescriptorHeaps(&[Some(renderer.texture_heap.clone())]);
            renderer.command_list.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
        }
        if frame == 0 {
            transition_resource(&renderer.command_list, &depth.resource, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE, D3D12_RESOURCE_STATE_DEPTH_WRITE);
        }
        let hdr = renderer.hdr_target.as_ref().unwrap();
        transition_resource(&renderer.command_list, &hdr.resource, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE, D3D12_RESOURCE_STATE_RENDER_TARGET);
        unsafe {
            renderer.command_list.ClearDepthStencilView(depth.dsv, D3D12_CLEAR_FLAG_DEPTH, 1.0, 0, None);
            renderer.command_list.ClearRenderTargetView(hdr.rtv, &[0.0; 4], None);
        }
        renderer.bind_camera(&device, &camera, Rect::new(0.0, 0.0, if frame == 6 { 32.0 } else { 64.0 }, 64.0)).unwrap();
        let layers: Vec<(usize, f32, f32)> = match frame {
            0 => vec![(0, 1.0, 0.5), (2, 0.0, 0.0)],
            1 => vec![(2, 0.0, 0.0), (0, 1.0, 0.5)],
            2 => vec![(0, 2.0, 0.5), (1, 1.0, 0.5), (2, 0.0, 0.0)],
            3 => vec![(2, 0.0, 0.0), (1, 1.0, 0.5), (0, 2.0, 0.5)],
            4 => vec![(0, 2.0, 1.0), (2, 0.0, 0.0)],
            5 => vec![(0, 1.0, 0.5), (2, 0.0, 0.0), (1, 2.0, 0.0)],
            6 => vec![(0, 1.0, 0.5), (1, -1.0, 0.5)],
            _ => vec![(0, 2.0, 0.5)],
        };
        for (color, z, transparency) in layers {
            renderer.draw_mesh(&device, &mesh, &materials[color], &transform(0.0, z, 4.0, 4.0), transparency).unwrap();
        }
        if frame == 6 {
            let mut reverse = Camera::new();
            reverse.set_position(Vec3::new(0.0, 0.0, -5.0));
            reverse.set_rotation(crate::math::Quaternion::from_to(Vec3::new(0.0, 0.0, -1.0), Vec3::new(0.0, 0.0, 1.0)));
            renderer.bind_camera(&device, &reverse, Rect::new(32.0, 0.0, 32.0, 64.0)).unwrap();
            renderer.draw_mesh(&device, &mesh, &materials[0], &transform(0.0, 1.0, 4.0, 4.0), 0.5).unwrap();
            renderer.draw_mesh(&device, &mesh, &materials[1], &transform(0.0, -1.0, 4.0, 4.0), 0.5).unwrap();
        }
        if frame == 7 {
            renderer.bind_camera(&device, &camera, Rect::new(0.0, 0.0, 64.0, 64.0)).unwrap();
            renderer.draw_mesh(&device, &mesh, &materials[2], &transform(0.0, 0.0, 4.0, 4.0), 0.0).unwrap();
        }
        let shadow = renderer.render_shadows(&device).unwrap();
        let rtv = renderer.hdr_target.as_ref().unwrap().rtv;
        unsafe { renderer.command_list.OMSetRenderTargets(1, Some(&rtv), false, Some(&depth.dsv)); }
        renderer.render_scene(shadow, depth.dsv);
        unsafe { renderer.command_list.OMSetRenderTargets(0, None, false, None); }
        let readback = copy_hdr(&renderer, &device);
        unsafe {
            if let Ok(queue) = device.device().cast::<ID3D12InfoQueue>() {
                for index in 0..queue.GetNumStoredMessagesAllowedByRetrievalFilter() {
                    let mut length = 0;
                    queue.GetMessage(index, None, &mut length).unwrap();
                    let mut buffer = vec![0_u64; (length + 7) / 8];
                    let message = buffer.as_mut_ptr() as *mut D3D12_MESSAGE;
                    queue.GetMessage(index, Some(message), &mut length).unwrap();
                    let message = &*message;
                    if message.Severity == D3D12_MESSAGE_SEVERITY_ERROR || message.Severity == D3D12_MESSAGE_SEVERITY_CORRUPTION {
                        let description = std::ffi::CStr::from_ptr(message.pDescription as *const i8).to_string_lossy();
                        panic!("D3D12 validation: {description}");
                    }
                }
                queue.ClearStoredMessages();
            }
            renderer.command_list.Close().unwrap();
            let list: ID3D12CommandList = renderer.command_list.cast().unwrap();
            device.command_queue().ExecuteCommandLists(&[Some(list)]);
        }
        renderer.wait_for_gpu(&device).unwrap();
        let mut pointer = std::ptr::null_mut();
        unsafe {
            readback.Map(0, None, Some(&mut pointer)).unwrap();
            assert!(!pointer.is_null(), "D3D12 readback pointer is null");
            let sample = |x: usize| -> [f32; 3] {
                [0, 1, 2].map(|channel| half_to_float(std::ptr::read_unaligned(
                    (pointer as *const u8).add(32 * 512 + x * 8 + channel * 2) as *const u16,
                )))
            };
            let expected = match frame {
                0 | 1 => [0.5, 0.0, 0.5],
                2 | 3 => [0.5, 0.25, 0.25],
                4 | 7 => [0.0, 0.0, 1.0],
                5 => [0.0, 1.0, 0.0],
                _ => [0.5, 0.25, 0.0],
            };
            let actual = sample(if frame == 6 { 16 } else { 32 });
            assert!(actual.into_iter().zip(expected).all(|(a, b)| (a - b).abs() < 0.003), "frame {frame}: {actual:?} != {expected:?}");
            if frame == 6 {
                let right = sample(48);
                assert!((right[0] - 0.25).abs() < 0.003 && (right[1] - 0.5).abs() < 0.003 && right[2] == 0.0, "opposing camera sort: {right:?}");
            }
            readback.Unmap(0, None);
        }
        renderer.finish_completed_frame(&device);
    }
}

#[test]
fn point_shadow_projection_faces_cover_all_axes() {
    let request = ActiveLight::Point(PointLight::new(Vec3::new(2.0,3.0,4.0), Color::WHITE, 1.0).unwrap()).shadow_request().unwrap();
    assert_eq!(request.faces, 6);
    for (face, direction) in [Vec3::new(1.0,0.0,0.0), Vec3::new(-1.0,0.0,0.0), Vec3::new(0.0,1.0,0.0), Vec3::new(0.0,-1.0,0.0), Vec3::new(0.0,0.0,1.0), Vec3::new(0.0,0.0,-1.0)].into_iter().enumerate() {
        for (distance, expected_depth) in [(request.settings.near_plane, 0.0), (request.settings.far_plane, 1.0)] {
            let point = request.position + direction * distance;
            let clip = request.matrices[face] * crate::math::Vec4::new(point.x, point.y, point.z, 1.0);
            assert!(clip.w > 0.0);
            assert!((clip.x / clip.w).abs() < 0.0001 && (clip.y / clip.w).abs() < 0.0001);
            assert!((clip.z / clip.w - expected_depth).abs() < 0.0001);
        }
    }
}

pub(super) fn headless_renderer(device: &D3D12Device) -> (D3D12Renderer, ShadowMap) {
    let mut renderer = D3D12Renderer::new(device).unwrap();
    let cpu = unsafe { renderer.texture_heap.GetCPUDescriptorHandleForHeapStart() };
    renderer.hdr_target = Some(HdrTarget::new(device, 64, 64, cpu).unwrap());
    let mut unused = cpu;
    unused.ptr += (FIRST_TEXTURE_DESCRIPTOR - 1) as usize * renderer.texture_descriptor_size as usize;
    let depth = ShadowMap::new(device, 64, unused).unwrap();
    renderer.set_environment(crate::graphics::Environment::new(Color::BLACK, 0.0, None, 0.0, None, Color::BLACK).unwrap());
    (renderer, depth)
}

pub(super) fn start_headless_frame(renderer: &mut D3D12Renderer, device: &D3D12Device, depth: &ShadowMap, first: bool) {
    renderer.reclaim_unused_assets();
    renderer.reset_frame_state();
    renderer.frame_size = (64, 64);
    unsafe {
        renderer.command_allocators[0].Reset().unwrap();
        renderer.command_list.Reset(&renderer.command_allocators[0], None).unwrap();
        renderer.command_list.SetDescriptorHeaps(&[Some(renderer.texture_heap.clone())]);
        renderer.command_list.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
    }
    if first {
        transition_resource(&renderer.command_list, &depth.resource, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE, D3D12_RESOURCE_STATE_DEPTH_WRITE);
    }
    let hdr = renderer.hdr_target.as_ref().unwrap();
    transition_resource(&renderer.command_list, &hdr.resource, D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE, D3D12_RESOURCE_STATE_RENDER_TARGET);
    unsafe {
        renderer.command_list.ClearDepthStencilView(depth.dsv, D3D12_CLEAR_FLAG_DEPTH, 1.0, 0, None);
        renderer.command_list.ClearRenderTargetView(hdr.rtv, &[0.0; 4], None);
    }
    let mut camera = Camera::new();
    camera.set_position(Vec3::new(0.0, 0.0, 5.0));
    renderer.bind_camera(device, &camera, Rect::new(0.0, 0.0, 64.0, 64.0)).unwrap();
}

pub(super) fn finish_headless_frame_samples(renderer: &mut D3D12Renderer, device: &D3D12Device, depth: &ShadowMap, points: &[(usize,usize)]) -> Vec<[f32;3]> {
    if !renderer.background_drawn { renderer.draw_background(device).unwrap(); }
    let shadow = renderer.render_shadows(device).unwrap();
    renderer.render_camera_surfaces(device, shadow);
    let rtv = renderer.hdr_target.as_ref().unwrap().rtv;
    unsafe { renderer.command_list.OMSetRenderTargets(1, Some(&rtv), false, Some(&depth.dsv)); }
    renderer.render_scene(shadow, depth.dsv);
    unsafe { renderer.command_list.OMSetRenderTargets(0, None, false, None); }
    let readback = copy_hdr(renderer, device);
    unsafe {
        renderer.command_list.Close().unwrap();
        if let Ok(queue) = device.device().cast::<ID3D12InfoQueue>() {
            for index in 0..queue.GetNumStoredMessagesAllowedByRetrievalFilter() {
                let mut length = 0;
                queue.GetMessage(index, None, &mut length).unwrap();
                let mut storage = vec![0_u64; length.div_ceil(8)];
                let message = storage.as_mut_ptr() as *mut D3D12_MESSAGE;
                queue.GetMessage(index, Some(message), &mut length).unwrap();
                if (*message).Severity == D3D12_MESSAGE_SEVERITY_ERROR || (*message).Severity == D3D12_MESSAGE_SEVERITY_CORRUPTION {
                    panic!("D3D12 validation: {}", std::ffi::CStr::from_ptr((*message).pDescription as *const i8).to_string_lossy());
                }
            }
            queue.ClearStoredMessages();
        }
        let list: ID3D12CommandList = renderer.command_list.cast().unwrap();
        device.command_queue().ExecuteCommandLists(&[Some(list)]);
    }
    renderer.wait_for_gpu(device).unwrap();
    let mut pointer = std::ptr::null_mut();
    let samples = unsafe {
        readback.Map(0, None, Some(&mut pointer)).unwrap();
        let values = points.iter().map(|(x,y)| [0,1,2].map(|channel| half_to_float(std::ptr::read_unaligned(
            (pointer as *const u8).add(y*512+x*8+channel*2) as *const u16)))).collect();
        readback.Unmap(0, None);
        values
    };
    renderer.finish_completed_frame(device);
    samples
}

fn finish_headless_frame(renderer: &mut D3D12Renderer, device: &D3D12Device, depth: &ShadowMap) -> f32 {
    finish_headless_frame_samples(renderer,device,depth,&[(32,32)])[0][0]
}

fn emissive_red() -> Material {
    let mut material = Material::solid(Color::BLACK);
    material.set_emissive_color(Color::RED);
    material.set_emissive_strength(1.0).unwrap();
    material
}

#[test]
fn transient_assets_survive_submission_and_recycle_more_than_a_heaps_worth_of_descriptors() {
    let device = D3D12Device::warp_for_test().unwrap();
    let (mut renderer, depth) = headless_renderer(&device);
    for frame in 0..12 {
        start_headless_frame(&mut renderer, &device, &depth, frame == 0);
        for _ in 0..100 {
            let mesh = quad();
            let material = emissive_red();
            renderer.draw_mesh(&device, &mesh, &material, &transform(0.0, 0.0, 4.0, 4.0), 0.0).unwrap();
        }
        assert_eq!(renderer.mesh_cache.len(), 100);
        assert_eq!(renderer.material_descriptor_cache.len(), 100);
        assert_eq!(renderer.descriptors.used(), 504);
        assert!((finish_headless_frame(&mut renderer, &device, &depth) - 1.0).abs() < 0.003);
        assert!(renderer.mesh_cache.is_empty());
        assert!(renderer.latest_mesh_versions.is_empty());
        assert!(renderer.material_descriptor_cache.is_empty());
        assert!(renderer.normal_descriptor_cache.is_empty());
        assert_eq!(renderer.texture_cache.len(), 1);
        assert_eq!(renderer.texture_owners.len(), 1);
        assert_eq!(renderer.descriptors.used(), 4);
    }
}

#[test]
fn texture_clones_and_environment_owners_keep_resources_until_the_last_owner_drops() {
    let device = D3D12Device::warp_for_test().unwrap();
    let (mut renderer, depth) = headless_renderer(&device);
    let texture = crate::graphics::Texture::solid(Color::WHITE);
    let clone = texture.clone();
    let id = texture.id();
    renderer.set_environment(crate::graphics::Environment::new(Color::BLACK, 0.0, Some(texture.clone()), 1.0, Some(texture.clone()), Color::BLACK).unwrap());
    start_headless_frame(&mut renderer, &device, &depth, true);
    let material = Material::new(texture);
    renderer.draw_mesh(&device, &quad(), &material, &transform(0.0, 0.0, 4.0, 4.0), 0.0).unwrap();
    drop(material);
    assert!(renderer.texture_cache[&id].upload_pending());
    assert!(renderer.specular_cache[&id].upload_pending());
    assert!(renderer.irradiance_cache[&id].upload_pending());
    finish_headless_frame(&mut renderer, &device, &depth);
    assert!(renderer.texture_cache.contains_key(&id));
    assert!(!renderer.texture_cache[&id].upload_pending());
    assert!(!renderer.specular_cache[&id].upload_pending());
    assert!(!renderer.irradiance_cache[&id].upload_pending());
    assert!(renderer.specular_cache.contains_key(&id));
    assert!(renderer.irradiance_cache.contains_key(&id));
    drop(clone);
    renderer.reclaim_unused_assets();
    assert!(renderer.specular_cache.contains_key(&id));
    renderer.clear_environment();
    renderer.reclaim_unused_assets();
    assert!(!renderer.texture_cache.contains_key(&id));
    assert!(renderer.specular_cache.is_empty());
    assert!(renderer.irradiance_cache.is_empty());
    assert!(renderer.environment_descriptor_cache.is_empty());
    assert_eq!(renderer.descriptors.used(), 1);
}

#[test]
fn descriptor_exhaustion_preserves_queued_draws_and_recovers_after_the_frame() {
    let device = D3D12Device::warp_for_test().unwrap();
    let (mut renderer, depth) = headless_renderer(&device);
    renderer.descriptors = DescriptorAllocator::new(FIRST_TEXTURE_DESCRIPTOR, FIRST_TEXTURE_DESCRIPTOR + 12);
    for frame in 0..2 {
        start_headless_frame(&mut renderer, &device, &depth, frame == 0);
        renderer.draw_mesh(&device, &quad(), &emissive_red(), &transform(0.0, 0.0, 4.0, 4.0), 0.0).unwrap();
        let queued = renderer.commands.len();
        let error = renderer.draw_mesh(&device, &quad(), &emissive_red(), &Transform::new(), 0.0).unwrap_err();
        assert!(error.to_string().contains("GPU descriptor capacity exhausted"));
        assert_eq!(renderer.commands.len(), queued);
        assert_eq!(renderer.descriptors.used(), 12);
        assert!((finish_headless_frame(&mut renderer, &device, &depth) - 1.0).abs() < 0.003);
        assert_eq!(renderer.descriptors.used(), 4);
    }
}

#[test]
fn excess_lights_are_rejected_before_preparing_a_draw_and_can_be_cleared() {
    let device = D3D12Device::warp_for_test().unwrap();
    let (mut renderer, depth) = headless_renderer(&device);
    start_headless_frame(&mut renderer, &device, &depth, true);
    for kind in ["point", "spot", "directional"] {
        renderer.clear_lights();
        for _ in 0..9 {
            match kind {
                "point" => renderer.add_point_light(&PointLight::new(Vec3::new(0.0, 2.0, 0.0), Color::WHITE, 1.0).unwrap()),
                "spot" => renderer.add_spot_light(&SpotLight::new(Vec3::new(0.0, 2.0, 0.0), Vec3::new(0.0, -1.0, 0.0), crate::math::Angle::degrees(30.0), Color::WHITE, 1.0).unwrap()),
                _ => renderer.add_directional_light(&DirectionalLight::new(Vec3::new(0.0, -1.0, 0.0), Color::WHITE, 1.0).unwrap()),
            }
        }
        let error = renderer.draw_mesh(&device, &quad(), &emissive_red(), &Transform::new(), 0.0).unwrap_err();
        assert!(error.to_string().contains(&format!("too many {kind} lights")));
        assert!(renderer.commands.is_empty());
        assert!(renderer.texture_cache.is_empty());
        assert_eq!(renderer.descriptors.used(), 0);
    }
    renderer.clear_lights();
    renderer.draw_mesh(&device, &quad(), &emissive_red(), &transform(0.0, 0.0, 4.0, 4.0), 0.0).unwrap();
    assert!((finish_headless_frame(&mut renderer, &device, &depth) - 1.0).abs() < 0.003);
}

#[test]
fn shadow_maps_and_transmission_storage_retire_after_the_last_using_frame() {
    let device = D3D12Device::warp_for_test().unwrap();
    let (mut renderer, depth) = headless_renderer(&device);
    let mesh = quad();
    let material = emissive_red();
    let mut light = DirectionalLight::new(Vec3::new(0.0, -1.0, -1.0), Color::WHITE, 1.0).unwrap();
    light.set_shadow_settings(ShadowSettings { resolution: 128, ..Default::default() }).unwrap();
    light.set_casts_shadows(true);
    for frame in 0..3 {
        start_headless_frame(&mut renderer, &device, &depth, frame == 0);
        if frame < 2 { renderer.add_directional_light(&light); }
        renderer.draw_mesh(&device, &mesh, &material, &transform(0.0, 0.0, 4.0, 4.0), if frame == 0 { 0.5 } else { 0.0 }).unwrap();
        finish_headless_frame(&mut renderer, &device, &depth);
        if frame < 2 {
            assert_eq!(renderer.shadow_resources.len(), 1);
            assert_eq!(renderer.shadow_resources[0].as_ref().unwrap().transmission.is_some(), frame == 0);
        } else {
            assert!(renderer.shadow_resources.is_empty());
        }
    }
}
