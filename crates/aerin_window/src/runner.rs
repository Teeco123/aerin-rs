use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent as WinitWindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window as WinitWindow, WindowId},
};

use crate::{config::WindowConfig, events::WindowEvent, window::Window};

pub trait WindowApp {
    fn on_start(&mut self, window: Window);
    fn on_event(&mut self, event: WindowEvent);
    fn on_update(&mut self);
}

pub struct WinitRunner<A: WindowApp> {
    config: WindowConfig,
    app: A,
    window: Option<Window>,
}

impl<A: WindowApp> WinitRunner<A> {
    pub fn new(config: WindowConfig, app: A) -> Self {
        Self {
            config,
            app,
            window: None,
        }
    }
}

impl<A: WindowApp> ApplicationHandler for WinitRunner<A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let attributes = WinitWindow::default_attributes()
                .with_title(&self.config.title)
                .with_inner_size(PhysicalSize::new(self.config.width, self.config.height))
                .with_resizable(self.config.resizable)
                .with_visible(self.config.visible);

            let raw_window = event_loop
                .create_window(attributes)
                .expect("Failed to create winit window");

            let window = Window::new(raw_window);
            self.window = Some(window.clone());
            self.app.on_start(window);
        }
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WinitWindowEvent,
    ) {
        match event {
            WinitWindowEvent::CloseRequested => {
                self.app.on_event(WindowEvent::CloseRequested);
            }
            WinitWindowEvent::Resized(size) => {
                self.app.on_event(WindowEvent::Resized {
                    width: size.width,
                    height: size.height,
                });
            }
            WinitWindowEvent::Focused(focused) => {
                self.app.on_event(WindowEvent::Focused(focused));
            }
            WinitWindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.app
                    .on_event(WindowEvent::ScaleFactorChanged(scale_factor));
            }
            WinitWindowEvent::RedrawRequested => {
                self.app.on_event(WindowEvent::RedrawRequested);
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.app.on_update();

        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

pub fn run_window_app<A: WindowApp + 'static>(config: WindowConfig, app: A) {
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut runner = WinitRunner::new(config, app);
    event_loop.run_app(&mut runner).expect("Event loop error");
}
