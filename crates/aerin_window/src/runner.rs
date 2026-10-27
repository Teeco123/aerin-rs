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

    fn on_raw_event(&mut self, _window: &Window, _event: &WinitWindowEvent) -> bool {
        false
    }
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

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        if let winit::event::DeviceEvent::MouseMotion { delta } = event {
            self.app.on_event(WindowEvent::MouseMoved {
                delta_x: delta.0 as f32,
                delta_y: delta.1 as f32,
            });
        }
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WinitWindowEvent,
    ) {
        if let Some(window) = &self.window {
            if self.app.on_raw_event(window, &event) {
                return;
            }
        }

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
            WinitWindowEvent::KeyboardInput { event, .. } => {
                use winit::event::ElementState;
                use winit::keyboard::PhysicalKey;

                if let PhysicalKey::Code(winit_code) = event.physical_key {
                    if let Some(key) = to_engine_keycode(winit_code) {
                        match event.state {
                            ElementState::Pressed => {
                                self.app.on_event(WindowEvent::KeyPressed(key))
                            }
                            ElementState::Released => {
                                self.app.on_event(WindowEvent::KeyReleased(key))
                            }
                        }
                    }
                }
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

fn to_engine_keycode(winit_key: winit::keyboard::KeyCode) -> Option<aerin_input::KeyCode> {
    use aerin_input::KeyCode as EKey;
    use winit::keyboard::KeyCode as WKey;

    match winit_key {
        WKey::KeyW => Some(EKey::KeyW),
        WKey::KeyA => Some(EKey::KeyA),
        WKey::KeyS => Some(EKey::KeyS),
        WKey::KeyD => Some(EKey::KeyD),
        WKey::Space => Some(EKey::Space),
        WKey::Escape => Some(EKey::Escape),
        WKey::ShiftLeft => Some(EKey::ShiftLeft),
        WKey::ControlLeft => Some(EKey::ControlLeft),
        _ => None,
    }
}
