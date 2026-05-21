use crate::buffers::Buffers;
use crate::camera;
use image::buffer;
use log::debug;

use crate::buffers;
use crate::env;
use crate::fs;
use crate::initializer;
use crate::output;

pub struct Gpu {
    device: wgpu::Device,
    queue: wgpu::Queue,
    buffers: buffers::Buffers,
}

pub fn load_shader(shader_path: &str) -> String {
    debug!("Loading shader from {}...", shader_path);
    let content_result = fs::read_to_string(shader_path);
    let content = {
        match content_result {
            Ok(content) => {
                debug!("Shader loaded");
                content
            }
            Err(error) => {
                match env::current_dir() {
                    Ok(path) => debug!("Current directory: {}", path.display()),
                    Err(e) => debug!("Error getting current directory: {}", e),
                };
                panic!("Error reading file: {}", error);
            }
        }
    };
    return content;
}

pub struct Pass {
    raw_buffer: Vec<f32>,
    stride: usize,
}

impl Pass {
    pub fn to_image(&self, width: u32, height: u32) -> image::RgbaImage {
        output::gen_image(&self.raw_buffer, width, height, self.stride)
    }
}

impl Gpu {
    pub fn new(camera: &camera::Camera) -> Gpu {
        debug!("Initializing GPU...");
        let (device, queue, _, _) = pollster::block_on(initializer::create_gpu_context());
        let camera_uniform = camera::CameraUniform::new(camera);
        let buffers = buffers::Buffers::new(&camera_uniform, &device);
        debug!("GPU initialized");

        Gpu {
            device,
            queue,
            buffers: buffers,
        }
    }

    pub fn compute(&self, texture_path: &str, shader_path: &str, width: u32, height: u32) -> Pass {
        debug!("Starting compute pipeline...");
        debug!("Creating storage texture...");
        let (texture, bind_group_layout) = initializer::create_storage_texture(
            &self.device,
            &self.queue,
            // texture_path,
            width,
            height,
            &self.buffers.camera_buffer.data,
            &self.buffers.camera_buffer,
        );
        let shader_source_code = load_shader(shader_path);
        debug!("Building compute pipeline...");
        let compute_pipeline = initializer::build_compute_pipeline(
            &self.device,
            &bind_group_layout,
            &shader_source_code,
        );
        let workgroup_size = 16u32;
        let (output_buffer, stride) = initializer::dispatch_compute_pass(
            &self.device,
            &self.queue,
            &compute_pipeline,
            &bind_group_layout,
            &texture,
            (
                (width + workgroup_size - 1) / workgroup_size,
                (height + workgroup_size - 1) / workgroup_size,
                1,
            ),
            width,
            height,
            &self.buffers.camera_buffer,
        );
        debug!("Reading buffer back to CPU...");
        Pass {
            raw_buffer: initializer::from_buffer_to_vec(&self.device, &output_buffer),
            stride: stride,
        }
    }
}
