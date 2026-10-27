use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle,
};
use std::sync::Arc;
use winit::window::Window as WinitWindow;

#[derive(Clone)]
pub struct Window {
    inner: Arc<WinitWindow>,
}

impl Window {
    pub(crate) fn new(winit_window: WinitWindow) -> Self {
        Self {
            inner: Arc::new(winit_window),
        }
    }

    pub fn winit_window(&self) -> &WinitWindow {
        &self.inner
    }

    pub fn inner_size(&self) -> (u32, u32) {
        let size = self.inner.inner_size();
        (size.width, size.height)
    }

    pub fn width(&self) -> u32 {
        self.inner.inner_size().width
    }

    pub fn height(&self) -> u32 {
        self.inner.inner_size().height
    }

    pub fn request_redraw(&self) {
        self.inner.request_redraw();
    }

    pub fn set_title(&self, title: &str) {
        self.inner.set_title(title);
    }

    pub fn set_visible(&self, visible: bool) {
        self.inner.set_visible(visible);
    }
}

impl HasWindowHandle for Window {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        self.inner.window_handle()
    }
}

impl HasDisplayHandle for Window {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        self.inner.display_handle()
    }
}
