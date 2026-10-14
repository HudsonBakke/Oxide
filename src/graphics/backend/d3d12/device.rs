use windows::{
    core::Result,
    Win32::Graphics::{
        Direct3D::D3D_FEATURE_LEVEL_11_0,
        Direct3D12::{
            D3D12GetDebugInterface,
            ID3D12Debug,
            D3D12CreateDevice,
            ID3D12CommandQueue,
            ID3D12Device,
            D3D12_COMMAND_LIST_TYPE_DIRECT,
            D3D12_COMMAND_QUEUE_DESC,
            D3D12_COMMAND_QUEUE_FLAG_NONE,
            D3D12_COMMAND_QUEUE_PRIORITY_NORMAL,
        },
        Dxgi::{
            CreateDXGIFactory2,
            IDXGIAdapter1,
            IDXGIFactory6,
            DXGI_ADAPTER_DESC1,
            DXGI_ADAPTER_FLAG_SOFTWARE,
            DXGI_CREATE_FACTORY_FLAGS,
            DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE,
        },
    },
};

pub(crate) struct D3D12Device {
    device: ID3D12Device,
    command_queue: ID3D12CommandQueue,
}

fn select_adapter() -> Result<IDXGIAdapter1> {
    let factory: IDXGIFactory6 = unsafe {
        CreateDXGIFactory2(DXGI_CREATE_FACTORY_FLAGS(0))?
    };

    let mut index = 0;

    loop {
        let adapter: IDXGIAdapter1 = unsafe {
            factory.EnumAdapterByGpuPreference(
                index,
                DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE,
            )?
        };

        let description = unsafe {
            adapter.GetDesc1()?
        };

        if description.Flags & DXGI_ADAPTER_FLAG_SOFTWARE.0 as u32 != 0 {
            index += 1;
            continue;
        }

        let mut test_device: Option<ID3D12Device> = None;

        let supported = unsafe {
            D3D12CreateDevice(
                &adapter,
                D3D_FEATURE_LEVEL_11_0,
                &mut test_device,
            )
            .is_ok()
        };

        if supported {
            println!(
                "Oxide: selected graphics adapter: {}",
                adapter_name(&description)
            );

            return Ok(adapter);
        }

        index += 1;
    }
}

fn adapter_name(description: &DXGI_ADAPTER_DESC1) -> String {
    let length = description
        .Description
        .iter()
        .position(|&c| c == 0)
        .unwrap_or(description.Description.len());

    String::from_utf16_lossy(
        &description.Description[..length]
    )
}

impl D3D12Device {
    pub(crate) fn new() -> Result<Self> {
        #[cfg(debug_assertions)]
        unsafe {
            let mut debug: Option<ID3D12Debug> = None;

            D3D12GetDebugInterface(&mut debug)?;

            let debug = debug.expect(
                "D3D12GetDebugInterface succeeded but returned no debug interface"
            );

            debug.EnableDebugLayer();
        }

        let adapter = select_adapter()?;

        let mut device: Option<ID3D12Device> = None;

        unsafe {
            D3D12CreateDevice(
                &adapter,
                D3D_FEATURE_LEVEL_11_0,
                &mut device,
            )?;
        }

        let device = device.expect(
            "D3D12CreateDevice succeeded but returned no device"
        );

        let queue_desc = D3D12_COMMAND_QUEUE_DESC {
            Type: D3D12_COMMAND_LIST_TYPE_DIRECT,
            Priority: D3D12_COMMAND_QUEUE_PRIORITY_NORMAL.0,
            Flags: D3D12_COMMAND_QUEUE_FLAG_NONE,
            NodeMask: 0,
        };

            let command_queue = unsafe {
                device.CreateCommandQueue(&queue_desc)?
            };

        Ok(Self { 
            device,
            command_queue
        })
    }

    pub(crate) fn command_queue(&self) -> &ID3D12CommandQueue {
        &self.command_queue
    }

    pub(crate) fn device(&self) -> &ID3D12Device {
        &self.device
    }
}