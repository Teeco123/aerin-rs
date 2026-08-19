pub mod builder;

use winit::{
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
}
