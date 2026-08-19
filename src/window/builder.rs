use winit::{dpi::LogicalSize, event_loop::ActiveEventLoop, window::Window as WinitWindow};

use crate::window::Window;

pub struct WindowBuilder {
    title: String,
    width: u32,
    height: u32,
}

impl WindowBuilder {
    pub fn new() -> Self {
        Self {
            title: "Default title".to_string(),
            width: 1280,
            height: 720,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn build(self, event_loop: &ActiveEventLoop) -> Result<Window, winit::error::OsError> {
        let attr = WinitWindow::default_attributes()
            .with_title(self.title)
            .with_inner_size(LogicalSize::new(self.width, self.height));

        let winit_window = event_loop.create_window(attr)?;

        Ok(Window { winit_window })
    }
}
