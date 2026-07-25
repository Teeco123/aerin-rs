use std::fs;

use aerin_rs::{
    app::{App, AppResources},
    ecs::{ECS, components::Component, entity_manager::Entity, system::SystemTrait},
    window::WindowSpecs,
};

#[allow(dead_code)]
pub struct Position {
    x: f32,
    y: f32,
}

pub struct Speed {
    v: f32,
}

#[derive(Default)]
pub struct TestSystem;

impl Component for Position {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    fn type_of() -> &'static str {
        "Position"
    }
}

impl Component for Speed {
    fn default() -> Self {
        Self { v: 0.0 }
    }
    fn type_of() -> &'static str {
        "Speed"
    }
}

impl SystemTrait for TestSystem {
    fn type_of() -> &'static str
    where
        Self: Sized,
    {
        "TestSystem"
    }

    fn update(&mut self, entities: &mut [Entity], ecs: &mut ECS, res: &mut AppResources) {
        for entity in entities {
            println!("entity: {}", entity);
            res.renderer.use_shader("triangle".to_string());
            res.renderer.draw();
        }
    }
    fn fixed_update(&mut self, _entities: &mut [Entity]) {
        println!("fixed test update")
    }
}

fn main() {
    let specs = WindowSpecs {
        title: "windowwww",
        width: 800,
        height: 600,
    };

    let mut app = App::new(specs);

    let vertex_shader_source: String =
        fs::read_to_string("shaders/vertex.glsl").expect("failed to load file");

    let fragment_shader_source: String =
        fs::read_to_string("shaders/frag.glsl").expect("failed to load file");

    app.renderer.load_shader(
        "triangle".to_string(),
        vertex_shader_source,
        fragment_shader_source,
    );

    app.ecs.create_entity();

    app.ecs.register_component::<Position>();
    app.ecs.register_component::<Speed>();
    app.ecs.register_system::<TestSystem>();

    app.ecs.update_signature::<TestSystem, Position>();
    app.ecs.update_signature::<TestSystem, Speed>();

    app.ecs.insert_component::<Position>(0);
    app.ecs.insert_component::<Speed>(0);

    app.run();
}
