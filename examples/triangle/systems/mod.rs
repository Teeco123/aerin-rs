use aerin_ecs::SystemTrait;
use aerin_math::{Vec2, Vec3};
use aerin_renderer::{Mesh, Renderer, Shader, Vertex};

use crate::components::{mesh::MeshComponent, shader::ShaderComponent};

const SHADER_SOURCE: &str = r#"
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) uv: vec2<f32>,
};
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};
@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 1.0);
    out.color = model.color;
    return out;
}
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
"#;

#[derive(Default)]
pub struct TriangleSystem;

impl SystemTrait for TriangleSystem {
    fn type_of() -> &'static str
    where
        Self: Sized,
    {
        "triangle"
    }

    fn start(&mut self, entities: &mut [aerin_ecs::Entity], ecs: &mut aerin_ecs::ECS) {
        let renderer = ecs.get_resource::<Renderer>();

        let vertices = &[
            Vertex {
                position: Vec3::new(0.0, 0.5, 0.0),
                color: Vec3::new(1.0, 0.0, 0.0),
                uv: Vec2::ZERO,
            },
            Vertex {
                position: Vec3::new(-0.5, -0.5, 0.0),
                color: Vec3::new(0.0, 1.0, 0.0),
                uv: Vec2::ZERO,
            },
            Vertex {
                position: Vec3::new(0.5, -0.5, 0.0),
                color: Vec3::new(0.0, 0.0, 1.0),
                uv: Vec2::ZERO,
            },
        ];
        let indices: &[u32] = &[0, 1, 2];
        let mesh = Mesh::new(renderer.device(), vertices, indices);
        let shader = Shader::new(renderer.device(), SHADER_SOURCE, renderer.format(), &[]);

        for entity in entities {
            let mut mesh_cmp = ecs.get_component_mut::<MeshComponent>(*entity);
            mesh_cmp.mesh = Some(mesh.clone());

            let mut shader_cmp = ecs.get_component_mut::<ShaderComponent>(*entity);
            shader_cmp.shader = Some(shader.clone());
        }
    }
    fn update(&mut self, entities: &mut [aerin_ecs::Entity], ecs: &mut aerin_ecs::ECS) {
        let mut renderer = ecs.get_resource_mut::<Renderer>();
        let _ = renderer.render(
            |scene_pass| {
                for entity in entities.iter() {
                    let shader_cmp = ecs.get_component::<ShaderComponent>(*entity);
                    let mesh_cmp = ecs.get_component::<MeshComponent>(*entity);

                    if let (Some(shader), Some(mesh)) = (&shader_cmp.shader, &mesh_cmp.mesh) {
                        shader.bind(scene_pass);
                        mesh.draw(scene_pass);
                    }
                }
            },
            |ui_pass| {},
        );
    }
    fn fixed_update(&mut self, _entities: &mut [aerin_ecs::Entity]) {}
}
