use wgpu::util::DeviceExt;

pub struct StorageBuffer<T: bytemuck::Pod> {
    buffer: wgpu::Buffer,
    len: usize,
    read_only: bool,
    _marker: std::marker::PhantomData<T>,
}

impl<T: bytemuck::Pod> StorageBuffer<T> {
    pub fn new(device: &wgpu::Device, initial_data: &[T], read_only: bool) -> Self {
        let mut usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST;
        if !read_only {
            usage |= wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::VERTEX;
        }

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Storage Buffer"),
            contents: bytemuck::cast_slice(initial_data),
            usage,
        });

        Self {
            buffer,
            len: initial_data.len(),
            read_only,
            _marker: std::marker::PhantomData,
        }
    }
}
