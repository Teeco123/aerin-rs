use std::fs;

use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    raw_window_handle::{DisplayHandle, HasDisplayHandle, HasWindowHandle, WindowHandle},
    window::Window as WinitWindow,
};

use crate::renderer::Renderer;

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
    pub fn run(&mut self) {
        println!("run_event_loop");

        let vertex_shader_source: String =
            fs::read_to_string("shaders/vertex.glsl").expect("failed to load file");

        let fragment_shader_source: String =
            fs::read_to_string("shaders/frag.glsl").expect("failed to load file");

        self.renderer.as_mut().unwrap().load_shader(
            "triangle".to_string(),
            vertex_shader_source,
            fragment_shader_source,
        );

        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        let _ = event_loop.run_app(self);
    }
    pub fn get_window_handle(&self) -> WindowHandle<'_> {
        self.window.as_ref().unwrap().window_handle().unwrap()
    }
    pub fn get_display_handle(&self) -> DisplayHandle<'_> {
        self.window.as_ref().unwrap().display_handle().unwrap()
    }
}

impl ApplicationHandler for Window {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        println!("Winit App resumed");
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
    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::RedrawRequested => {
                self.renderer.as_ref().unwrap().draw();
            }
            _ => {}
        }
    }
}
