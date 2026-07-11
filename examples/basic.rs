use std::fs;

use aerin_rs::{app::App, ecs::components::Component, window::WindowSpecs};

pub struct Position {
    x: f32,
    y: f32,
}

impl Component for Position {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    fn type_of() -> &'static str {
        "Position"
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

    app.window.renderer.as_mut().unwrap().load_shader(
        "triangle".to_string(),
        vertex_shader_source,
        fragment_shader_source,
    );

    println!("Alive: {}", app.ecs.alive_entities());
    app.ecs.create_entity();
    println!("Has component: {}", app.ecs.has_component::<Position>(0));
    app.ecs.insert_component::<Position>(0);

    app.ecs.register_component::<Position>();
    app.ecs.register_component::<Position>();
    app.ecs.insert_component::<Position>(0);

    println!("Has component: {}", app.ecs.has_component::<Position>(0));

    println!("Alive: {}", app.ecs.alive_entities());
    app.ecs.create_entity();
    app.ecs.create_entity();
    println!("Alive: {}", app.ecs.alive_entities());
    app.ecs.destroy_entity(1);
    println!("Alive: {}", app.ecs.alive_entities());
    app.ecs.create_entity();

    app.run();
}
