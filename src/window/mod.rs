pub mod builder;

use winit::{
    dpi::PhysicalSize,
    raw_window_handle::{DisplayHandle, HasDisplayHandle, HasWindowHandle, WindowHandle},
    window::Window as WinitWindow,
};

pub struct Window {
    winit_window: WinitWindow,
}

impl Window {
    pub fn request_redraw(&self) {
        self.winit_window.request_redraw();
    }

    pub fn get_window_handle(&self) -> WindowHandle<'_> {
        self.winit_window.window_handle().unwrap()
    }

    pub fn get_display_handle(&self) -> DisplayHandle<'_> {
        self.winit_window.display_handle().unwrap()
    }

    pub fn get_inner_size(&self) -> PhysicalSize<u32> {
        self.winit_window.inner_size()
    }
}
