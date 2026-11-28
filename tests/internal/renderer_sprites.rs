//! Real sRGB output and overlay readbacks, including the display pass.
use super::*;
use crate::{graphics::{Color,TextOptions},math::Vec2};
use super::readback::{headless_renderer,start_headless_frame};
use windows::Win32::Graphics::{Direct3D12::*,Dxgi::Common::*};

fn render(exposure:f32,draw:impl FnOnce(&mut D3D12Renderer,&D3D12Device))->Vec<[u8;4]> {
    let device=D3D12Device::warp_for_test().unwrap();let (mut renderer,depth)=headless_renderer(&device);
    start_headless_frame(&mut renderer,&device,&depth,true);
    // Exercise drawing without any bound camera and with dropped CPU assets.
    renderer.reset_frame_state();renderer.frame_size=(64,64);renderer.set_exposure(exposure);
    draw(&mut renderer,&device);
    let mut output=None;let heap;let mut readback=None;
    unsafe {
        device.device().CreateCommittedResource(&D3D12_HEAP_PROPERTIES {Type:D3D12_HEAP_TYPE_DEFAULT,..Default::default()},D3D12_HEAP_FLAG_NONE,
            &D3D12_RESOURCE_DESC {Dimension:D3D12_RESOURCE_DIMENSION_TEXTURE2D,Width:64,Height:64,DepthOrArraySize:1,MipLevels:1,Format:DXGI_FORMAT_R8G8B8A8_TYPELESS,
                SampleDesc:DXGI_SAMPLE_DESC {Count:1,Quality:0},Flags:D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET,..Default::default()},D3D12_RESOURCE_STATE_RENDER_TARGET,None,&mut output).unwrap();
        heap=device.device().CreateDescriptorHeap::<ID3D12DescriptorHeap>(&D3D12_DESCRIPTOR_HEAP_DESC {Type:D3D12_DESCRIPTOR_HEAP_TYPE_RTV,NumDescriptors:1,..Default::default()}).unwrap();
    }
    let output:ID3D12Resource=output.unwrap();let rtv=unsafe {heap.GetCPUDescriptorHandleForHeapStart()};
    unsafe {
        device.device().CreateRenderTargetView(&output,Some(&D3D12_RENDER_TARGET_VIEW_DESC {Format:DXGI_FORMAT_R8G8B8A8_UNORM_SRGB,ViewDimension:D3D12_RTV_DIMENSION_TEXTURE2D,..Default::default()}),rtv);
        let hdr=renderer.hdr_target.as_ref().unwrap();
        renderer.command_list.ClearRenderTargetView(hdr.rtv,&[1.0,1.0,1.0,1.0],None);
        transition_resource(&renderer.command_list,&hdr.resource,D3D12_RESOURCE_STATE_RENDER_TARGET,D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE);
        renderer.command_list.OMSetRenderTargets(1,Some(&rtv),false,None);
        renderer.set_viewport(Rect::new(0.0,0.0,64.0,64.0));
        let constants=renderer.constant_buffer.write(&device,&[exposure,0.0,0.0,0.0]).unwrap();
        renderer.command_list.SetGraphicsRootSignature(renderer.display_pipeline.root_signature());
        renderer.command_list.SetPipelineState(renderer.display_pipeline.pipeline_state());
        renderer.command_list.SetGraphicsRootConstantBufferView(0,constants);
        renderer.command_list.SetGraphicsRootDescriptorTable(1,renderer.texture_heap.GetGPUDescriptorHandleForHeapStart());
        renderer.command_list.DrawInstanced(3,1,0,0);
        renderer.render_overlays(64,64);
        renderer.command_list.OMSetRenderTargets(0,None,false,None);
        transition_resource(&renderer.command_list,&output,D3D12_RESOURCE_STATE_RENDER_TARGET,D3D12_RESOURCE_STATE_COPY_SOURCE);
        device.device().CreateCommittedResource(&D3D12_HEAP_PROPERTIES {Type:D3D12_HEAP_TYPE_READBACK,..Default::default()},D3D12_HEAP_FLAG_NONE,
            &D3D12_RESOURCE_DESC {Dimension:D3D12_RESOURCE_DIMENSION_BUFFER,Width:64*256,Height:1,DepthOrArraySize:1,MipLevels:1,SampleDesc:DXGI_SAMPLE_DESC {Count:1,Quality:0},Layout:D3D12_TEXTURE_LAYOUT_ROW_MAJOR,..Default::default()},D3D12_RESOURCE_STATE_COPY_DEST,None,&mut readback).unwrap();
    }
    let readback:ID3D12Resource=readback.unwrap();
    let mut src=D3D12_TEXTURE_COPY_LOCATION {pResource:std::mem::ManuallyDrop::new(Some(output)),Type:D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX,Anonymous:D3D12_TEXTURE_COPY_LOCATION_0 {SubresourceIndex:0}};
    let mut dst=D3D12_TEXTURE_COPY_LOCATION {pResource:std::mem::ManuallyDrop::new(Some(readback.clone())),Type:D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT,Anonymous:D3D12_TEXTURE_COPY_LOCATION_0 {PlacedFootprint:D3D12_PLACED_SUBRESOURCE_FOOTPRINT {Offset:0,Footprint:D3D12_SUBRESOURCE_FOOTPRINT {Format:DXGI_FORMAT_R8G8B8A8_UNORM,Width:64,Height:64,Depth:1,RowPitch:256}}}};
    unsafe {
        renderer.command_list.CopyTextureRegion(&dst,0,0,0,&src,None);
        renderer.command_list.Close().unwrap();
        if let Ok(queue)=device.device().cast::<ID3D12InfoQueue>() {
            for i in 0..queue.GetNumStoredMessagesAllowedByRetrievalFilter() {
                let mut length=0;queue.GetMessage(i,None,&mut length).unwrap();let mut data=vec![0u64;length.div_ceil(8)];let message=data.as_mut_ptr() as *mut D3D12_MESSAGE;
                queue.GetMessage(i,Some(message),&mut length).unwrap();
                assert!((*message).Severity!=D3D12_MESSAGE_SEVERITY_ERROR && (*message).Severity!=D3D12_MESSAGE_SEVERITY_CORRUPTION,"D3D12: {}",std::ffi::CStr::from_ptr((*message).pDescription as *const i8).to_string_lossy());
            }
        }
        device.command_queue().ExecuteCommandLists(&[Some(renderer.command_list.cast().unwrap())]);
        renderer.wait_for_gpu(&device).unwrap();
        std::mem::ManuallyDrop::drop(&mut src.pResource);std::mem::ManuallyDrop::drop(&mut dst.pResource);
        let mut pointer=std::ptr::null_mut();readback.Map(0,None,Some(&mut pointer)).unwrap();
        let bytes=std::slice::from_raw_parts(pointer as *const u8,64*256);
        let result=bytes.chunks_exact(4).map(|p|[p[0],p[1],p[2],p[3]]).collect();
        readback.Unmap(0,None);renderer.finish_completed_frame(&device);result
    }
}
fn pixel(image:&[[u8;4]],x:usize,y:usize)->[u8;3] {image[y*64+x][..3].try_into().unwrap()}
fn near(actual:[u8;3],expected:[u8;3]) {assert!(actual.into_iter().zip(expected).all(|(a,b)|a.abs_diff(b)<=2),"{actual:?} != {expected:?}");}

#[test]
fn overlays_blend_in_linear_space_after_display_and_survive_dropped_assets() {
    for exposure in [0.0,1.0,8.0] {
        let image=render(exposure,|r,d| {
            let red=Texture::solid(Color::RED);let green=Texture::solid(Color::GREEN);
            r.draw_2d(d,&red,Rect::new(0.0,0.0,32.0,32.0),Draw2DOptions::default()).unwrap();
            r.draw_2d(d,&green,Rect::new(0.0,0.0,16.0,16.0),Draw2DOptions {tint:Color::rgba(1.0,1.0,1.0,0.5),..Default::default()}).unwrap();
        });
        near(pixel(&image,8,8),[188,188,0]);near(pixel(&image,24,24),[255,0,0]);
        if exposure==1.0 {near(pixel(&image,60,60),[188,188,188]);}
        if exposure==0.0 {near(pixel(&image,60,60),[0,0,0]);}
    }
}

#[test]
fn sprite_crop_flip_rotation_and_clip_use_physical_pixels() {
    let image=render(0.0,|r,d| {
        let atlas=Texture::rgba(2,2,vec![255,0,0,255,0,255,0,255,0,0,255,255,255,255,255,255]).unwrap();
        r.draw_2d(d,&atlas,Rect::new(0.0,0.0,16.0,16.0),Draw2DOptions {source:Some(Rect::new(0.0,0.0,1.0,1.0)),..Default::default()}).unwrap();
        r.draw_2d(d,&atlas,Rect::new(16.0,0.0,32.0,32.0),Draw2DOptions {flip_horizontally:true,flip_vertically:true,..Default::default()}).unwrap();
        let white=Texture::solid(Color::WHITE);
        r.draw_2d(d,&white,Rect::new(0.0,40.0,16.0,8.0),Draw2DOptions {rotation:crate::math::Angle::degrees(90.0),origin:Vec2::new(8.0,4.0),clip:Some(Rect::new(8.0,36.0,8.0,16.0)),..Default::default()}).unwrap();
        assert!(r.draw_2d(d,&atlas,Rect::new(0.0,0.0,1.0,1.0),Draw2DOptions {source:Some(Rect::new(1.0,0.0,2.0,1.0)),..Default::default()}).is_err());
    });
    near(pixel(&image,15,15),[255,0,0]); // Crop clamps avoid adjacent atlas cells.
    near(pixel(&image,17,1),[255,255,255]);near(pixel(&image,46,30),[255,0,0]);
    near(pixel(&image,9,38),[255,255,255]);near(pixel(&image,6,38),[0,0,0]);near(pixel(&image,13,44),[0,0,0]);
}

#[test]
fn text_coverage_is_straight_alpha_and_zero_sprites_are_noops() {
    let image=render(0.0,|r,d| {
        let mut cache=crate::graphics::text::TextCache::default();
        let label=cache.get("Ox",&TextOptions {font_size:20.0,..Default::default()}).unwrap();
        r.draw_2d(d,&label.texture,Rect::new(2.0,2.0,label.texture.width() as f32,label.texture.height() as f32),
            Draw2DOptions {tint:Color::rgba(0.0,1.0,1.0,0.5),..Default::default()}).unwrap();
        let white=Texture::solid(Color::WHITE);let count=r.overlays.len();
        r.draw_2d(d,&white,Rect::new(0.0,0.0,0.0,10.0),Draw2DOptions::default()).unwrap();
        r.draw_2d(d,&white,Rect::new(0.0,0.0,64.0,64.0),Draw2DOptions {tint:Color::rgba(1.0,1.0,1.0,0.0),..Default::default()}).unwrap();
        r.draw_2d(d,&white,Rect::new(0.0,0.0,64.0,64.0),Draw2DOptions {clip:Some(Rect::default()),..Default::default()}).unwrap();
        assert_eq!(r.overlays.len(),count);
    });
    assert!(image.iter().any(|p|p[1]>=185),"glyph interiors did not reach half opacity");
    assert!(image.iter().any(|p|p[1]>0&&p[1]<180),"missing glyph antialias coverage");
    assert!(image.iter().all(|p|p[0]==0&&p[1]==p[2]&&p[1]<=189),"text tint/coverage blend is incorrect");
}
