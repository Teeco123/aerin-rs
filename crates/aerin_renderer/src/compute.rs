#[derive(Clone)]
pub struct Compute {
    pipeline: wgpu::ComputePipeline,
}

impl Compute {
    pub fn new(
        device: &wgpu::Device,
        wgsl_source: &str,
        entry_point: Option<&str>,
        bind_group_layouts: &[Option<&wgpu::BindGroupLayout>],
    ) -> Self {
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline layout"),
            bind_group_layouts: bind_group_layouts,
            immediate_size: 0,
        });

        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: wgpu::ShaderSource::Wgsl(wgsl_source.into()),
        });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute pipeline"),
            layout: Some(&layout),
            module: &module,
            entry_point: entry_point,
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            cache: None,
        });

        Self { pipeline }
    }

    pub fn bind(&self, pass: &mut wgpu::ComputePass) {
        pass.set_pipeline(&self.pipeline);
    }

    pub fn dispatch(&self, pass: &mut wgpu::ComputePass, x: u32, y: u32, z: u32) {
        pass.set_pipeline(&self.pipeline);
        pass.dispatch_workgroups(x, y, z);
    }
}
