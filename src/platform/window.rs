use raw_window_handle::{HasWindowHandle, RawWindowHandle};

use windows::Win32::Foundation::HWND;
use winit::window::Window;

pub(crate) struct PlatformWindow {
    window: Window,
}

impl PlatformWindow {
    pub(crate) fn new(window: Window) -> Self {
        Self { window }
    }

    pub(crate) fn inner(&self) -> &Window {
        &self.window
    }

    pub(crate) fn hwnd(&self) -> windows::core::Result<HWND> {
        let handle = self
            .window
            .window_handle()
            .map_err(|error| windows::core::Error::new(windows::Win32::Foundation::E_FAIL,
                format!("failed to retrieve native window handle: {error}")))?;

        match handle.as_raw() {
            RawWindowHandle::Win32(handle) => Ok(HWND(handle.hwnd.get() as *mut _)),

            _ => Err(windows::core::Error::new(windows::Win32::Foundation::E_INVALIDARG, "D3D12 requires a Win32 window")),
        }
    }
}
