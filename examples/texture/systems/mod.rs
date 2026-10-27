use aerin_ecs::{ECS, Entity, SystemTrait};
use aerin_math::{Vec2, Vec3};
use aerin_renderer::{Mesh, Renderer, Shader, Texture, Vertex};

use crate::components::QuadComponent;

fn load_image(path: &str) -> (u32, u32, Vec<u8>) {
    let img = image::open(path)
        .or_else(|_| image::open(format!("examples/texture/{path}")))
        .unwrap();

    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    (w, h, rgba.into_raw())
}

const SHADER_SOURCE: &str = r#"
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 1.0);
    out.uv = model.uv;
    return out;
}

@group(0) @binding(0) var t_diffuse: texture_2d<f32>;
@group(0) @binding(1) var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.uv);
}
"#;

#[derive(Default)]
pub struct FullscreenTextureSystem;

impl SystemTrait for FullscreenTextureSystem {
    fn type_of() -> &'static str {
        "fullscreen_texture"
    }

    fn start(&mut self, entities: &mut [Entity], ecs: &mut ECS) {
        let renderer = ecs.get_resource::<Renderer>();

        let vertices = &[
            Vertex {
                position: Vec3::new(-1.0, 1.0, 0.0),
                color: Vec3::ONE,
                uv: Vec2::new(0.0, 0.0),
            },
            Vertex {
                position: Vec3::new(-1.0, -1.0, 0.0),
                color: Vec3::ONE,
                uv: Vec2::new(0.0, 1.0),
            },
            Vertex {
                position: Vec3::new(1.0, -1.0, 0.0),
                color: Vec3::ONE,
                uv: Vec2::new(1.0, 1.0),
            },
            Vertex {
                position: Vec3::new(1.0, 1.0, 0.0),
                color: Vec3::ONE,
                uv: Vec2::new(1.0, 0.0),
            },
        ];
        let indices: &[u32] = &[0, 1, 2, 0, 2, 3];
        let mesh = Mesh::new(renderer.device(), vertices, indices);

        let (width, height, rgba_data) = load_image("assets/image.png");

        let layout = Texture::default_layout(renderer.device());
        let texture = Texture::new(renderer.device(), &layout, width, height);
        texture.write_texture(renderer.queue(), &rgba_data);

        let shader = Shader::new(
            renderer.device(),
            SHADER_SOURCE,
            renderer.format(),
            &[Some(&layout)],
        );

        for entity in entities {
            let mut comp = ecs.get_component_mut::<QuadComponent>(*entity);
            comp.mesh = Some(mesh.clone());
            comp.shader = Some(shader.clone());
            comp.texture = Some(texture.clone());
        }
    }

    fn update(&mut self, entities: &mut [Entity], ecs: &mut ECS) {
        let mut renderer = ecs.get_resource_mut::<Renderer>();
        renderer.render(|render_pass| {
            for entity in entities.iter() {
                let comp = ecs.get_component::<QuadComponent>(*entity);
                if let (Some(shader), Some(texture), Some(mesh)) =
                    (&comp.shader, &comp.texture, &comp.mesh)
                {
                    shader.bind(render_pass);
                    texture.bind(render_pass, 0);
                    mesh.draw(render_pass);
                }
            }
        });
    }

    fn fixed_update(&mut self, _entities: &mut [Entity]) {}
}
