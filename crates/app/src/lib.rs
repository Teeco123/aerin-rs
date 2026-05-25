use window::Window;

pub struct App {
    window: Window,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: Window::new(),
        }
    }
    pub fn run(&mut self) {
        println!("App run");

        self.window.run();
    }
}
