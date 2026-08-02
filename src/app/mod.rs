use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::{
    ecs::ECS,
    input::InputManager,
    renderer::Renderer,
    window::{Window, WindowSpecs},
};

pub struct App {
    pub window: Window,
    pub renderer: Renderer,
    pub input: InputManager,
    pub ecs: ECS,
}

pub struct AppResources<'a> {
    pub window: &'a mut Window,
    pub renderer: &'a mut Renderer,
    pub input: &'a mut InputManager,
}

impl App {
    pub fn new(window_specs: WindowSpecs) -> Self {
        Self {
            window: Window::new(window_specs),
            renderer: Renderer::new(),
            input: InputManager::new(),
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

        let mut res = AppResources {
            window: &mut self.window,
            renderer: &mut self.renderer,
            input: &mut self.input,
        };

        self.ecs.start(&mut res);
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        let mut res = AppResources {
            window: &mut self.window,
            renderer: &mut self.renderer,
            input: &mut self.input,
        };

        res.renderer.clear();
        self.ecs.update(&mut res);
        res.window.request_redraw();
        res.renderer.swap_buffers();
        res.input.clear_input();
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::RedrawRequested => {}
            WindowEvent::KeyboardInput { event, .. } => self.input.process_keyboard(event),
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        match event {
            DeviceEvent::MouseMotion { delta } => self.input.process_mouse_movement(delta),
            _ => {}
        }
    }
}
