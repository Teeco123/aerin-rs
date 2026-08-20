use crate::{
    app::App, ecs::ECS, input::InputManager, renderer::builder::RendererBuilder,
    window::builder::WindowBuilder,
};

pub struct AppBuilder {
    window_builder: Option<WindowBuilder>,
    renderer_builder: Option<RendererBuilder>,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self {
            window_builder: None,
            renderer_builder: None,
        }
    }

    pub fn with_window(mut self, window_builder: WindowBuilder) -> Self {
        self.window_builder = Some(window_builder);
        self
    }

    pub fn with_renderer(mut self, renderer_builder: RendererBuilder) -> Self {
        self.renderer_builder = Some(renderer_builder);
        self
    }

    pub fn build(self) -> Result<App, winit::error::OsError> {
        return Ok(App {
            window_builder: self.window_builder,
            renderer_builder: self.renderer_builder,
            window: None,
            renderer: None,
            input: InputManager::new(),
            ecs: ECS::new(),
        });
    }
}
