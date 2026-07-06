use std::fs;

use app::App;
use window::WindowSpecs;

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

    app.run();
}
