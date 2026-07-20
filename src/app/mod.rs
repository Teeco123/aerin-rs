use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
};

use crate::{
    ecs::ECS,
    renderer::Renderer,
    window::{Window, WindowSpecs},
};

pub struct App {
    pub window: Window,
    pub renderer: Renderer,
    pub ecs: ECS,
}

impl App {
    pub fn new(window_specs: WindowSpecs) -> Self {
        Self {
            window: Window::new(window_specs),
            renderer: Renderer::new(),
            ecs: ECS::new(),
        }
    }

    pub fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        let _ = event_loop.run_app(self);
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.window.init(event_loop);
        let window_handle = self.window.get_window_handle();
        let display_handle = self.window.get_display_handle();
        self.renderer.init(window_handle, display_handle);
        self.renderer.compile_shaders();
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        self.ecs.update();
        self.window.request_redraw();
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::RedrawRequested => {
                self.renderer.draw();
            }
            _ => {}
        }
    }
}
