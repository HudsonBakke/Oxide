//! Headless D3D12 render/readback regression checks, using WARP only in tests.
use super::*;
use windows::Win32::Graphics::Direct3D12::*;
use crate::{graphics::{Color, Vertex}, math::{Vec2, Vec3}};

fn quad() -> Mesh {
    Mesh::new(
        [(-0.5, -0.5), (0.5, -0.5), (0.5, 0.5), (-0.5, 0.5)]
            .into_iter().map(|(x, y)| Vertex::new(Vec3::new(x, y, 0.0), Vec3::new(0.0, 0.0, 1.0), Vec2::new(x + 0.5, y + 0.5))).collect(),
        vec![0, 2, 1, 0, 3, 2],
    )
}

fn transform(x: f32, z: f32, width: f32, height: f32) -> Transform {
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
    let device = D3D12Device::warp_for_test().unwrap();
    let mut renderer = D3D12Renderer::new(&device).unwrap();
    let cpu = unsafe { renderer.texture_heap.GetCPUDescriptorHandleForHeapStart() };
    renderer.hdr_target = Some(HdrTarget::new(&device, 64, 64, cpu).unwrap());
    // This separate descriptor is not sampled by any shader.
    let mut unused = cpu;
    unused.ptr += (FIRST_TEXTURE_DESCRIPTOR - 1) as usize * renderer.texture_descriptor_size as usize;
    let depth = ShadowMap::new(&device, 64, unused).unwrap();
    let light = DirectionalLight::new(Vec3::new(-1.0, 0.0, -1.0), Color::WHITE, 1.0).unwrap();
    let settings = DirectionalShadowSettings { size: 12.0, depth: 16.0, resolution: 128, normal_bias: 0.0, ..Default::default() };
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
        renderer.set_directional_shadows(&light, &settings).unwrap();
        renderer.bind_camera(&device, &camera, Rect::new(0.0, 0.0, 64.0, 64.0)).unwrap();
        renderer.add_directional_light(&light);
        if frame == 7 {
            // Dropping CPU resources after submission must leave the GPU draw valid.
            let transient_mesh = quad();
            let transient_material = Material::solid(Color::WHITE);
            renderer.draw_mesh(&device, &transient_mesh, &transient_material, &transform(0.0, 0.0, 4.0, 4.0), 1.0).unwrap();
        } else {
        renderer.draw_mesh(&device, &mesh, &material, &transform(0.0, 0.0, 4.0, 4.0), 1.0).unwrap();
        }
        let count = renderer.commands.len();
        assert!(renderer.draw_mesh(&device, &mesh, &material, &Transform::new(), f32::NAN).is_err());
        assert_eq!(renderer.commands.len(), count);
        assert!(renderer.clear_directional_shadows().is_err());
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
            renderer.draw_mesh(&device, &mesh, &material, &transform(2.0, 2.0, 1.0, 3.0), 0.5).unwrap();
        }
        if (2..=3).contains(&frame) {
            renderer.draw_mesh(&device, &mesh, &material, &transform(3.0, 3.0, 1.0, 3.0), 0.5).unwrap();
        }
        if frame == 3 {
            renderer.draw_mesh(&device, &mesh, &material, &transform(1.0, 1.0, 1.0, 3.0), 0.0).unwrap();
        }
        if frame == 4 {
            renderer.draw_mesh(&device, &mesh, &material, &transform(-2.0, -2.0, 1.0, 3.0), 0.5).unwrap();
        }
        if frame == 5 {
            // A second camera can submit the same caster without doubling its transmission.
            renderer.bind_camera(&device, &camera, Rect::new(48.0, 0.0, 16.0, 64.0)).unwrap();
            renderer.draw_mesh(&device, &mesh, &material, &transform(2.0, 2.0, 1.0, 3.0), 0.5).unwrap();
        }
        let shadow = renderer.render_shadows(&device).unwrap();
        let rtv = renderer.hdr_target.as_ref().unwrap().rtv;
        unsafe { renderer.command_list.OMSetRenderTargets(1, Some(&rtv), false, Some(&depth.dsv)); }
        renderer.render_scene(shadow);
        unsafe { renderer.command_list.OMSetRenderTargets(0, None, false, None); }
        let readback = copy_hdr(&renderer, &device);
        unsafe {
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
    }
    let full = samples[0];
    assert!(full > 0.05, "receiver must be lit: {samples:?}");
    for (index, expected) in [(1, 0.5), (2, 0.25), (3, 0.0), (4, 1.0), (5, 0.5), (6, 1.0), (7, 1.0)] {
        assert!((samples[index] / full - expected).abs() < 0.025, "wrong transmission in frame {index}: {samples:?}");
    }
}
