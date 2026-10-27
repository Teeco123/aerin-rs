use aerin_ecs::ECS;
use aerin_input::Input;
use aerin_renderer::Renderer;
use aerin_window::{Window, WindowApp, WindowConfig, WindowEvent, run_window_app};

pub struct App {
    pub ecs: ECS,
}

impl App {
    pub fn new() -> Self {
        Self { ecs: ECS::new() }
    }

    pub fn run(self, config: WindowConfig) {
        run_window_app(config, self);
    }
}

impl WindowApp for App {
    fn on_start(&mut self, window: Window) {
        let renderer = pollster::block_on(Renderer::new(&window));

        self.ecs.insert_resource(Input::new());
        self.ecs.insert_resource(renderer);
        self.ecs.insert_resource(window);

        self.ecs.start();
    }

    fn on_event(&mut self, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                std::process::exit(0);
            }
            WindowEvent::Resized { width, height } => {
                self.ecs
                    .get_resource_mut::<Renderer>()
                    .resize(width, height);
            }
            WindowEvent::KeyPressed(key) => {
                self.ecs.get_resource_mut::<Input>().press_key(key);
            }
            WindowEvent::KeyReleased(key) => {
                self.ecs.get_resource_mut::<Input>().release_key(key);
            }
            WindowEvent::MouseMoved { delta_x, delta_y } => {
                self.ecs
                    .get_resource_mut::<Input>()
                    .set_mouse_delta(delta_x, delta_y);
            }
            _ => {}
        }
    }

    fn on_update(&mut self) {
        self.ecs.update();
        self.ecs.get_resource_mut::<Input>().clear_frame();
    }
}
