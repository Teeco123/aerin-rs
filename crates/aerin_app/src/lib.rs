use aerin_ecs::ECS;
use aerin_input::Input;
use aerin_renderer::Renderer;
use aerin_window::{Window, WindowApp, WindowConfig, WindowEvent, run_window_app};

pub struct App {
    pub ecs: ECS,
    pub input: Input,
    pub renderer: Option<Renderer>,
    pub window: Option<Window>,
}

impl App {
    pub fn new() -> Self {
        Self {
            ecs: ECS::new(),
            input: Input::new(),
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
        let renderer = pollster::block_on(Renderer::new(&window));

        self.renderer = Some(renderer);
        self.window = Some(window);

        self.ecs.as_mut().unwrap().start();
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
                self.input.press_key(key);
            }
            WindowEvent::KeyReleased(key) => {
                self.input.release_key(key);
            }
            WindowEvent::MouseMoved { delta_x, delta_y } => {
                self.input.set_mouse_delta(delta_x, delta_y);
            }
            _ => {}
        }
    }

    fn on_update(&mut self) {
        self.ecs.update();

        input.clear_frame();
    }
}
