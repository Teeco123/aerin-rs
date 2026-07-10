use crate::{
    ecs::ECS,
    window::{Window, WindowSpecs},
};

pub struct App {
    pub window: Window,
    pub ecs: ECS,
}

impl App {
    pub fn new(window_specs: WindowSpecs) -> Self {
        Self {
            window: Window::new(window_specs),
            ecs: ECS::new(),
        }
    }
    pub fn run(&mut self) {
        self.window.run();
    }
}
