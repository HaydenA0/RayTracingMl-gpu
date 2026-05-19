use crate::camera::Camera;

pub struct GpuBuffer {
    pub data: Camera,
    pub buffer_proper: wgpu::Buffer,
}

pub struct Buffers {
    pub camera: GpuBuffer,
    // to add buffers later
}
