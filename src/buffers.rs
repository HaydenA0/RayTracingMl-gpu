use wgpu::util::DeviceExt;

use crate::camera::CameraUniform;

pub struct CameraBuffer {
    pub data: CameraUniform,
    pub buffer_proper: wgpu::Buffer,
}

pub struct Buffers {
    pub camera_buffer: CameraBuffer,
    pub spheres_buffer: SpheresBuffer,
}

impl CameraBuffer {
    pub fn new(camera: CameraUniform, device: &wgpu::Device) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::bytes_of(&camera),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            data: camera,
            buffer_proper: buffer,
        }
    }
}

impl Buffers {
    pub fn new(camera: &CameraUniform, device: &wgpu::Device) -> Self {
        let camera = CameraBuffer::new(camera.clone(), device);

        Self {
            camera_buffer: camera,
        }
    }
}
