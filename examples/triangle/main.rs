mod components;
mod systems;

use aerin_app::App;
use aerin_window::WindowConfig;

use crate::{
    components::{mesh::MeshComponent, shader::ShaderComponent},
    systems::TriangleSystem,
};

fn main() {
    let mut app = App::new();

    let window_config = WindowConfig::default();

    app.ecs.register_component::<ShaderComponent>();
    app.ecs.register_component::<MeshComponent>();

    app.ecs.register_system::<TriangleSystem>();
    app.ecs
        .update_signature::<TriangleSystem, ShaderComponent>();
    app.ecs.update_signature::<TriangleSystem, MeshComponent>();

    let entity_1 = app.ecs.create_entity();
    app.ecs.insert_component::<ShaderComponent>(entity_1);
    app.ecs.insert_component::<MeshComponent>(entity_1);

    app.run(window_config);
}
