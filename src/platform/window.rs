use raw_window_handle::{
    HasWindowHandle,
    RawWindowHandle,
};

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

    pub(crate) fn hwnd(&self) -> HWND {
        let handle = self
            .window
            .window_handle()
            .expect("failed to retrieve native window handle");

        match handle.as_raw() {
            RawWindowHandle::Win32(handle) => {
                HWND(handle.hwnd.get() as *mut _)
            }

            _ => panic!("D3D12 required a Win32 window"),
        }
    }
}