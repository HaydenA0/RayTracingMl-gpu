use crate::buffers;
use crate::env;
use crate::fs;
use crate::initializer;
use crate::output;

pub struct Gpu {
    device: wgpu::Device,
    queue: wgpu::Queue,
    buffer: buffers::Buffers,
}

pub fn load_shader(shader_path: &str) -> String {
    let content_result = fs::read_to_string(shader_path);
    let content = {
        match content_result {
            Ok(content) => content,
            Err(error) => {
                match env::current_dir() {
                    Ok(path) => println!("Current directory: {}", path.display()),
                    Err(e) => eprintln!("Error getting current directory: {}", e),
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
    pub fn new() -> Gpu {
        let (device, queue, _, _) = pollster::block_on(initializer::create_gpu_context());
        Gpu { device, queue }
    }

    pub fn compute(&self, texture_path: &str, shader_path: &str, width: u32, height: u32) -> Pass {
        let (texture, bind_group_layout) = initializer::create_storage_texture(
            &self.device,
            &self.queue,
            texture_path,
            width,
            height,
        );
        let shader_source_code = load_shader(shader_path);
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
        );
        Pass {
            raw_buffer: initializer::from_buffer_to_vec(&self.device, &output_buffer),
            stride: stride,
        }
    }
}
