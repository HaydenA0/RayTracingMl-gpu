#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Sphere {
    pub center: [f32; 3],
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: [f32; 3], radius: f32) -> Self {
        Self { center, radius }
    }
}

pub struct SphereWorld {
    pub spheres: Vec<Sphere>,
    pub buffer: wgpu::Buffer,
    pub capacity: usize,
    pub needs_bind_group_update: bool,
}

impl SphereWorld {
    pub fn new(spheres: Vec<Sphere>, device: &wgpu::Device) -> Self {
        let capacity = spheres.len().max(1);
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Spheres"),
            size: (capacity * std::mem::size_of::<Sphere>()) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        Self {
            spheres,
            buffer,
            capacity,
            needs_bind_group_update: true,
        }
    }

    pub fn update_buffers(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        let count = self.spheres.len();
        if count > self.capacity {
            self.capacity = count.next_power_of_two();
            self.buffer = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Spheres"),
                size: (self.capacity * std::mem::size_of::<Sphere>()) as u64,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            self.needs_bind_group_update = true;
        }
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&self.spheres));
    }
}
