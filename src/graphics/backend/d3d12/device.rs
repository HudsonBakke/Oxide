use windows::{
    Win32::Graphics::{
        Direct3D::D3D_FEATURE_LEVEL_11_0,
        Direct3D12::{
            D3D12_COMMAND_LIST_TYPE_DIRECT, D3D12_COMMAND_QUEUE_DESC,
            D3D12_COMMAND_QUEUE_FLAG_NONE, D3D12_COMMAND_QUEUE_PRIORITY_NORMAL, D3D12CreateDevice,
            D3D12GetDebugInterface, ID3D12CommandQueue, ID3D12Debug, ID3D12Device,
        },
        Dxgi::{
            CreateDXGIFactory2, DXGI_ADAPTER_DESC1, DXGI_ADAPTER_FLAG_SOFTWARE,
            DXGI_CREATE_FACTORY_FLAGS, DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE, DXGI_ERROR_NOT_FOUND, IDXGIAdapter1,
            IDXGIFactory6,
        },
    },
    core::Result,
};

pub(crate) struct D3D12Device {
    device: ID3D12Device,
    command_queue: ID3D12CommandQueue,
}

fn select_adapter() -> Result<IDXGIAdapter1> {
    let factory: IDXGIFactory6 = unsafe { CreateDXGIFactory2(DXGI_CREATE_FACTORY_FLAGS(0))? };

    let mut index = 0;

    loop {
        let adapter: IDXGIAdapter1 = match unsafe {
            factory.EnumAdapterByGpuPreference(index, DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE)
        } {
            Ok(adapter) => adapter,
            Err(error) if error.code() == DXGI_ERROR_NOT_FOUND => {
                return Err(windows::core::Error::new(error.code(),
                    "no Direct3D 12-capable hardware graphics adapter was found"));
            }
            Err(error) => return Err(error),
        };

        let description = unsafe { adapter.GetDesc1()? };

        if description.Flags & DXGI_ADAPTER_FLAG_SOFTWARE.0 as u32 != 0 {
            index += 1;
            continue;
        }

        let mut test_device: Option<ID3D12Device> = None;

        let supported = unsafe {
            D3D12CreateDevice(&adapter, D3D_FEATURE_LEVEL_11_0, &mut test_device).is_ok()
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

    String::from_utf16_lossy(&description.Description[..length])
}

impl D3D12Device {
    pub(crate) fn new() -> Result<Self> {
        enable_debug_layer();

        let adapter = select_adapter()?;

        let mut device: Option<ID3D12Device> = None;

        unsafe {
            D3D12CreateDevice(&adapter, D3D_FEATURE_LEVEL_11_0, &mut device)?;
        }

        let device = device.ok_or_else(|| windows::core::Error::new(
            windows::Win32::Foundation::E_FAIL, "D3D12CreateDevice returned no device"))?;

        let queue_desc = D3D12_COMMAND_QUEUE_DESC {
            Type: D3D12_COMMAND_LIST_TYPE_DIRECT,
            Priority: D3D12_COMMAND_QUEUE_PRIORITY_NORMAL.0,
            Flags: D3D12_COMMAND_QUEUE_FLAG_NONE,
            NodeMask: 0,
        };

        let command_queue = unsafe { device.CreateCommandQueue(&queue_desc)? };

        Ok(Self {
            device,
            command_queue,
        })
    }



    pub(crate) fn command_queue(&self) -> &ID3D12CommandQueue {
        &self.command_queue
    }

    pub(crate) fn device(&self) -> &ID3D12Device {
        &self.device
    }
}

// Debug diagnostics are optional. Initialize once, before any device is created;
// enabling the layer a second time could invalidate existing D3D12 devices.
fn enable_debug_layer() {
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| {
        if cfg!(debug_assertions) {
            let mut debug: Option<ID3D12Debug> = None;
            if unsafe { D3D12GetDebugInterface(&mut debug) }.is_ok() {
                if let Some(debug) = debug { unsafe { debug.EnableDebugLayer() }; }
            }
        }
    });
}
