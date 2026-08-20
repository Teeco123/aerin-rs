pub mod builder;

use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::{
    ecs::ECS,
    input::InputManager,
    renderer::{Renderer, builder::RendererBuilder},
    window::{Window, builder::WindowBuilder},
};

pub struct App {
    window_builder: Option<WindowBuilder>,
    renderer_builder: Option<RendererBuilder>,
    pub window: Option<Window>,
    pub renderer: Option<Renderer>,
    pub input: InputManager,
    pub ecs: ECS,
}

pub struct AppResources<'a> {
    pub window: &'a mut Window,
    pub renderer: &'a mut Renderer,
    pub input: &'a mut InputManager,
}

impl App {
    pub fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        let _ = event_loop.run_app(self);
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let builder = self.window_builder.take().unwrap();
            let window = builder.build(event_loop);

            self.window = Some(window.unwrap())
        }
        let window = self.window.as_ref().unwrap();
        let window_handle = window.get_window_handle();
        let display_handle = window.get_display_handle();
        let inner_size = window.get_inner_size();

        if self.renderer.is_none() {
            let builder = self
                .renderer_builder
                .take()
                .unwrap()
                .with_size(inner_size.width, inner_size.height)
                .with_display_handle(display_handle.as_raw())
                .with_window_handle(window_handle.as_raw());

            let renderer = builder.build();
            self.renderer = Some(renderer.unwrap())
        }

        let mut res = AppResources {
            window: &mut self.window.as_mut().unwrap(),
            renderer: &mut self.renderer.as_mut().unwrap(),
            input: &mut self.input,
        };

        self.ecs.start(&mut res);
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        let mut res = AppResources {
            window: &mut self.window.as_mut().unwrap(),
            renderer: &mut self.renderer.as_mut().unwrap(),
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
            WindowEvent::Resized(size) => {
                self.renderer
                    .as_ref()
                    .unwrap()
                    .resize(size.width, size.height);
            }
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
