use aerin_ecs::ECS;
use aerin_input::Input;
use aerin_renderer::Renderer;
use aerin_window::{Window, WindowApp, WindowConfig, WindowEvent, run_window_app};

pub struct App {
    pub ecs: ECS,
    pub renderer: Option<Renderer>,
    pub window: Option<Window>,
}

impl App {
    pub fn new() -> Self {
        let mut ecs = ECS::new();
        ecs.insert_resource(Input::new());

        Self {
            ecs,
            renderer: None,
            window: None,
        }
    }

    pub fn run(self, config: WindowConfig) {
        run_window_app(config, self);
    }
}

impl WindowApp for App {
    fn on_start(&mut self, window: Window) {
        let renderer = pollster::block_on(Renderer::new(window.clone()));
        self.ecs.insert_resource(window.clone());
        self.ecs.insert_resource(renderer.gl().clone());
        self.renderer = Some(renderer);
        self.window = Some(window);
        self.ecs.start();
    }

    fn on_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                std::process::exit(0);
            }
            WindowEvent::Resized { width, height } => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(width, height);
                }
            }
            WindowEvent::KeyPressed(key) => {
                if let Some(input) = self.ecs.get_resource_mut::<Input>() {
                    input.press_key(key);
                }
            }
            WindowEvent::KeyReleased(key) => {
                if let Some(input) = self.ecs.get_resource_mut::<Input>() {
                    input.release_key(key);
                }
            }
            WindowEvent::MouseMoved { delta_x, delta_y } => {
                if let Some(input) = self.ecs.get_resource_mut::<Input>() {
                    input.set_mouse_delta(delta_x, delta_y);
                }
            }
            _ => {}
        }
    }

    fn on_update(&mut self) {
        let renderer = self.renderer.as_ref().unwrap();

        renderer.clear(0.1, 0.1, 0.15, 1.0);

        self.ecs.update();

        renderer.swap_buffers();

        if let Some(input) = self.ecs.get_resource_mut::<Input>() {
            input.clear_frame();
        }
    }
}
