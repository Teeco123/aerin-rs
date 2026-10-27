use aerin_math::{Vec2, Vec3};
use wgpu::util::DeviceExt;

use crate::Vertex;

#[derive(Clone)]
pub struct Mesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    index_count: u32,
}

impl Mesh {
    pub fn new(device: &wgpu::Device, vertices: &[Vertex], indices: &[u32]) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            vertex_buffer,
            index_buffer,
            index_count: indices.len() as u32,
        }
    }

    pub fn draw(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}

impl Mesh {
    pub fn quad(device: &wgpu::Device) -> Self {
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
        Mesh::new(device, vertices, indices)
    }
}
