use winit::{
    application::ApplicationHandler,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

pub struct WinitApp {
    window: Option<Window>,
}

impl WinitApp {
    fn new() -> Self {
        Self { window: None }
    }
}

impl ApplicationHandler for WinitApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        println!("winit resumed");
        let winit_window_attributes = Window::default_attributes();
        self.window = Some(event_loop.create_window(winit_window_attributes).unwrap());
    }
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
    }
}

pub fn run_winit() {
    println!("winit run");
    let mut winit_app = WinitApp::new();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let _ = event_loop.run_app(&mut winit_app);
}
