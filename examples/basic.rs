use std::fs;
use std::sync::Arc;

use aerin_app::App;
use aerin_ecs::{Component, ECS, Entity, SystemTrait};
use aerin_input::{Input, KeyCode};
use aerin_math::{Mat4, Vec3};
use aerin_renderer::{
    Mesh, Shader, Vertex,
    glow::{self, HasContext, NativeBuffer},
};
use aerin_window::{Window, WindowConfig};

// -----------------------------------------------------------------------------
// Components
// -----------------------------------------------------------------------------

pub struct CameraComponent {
    pub ubo: Option<NativeBuffer>,
}

impl Component for CameraComponent {
    fn default() -> Self {
        Self { ubo: None }
    }
    fn type_of() -> &'static str {
        "Camera"
    }
}

pub struct ShaderComponent {
    pub shader: Option<Shader>,
}

impl Component for ShaderComponent {
    fn default() -> Self {
        Self { shader: None }
    }
    fn type_of() -> &'static str {
        "Shader"
    }
}

pub struct MeshComponent {
    pub mesh: Option<Mesh>,
}

impl Component for MeshComponent {
    fn default() -> Self {
        Self { mesh: None }
    }
    fn type_of() -> &'static str {
        "Mesh"
    }
}

pub struct PositionComponent {
    pub position: Vec3,
}

impl Component for PositionComponent {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
        }
    }
    fn type_of() -> &'static str {
        "Position"
    }
}

// -----------------------------------------------------------------------------
// Systems (Notice: Zero AppResources, pure (entities, ecs)!)
// -----------------------------------------------------------------------------

#[derive(Default)]
pub struct CameraSystem;

impl SystemTrait for CameraSystem {
    fn type_of() -> &'static str {
        "CameraSystem"
    }

    fn start(&mut self, entities: &mut [Entity], ecs: &mut ECS) {
        // Query the OpenGL context from ECS resources
        let gl = ecs.get_resource::<Arc<glow::Context>>().unwrap().clone();

        for entity in entities {
            let cam_cmp = ecs.get_component::<CameraComponent>(*entity);
            cam_cmp.ubo = Some(Shader::create_ubo(&gl, 0, 128));
        }
    }

    fn update(&mut self, entities: &mut [Entity], ecs: &mut ECS) {
        // 1. Read resources upfront (releases borrow of ecs immediately!)
        let gl = ecs.get_resource::<Arc<glow::Context>>().unwrap().clone();
        let window = ecs.get_resource::<Window>().unwrap().clone();
        let (width, height) = window.inner_size();

        let input = ecs.get_resource::<Input>().unwrap();
        let mut move_x = 0.0;
        let mut move_z = 0.0;
        if input.is_key_held(KeyCode::KeyA) {
            move_x -= 0.1;
        }
        if input.is_key_held(KeyCode::KeyD) {
            move_x += 0.1;
        }
        if input.is_key_held(KeyCode::KeyW) {
            move_z -= 0.1;
        }
        if input.is_key_held(KeyCode::KeyS) {
            move_z += 0.1;
        }

        // 2. Mutate components & update UBO
        for entity in entities {
            let pos = ecs.get_component::<PositionComponent>(*entity);
            pos.position.x += move_x;
            pos.position.z += move_z;

            let view =
                Mat4::translate(Vec3::new(-pos.position.x, -pos.position.y, -pos.position.z))
                    .to_array();
            let projection =
                Mat4::projection(60.0, width as f32, height as f32, 0.5, 20.0).to_array();

            let cam_comp = ecs.get_component::<CameraComponent>(*entity);

            unsafe {
                gl.bind_buffer(glow::UNIFORM_BUFFER, cam_comp.ubo);

                let view_bytes: &[u8] = core::slice::from_raw_parts(view.as_ptr() as *const u8, 64);
                gl.buffer_sub_data_u8_slice(glow::UNIFORM_BUFFER, 0, view_bytes);

                let projection_bytes: &[u8] =
                    core::slice::from_raw_parts(projection.as_ptr() as *const u8, 64);
                gl.buffer_sub_data_u8_slice(glow::UNIFORM_BUFFER, 64, projection_bytes);

                gl.bind_buffer(glow::UNIFORM_BUFFER, None);
            }
        }
    }

    fn fixed_update(&mut self, _entities: &mut [Entity]) {}
}

#[derive(Default)]
pub struct ShaderSystem;

impl SystemTrait for ShaderSystem {
    fn type_of() -> &'static str {
        "ShaderSystem"
    }

    fn start(&mut self, entities: &mut [Entity], ecs: &mut ECS) {
        let gl = ecs.get_resource::<Arc<glow::Context>>().unwrap().clone();

        let vertex_source =
            fs::read_to_string("shaders/vertex.glsl").expect("Failed to load vertex shader");
        let fragment_source =
            fs::read_to_string("shaders/frag.glsl").expect("Failed to load frag shader");

        let shader = Shader::new(&gl, &vertex_source, &fragment_source);

        let vertices = vec![
            Vertex {
                position: Vec3::new(0.0, 1.0, 0.0),
                color: Vec3::new(1.0, 0.0, 0.0),
            },
            Vertex {
                position: Vec3::new(-1.0, -1.0, 0.0),
                color: Vec3::new(0.0, 1.0, 0.0),
            },
            Vertex {
                position: Vec3::new(1.0, -1.0, 0.0),
                color: Vec3::new(0.0, 0.0, 1.0),
            },
        ];
        let indices = vec![0, 1, 2];

        let mesh = Mesh::new(&gl, &vertices, &indices);

        for entity in entities {
            let shader_component = ecs.get_component::<ShaderComponent>(*entity);
            shader_component.shader = Some(shader.clone());

            let mesh_component = ecs.get_component::<MeshComponent>(*entity);
            mesh_component.mesh = Some(mesh.clone());
        }
    }

    fn update(&mut self, entities: &mut [Entity], ecs: &mut ECS) {
        let gl = ecs.get_resource::<Arc<glow::Context>>().unwrap().clone();
        let model = Mat4::rotate_z(90.0_f32.to_radians());

        for entity in entities {
            let component = ecs.get_component::<ShaderComponent>(*entity);
            component.shader.as_ref().unwrap().bind(&gl);
            component
                .shader
                .as_ref()
                .unwrap()
                .set_uniform_mat4(&gl, "u_model", &model.to_array());

            let mesh = ecs.get_component::<MeshComponent>(*entity);
            mesh.mesh.as_ref().unwrap().draw(&gl);
        }
    }

    fn fixed_update(&mut self, _entities: &mut [Entity]) {}
}

// -----------------------------------------------------------------------------
// Main Entry Point
// -----------------------------------------------------------------------------

fn main() {
    let mut app = App::new();

    // 1. Entities
    app.ecs.create_entity();
    app.ecs.create_entity();

    // 2. Components
    app.ecs.register_component::<ShaderComponent>();
    app.ecs.register_component::<MeshComponent>();
    app.ecs.register_component::<PositionComponent>();
    app.ecs.register_component::<CameraComponent>();

    // 3. Systems
    app.ecs.register_system::<ShaderSystem>();
    app.ecs.register_system::<CameraSystem>();

    // 4. Signatures
    app.ecs.update_signature::<ShaderSystem, ShaderComponent>();
    app.ecs.update_signature::<ShaderSystem, MeshComponent>();
    app.ecs
        .update_signature::<ShaderSystem, PositionComponent>();

    app.ecs.update_signature::<CameraSystem, CameraComponent>();
    app.ecs
        .update_signature::<CameraSystem, PositionComponent>();

    // 5. Attach Components to Entity 0 (Triangle)
    app.ecs.insert_component::<ShaderComponent>(0);
    app.ecs.insert_component::<MeshComponent>(0);
    app.ecs.insert_component::<PositionComponent>(0);

    // 6. Attach Components to Entity 1 (Camera)
    app.ecs.insert_component::<PositionComponent>(1);
    app.ecs.insert_component::<CameraComponent>(1);

    // 7. Run Windowed App
    let config = WindowConfig {
        title: "Aerin Engine (Modular)".to_string(),
        width: 1920,
        height: 1080,
        ..Default::default()
    };

    app.run(config);
}
