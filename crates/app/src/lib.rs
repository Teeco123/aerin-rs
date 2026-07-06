use window::{Window, WindowSpecs};

pub struct App {
    window: Window,
}

impl App {
    pub fn new(window_specs: WindowSpecs) -> Self {
        Self {
            window: Window::new(window_specs),
        }
    }
    pub fn run(&mut self) {
        println!("App run");

        self.window.run();
    }
}
