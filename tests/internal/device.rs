include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/graphics/backend/d3d12/device.rs"));

impl D3D12Device {
    pub(crate) fn warp_for_test() -> Result<Self> {
        enable_debug_layer();
        let factory: IDXGIFactory6 = unsafe { CreateDXGIFactory2(DXGI_CREATE_FACTORY_FLAGS(0))? };
        let adapter: IDXGIAdapter1 = unsafe { factory.EnumWarpAdapter()? };
        let mut device: Option<ID3D12Device> = None;
        unsafe { D3D12CreateDevice(&adapter, D3D_FEATURE_LEVEL_11_0, &mut device)?; }
        let device = device.unwrap();
        let command_queue = unsafe { device.CreateCommandQueue(&D3D12_COMMAND_QUEUE_DESC {
            Type: D3D12_COMMAND_LIST_TYPE_DIRECT,
            ..Default::default()
        })? };
        Ok(Self { device, command_queue })
    }
}
