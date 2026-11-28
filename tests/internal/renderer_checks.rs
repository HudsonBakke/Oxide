use super::*;

#[test]
fn directional_shadow_pipelines_and_depth_views_are_valid_on_d3d12() {
    let device = D3D12Device::warp_for_test().unwrap();
    // Creating the real PSOs checks shader/root-signature compatibility,
    // including comparison sampling and the depth-only output layout.
    let renderer = D3D12Renderer::new(&device).unwrap();
    let mut srv = unsafe { renderer.texture_heap.GetCPUDescriptorHandleForHeapStart() };
    srv.ptr += renderer.texture_descriptor_size as usize;
    for resolution in [128, 1024] {
        let map = ShadowMap::new_array(&device, resolution, 1, srv).unwrap();
        assert_eq!(map.resolution, resolution);
        assert_eq!(unsafe { map.resource.GetDesc() }.Width, resolution as u64);
    }
    let light = DirectionalLight::new(crate::math::Vec3::new(0.0, -1.0, 0.0), crate::graphics::Color::WHITE, 1.0).unwrap();
    let _ = light;
    let layers = TransmissionMap::new(&device, 128, 1, renderer.shadow_cpu(1 + MAX_SHADOW_LIGHTS), renderer.shadow_cpu(1 + 2 * MAX_SHADOW_LIGHTS), renderer.shadow_cpu(1 + 3 * MAX_SHADOW_LIGHTS), renderer.texture_descriptor_size as usize).unwrap();
    assert_eq!(layers.dsvs.len(), TRANSMISSION_LAYERS as usize);
}
