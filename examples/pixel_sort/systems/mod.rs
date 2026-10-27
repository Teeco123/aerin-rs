use aerin_ecs::{ECS, Entity, SystemTrait};
use aerin_math::{Vec2, Vec3};
use aerin_renderer::{Mesh, Renderer, Shader, Texture, Vertex};

use crate::components::QuadComponent;

fn load_image(path: &str) -> (u32, u32, Vec<u8>) {
    let img = image::open(path)
        .or_else(|_| image::open(format!("examples/pixel_sort/{path}")))
        .unwrap();
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    (w, h, rgba.into_raw())
}

fn luminance(rgba: &[u8]) -> f32 {
    0.299 * rgba[0] as f32 + 0.587 * rgba[1] as f32 + 0.114 * rgba[2] as f32
}

fn sort_pixels(pixels: &mut [u8], width: usize, height: usize, threshold: f32) {
    let row_stride = width * 4;

    for row in 0..height {
        let row_start = row * row_stride;
        let row_data = &mut pixels[row_start..row_start + row_stride];

        let mut start_x = 0;
        while start_x < width {
            while start_x < width && luminance(&row_data[start_x * 4..start_x * 4 + 4]) < threshold
            {
                start_x += 1;
            }

            let mut end_x = start_x;
            while end_x < width && luminance(&row_data[end_x * 4..end_x * 4 + 4]) >= threshold {
                end_x += 1;
            }

            if end_x > start_x + 1 {
                let slice: &mut [[u8; 4]] =
                    bytemuck::cast_slice_mut(&mut row_data[start_x * 4..end_x * 4]);
                slice.sort_by(|a, b| {
                    luminance(a)
                        .partial_cmp(&luminance(b))
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }

            start_x = end_x;
        }
    }
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
pub struct PixelSortSystem;

impl SystemTrait for PixelSortSystem {
    fn type_of() -> &'static str {
        "pixel_sort"
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

        let (width, height, mut rgba_data) = load_image("assets/image.png");
        sort_pixels(&mut rgba_data, width as usize, height as usize, 10.0);

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
        renderer.render(
            |scene_pass| {
                for entity in entities.iter() {
                    let comp = ecs.get_component::<QuadComponent>(*entity);
                    if let (Some(shader), Some(texture), Some(mesh)) =
                        (&comp.shader, &comp.texture, &comp.mesh)
                    {
                        shader.bind(scene_pass);
                        texture.bind(scene_pass, 0);
                        mesh.draw(scene_pass);
                    }
                }
            },
            |ui_pass| {},
        );
    }

    fn fixed_update(&mut self, _entities: &mut [Entity]) {}
}
