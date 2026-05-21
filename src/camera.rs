use glam::Vec3;

pub struct Camera {
    pub origin: Vec3,
    pub basis_u: Vec3,
    pub basis_v: Vec3,
    pub basis_w: Vec3,
    pub aspect_ratio: f32,
    pub vertical_field_of_view: f32,
}

impl Camera {
    pub fn new(aspect_ratio: f32, vertical_field_of_view: f32) -> Self {
        Self {
            origin: Vec3::new(0.0, 0.0, 0.0),
            basis_u: Vec3::new(-1.0, 0.0, 0.0),
            basis_v: Vec3::new(0.0, -1.0, 0.0),
            basis_w: Vec3::new(0.0, 0.0, -1.0),
            aspect_ratio,
            vertical_field_of_view,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub origin: [f32; 3],
    pub padding1: f32,
    pub basis_u: [f32; 3],
    pub padding2: f32,
    pub basis_v: [f32; 3],
    pub padding3: f32,
    pub basis_w: [f32; 3],
    pub aspect_ratio: f32,
}

impl CameraUniform {
    pub fn update_from_camera(&mut self, camera: &Camera) {
        self.origin = camera.origin.into();
        self.basis_u = camera.basis_u.into();
        self.basis_v = camera.basis_v.into();
        self.basis_w = camera.basis_w.into();
        self.aspect_ratio = camera.aspect_ratio;
    }

    pub fn new(camera: &Camera) -> Self {
        let mut uniform = Self::default();
        uniform.update_from_camera(camera);
        uniform
    }
}

impl Default for CameraUniform {
    fn default() -> Self {
        Self {
            origin: [0.0; 3],
            padding1: 0.0,
            basis_u: [0.0; 3],
            padding2: 0.0,
            basis_v: [0.0; 3],
            padding3: 0.0,
            basis_w: [0.0; 3],
            aspect_ratio: 0.0,
        }
    }
}

pub struct CameraBuffer {
    pub uniform: CameraUniform,
    pub buffer: wgpu::Buffer,
}

impl CameraBuffer {
    pub fn new(camera: &Camera, device: &wgpu::Device) -> Self {
        let uniform = CameraUniform::new(camera);
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera"),
            size: std::mem::size_of::<CameraUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        Self { uniform, buffer }
    }

    pub fn write_to_gpu(&self, queue: &wgpu::Queue) {
        queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&self.uniform));
    }
}
