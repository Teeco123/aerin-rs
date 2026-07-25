use aerin_rs::{
    app::{App, AppResources},
    ecs::{ECS, components::Component, entity_manager::Entity, system::SystemTrait},
    renderer::shader::Shader,
    window::WindowSpecs,
};

#[allow(dead_code)]
pub struct ShaderComponent {
    shader: Option<Shader>,
}

#[derive(Default)]
pub struct ShaderSystem;

impl Component for ShaderComponent {
    fn default() -> Self {
        Self { shader: None }
    }
    fn type_of() -> &'static str {
        "Shader"
    }
}

impl SystemTrait for ShaderSystem {
    fn type_of() -> &'static str
    where
        Self: Sized,
    {
        "ShaderSystem"
    }

    fn start(&mut self, entities: &mut [Entity], ecs: &mut ECS, res: &mut AppResources) {
        for entity in entities {
            println!("entity: {} start", entity);
        }
    }
    fn update(&mut self, entities: &mut [Entity], _ecs: &mut ECS, _res: &mut AppResources) {
        for entity in entities {
            println!("entity: {} update", entity);
        }
    }
    fn fixed_update(&mut self, _entities: &mut [Entity]) {}
}

fn main() {
    let specs = WindowSpecs {
        title: "windowwww",
        width: 800,
        height: 600,
    };

    let mut app = App::new(specs);

    app.ecs.create_entity();

    app.ecs.register_component::<ShaderComponent>();
    app.ecs.register_system::<ShaderSystem>();

    app.ecs.update_signature::<ShaderSystem, ShaderComponent>();

    app.ecs.insert_component::<ShaderComponent>(0);

    app.run();
}
