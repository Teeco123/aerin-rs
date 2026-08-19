use crate::{
    app::App, ecs::ECS, input::InputManager, renderer::Renderer, window::builder::WindowBuilder,
};

pub struct AppBuilder {
    window_builder: Option<WindowBuilder>,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self {
            window_builder: None,
        }
    }

    pub fn with_window(mut self, window_builder: WindowBuilder) -> Self {
        self.window_builder = Some(window_builder);
        self
    }

    pub fn build(self) -> Result<App, winit::error::OsError> {
        return Ok(App {
            window_builder: self.window_builder,
            window: None,
            renderer: Renderer::new(),
            input: InputManager::new(),
            ecs: ECS::new(),
        });
    }
}
