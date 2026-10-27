mod components;
mod systems;

use aerin_app::App;
use aerin_window::WindowConfig;

use crate::{components::QuadComponent, systems::FullscreenTextureSystem};

fn main() {
    let mut app = App::new();

    let window_config = WindowConfig {
        title: "Aerin - Fullscreen Texture".to_string(),
        width: 1964,
        height: 1964,
        ..Default::default()
    };

    app.ecs.register_component::<QuadComponent>();
    app.ecs.register_system::<FullscreenTextureSystem>();
    app.ecs
        .update_signature::<FullscreenTextureSystem, QuadComponent>();

    let entity = app.ecs.create_entity();
    app.ecs.insert_component::<QuadComponent>(entity);

    app.run(window_config);
}
