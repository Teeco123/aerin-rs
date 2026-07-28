use std::fs;

use aerin_rs::{
    app::{App, AppResources},
    ecs::{ECS, components::Component, entity_manager::Entity, system::SystemTrait},
    renderer::{mesh::Mesh, shader::Shader},
    window::WindowSpecs,
};

pub struct ShaderComponent {
    shader: Option<Shader>,
}

pub struct MeshComponent {
    mesh: Option<Mesh>,
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

impl Component for MeshComponent {
    fn default() -> Self {
        Self { mesh: None }
    }
    fn type_of() -> &'static str {
        "Mesh"
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
        let gl = res.renderer.get_gl();

        let vertex_source: String =
            fs::read_to_string("shaders/vertex.glsl").expect("failed to load file");

        let fragment_source: String =
            fs::read_to_string("shaders/frag.glsl").expect("failed to load file");

        let shader = Shader::new(gl, vertex_source, fragment_source);

        for entity in entities {
            println!("entity: {} start", entity);

            let vertices: [f32; 6] = [0.0, 1.0, -1.0, -1.0, 1.0, -1.0];
            let indices: [u32; 3] = [0, 1, 2];
            let mesh = Mesh::new(gl, &vertices, &indices);

            let shader_component = ecs.get_component::<ShaderComponent>(*entity);
            shader_component.shader = Some(shader.clone());

            let mesh_component = ecs.get_component::<MeshComponent>(*entity);
            mesh_component.mesh = Some(mesh.clone());
        }
    }
    fn update(&mut self, entities: &mut [Entity], ecs: &mut ECS, res: &mut AppResources) {
        let gl = res.renderer.get_gl();

        for entity in entities {
            println!("entity: {} update", entity);
            let component = ecs.get_component::<ShaderComponent>(*entity);
            component.shader.as_ref().unwrap().bind(gl);

            let mesh = ecs.get_component::<MeshComponent>(*entity);
            mesh.mesh.as_ref().unwrap().draw(gl);
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
    app.ecs.register_component::<MeshComponent>();
    app.ecs.register_system::<ShaderSystem>();

    app.ecs.update_signature::<ShaderSystem, ShaderComponent>();
    app.ecs.update_signature::<ShaderSystem, MeshComponent>();

    app.ecs.insert_component::<ShaderComponent>(0);
    app.ecs.insert_component::<MeshComponent>(0);

    app.run();
}
