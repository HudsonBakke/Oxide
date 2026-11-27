include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/graphics/backend/d3d12/device.rs"));

impl D3D12Device {
    pub(crate) fn warp_for_test() -> Result<Self> {
        static DEBUG_INIT: std::sync::Once = std::sync::Once::new();
        DEBUG_INIT.call_once(|| unsafe {
            let mut debug: Option<ID3D12Debug> = None;
            if D3D12GetDebugInterface(&mut debug).is_ok() {
                if let Some(debug) = debug { debug.EnableDebugLayer(); }
            }
        });
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
