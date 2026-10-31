use crate::{StorageBuffer, Uniform};

pub struct BindGroupBuilder<'a> {
    layout_entries: Vec<wgpu::BindGroupLayoutEntry>,
    entries: Vec<wgpu::BindGroupEntry<'a>>,
}

impl<'a> BindGroupBuilder<'a> {
    pub fn new() -> Self {
        Self {
            layout_entries: Vec::new(),
            entries: Vec::new(),
        }
    }

    pub fn add_uniform<T: bytemuck::Pod>(
        mut self,
        uniform: &'a Uniform<T>,
        stages: wgpu::ShaderStages,
    ) -> Self {
        let binding = self.entries.len() as u32;
        self.layout_entries.push(wgpu::BindGroupLayoutEntry {
            binding,
            visibility: stages,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        });
        self.entries.push(wgpu::BindGroupEntry {
            binding,
            resource: uniform.binding(),
        });
        self
    }

    pub fn add_storage_buffer<T: bytemuck::Pod>(
        mut self,
        storage: &'a StorageBuffer<T>,
        stages: wgpu::ShaderStages,
    ) -> Self {
        let binding = self.entries.len() as u32;
        self.layout_entries.push(wgpu::BindGroupLayoutEntry {
            binding,
            visibility: stages,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage {
                    read_only: storage.is_read_only(),
                },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        });
        self.entries.push(wgpu::BindGroupEntry {
            binding,
            resource: storage.binding(),
        });
        self
    }

    pub fn build(self, device: &wgpu::Device) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &self.layout_entries,
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &self.entries,
        });
        (layout, bind_group)
    }
}
