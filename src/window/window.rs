use winit::{
    dpi::LogicalSize,
    event_loop::ActiveEventLoop,
    raw_window_handle::{DisplayHandle, HasDisplayHandle, HasWindowHandle, WindowHandle},
    window::Window as WinitWindow,
};

use super::renderer::Renderer;

pub struct WindowSpecs {
    pub title: &'static str,
    pub width: i32,
    pub height: i32,
}

pub struct Window {
    window: Option<WinitWindow>,
    window_specs: Option<WindowSpecs>,
    pub renderer: Option<Renderer>,
}

impl Window {
    pub fn new(window_specs: WindowSpecs) -> Self {
        Self {
            window: None,
            window_specs: Some(window_specs),
            renderer: Some(Renderer::new()),
        }
    }

    pub fn init(&mut self, event_loop: &ActiveEventLoop) {
        let window_specs = self.window_specs.as_ref().unwrap();

        let winit_window_attr = WinitWindow::default_attributes()
            .with_title(window_specs.title)
            .with_inner_size(LogicalSize::new(window_specs.width, window_specs.height));

        let window = event_loop.create_window(winit_window_attr).unwrap();
        self.window = Some(window);

        let wh = self.window.as_ref().unwrap().window_handle().unwrap();
        let dh = self.window.as_ref().unwrap().display_handle().unwrap();
        self.renderer.as_mut().unwrap().create(wh, dh);

        self.renderer.as_mut().unwrap().compile_shaders();
    }

    pub fn request_redraw(&self) {
        self.window.as_ref().unwrap().request_redraw();
    }

    pub fn draw(&self) {
        self.renderer.as_ref().unwrap().draw();
    }

    pub fn get_window_handle(&self) -> WindowHandle<'_> {
        self.window.as_ref().unwrap().window_handle().unwrap()
    }

    pub fn get_display_handle(&self) -> DisplayHandle<'_> {
        self.window.as_ref().unwrap().display_handle().unwrap()
    }
}
