use std::{env, fs};

use log::debug;

use crate::camera::CameraBuffer;
use crate::initializer;
use crate::output;
use crate::spheres::SphereWorld;

pub fn load_shader(path: &str) -> String {
    debug!("Loading shader from {}...", path);
    match fs::read_to_string(path) {
        Ok(content) => {
            debug!("Shader loaded");
            content
        }
        Err(err) => {
            match env::current_dir() {
                Ok(path) => debug!("Current directory: {}", path.display()),
                Err(e) => debug!("Error getting current directory: {}", e),
            }
            panic!("Error reading shader file: {}", err);
        }
    }
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

pub struct Gpu {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    bind_group_layout: wgpu::BindGroupLayout,
    pipeline_layout: wgpu::PipelineLayout,
    compute_pipeline: wgpu::ComputePipeline,
}

impl Gpu {
    pub async fn new(shader_source: &str) -> Self {
        debug!("Initializing GPU...");
        let (device, queue) = initializer::create_gpu_context().await;

        let bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Raytracer Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        count: None,
                        ty: wgpu::BindingType::StorageTexture {
                            format: wgpu::TextureFormat::Rgba32Float,
                            access: wgpu::StorageTextureAccess::WriteOnly,
                            view_dimension: wgpu::TextureViewDimension::D2,
                        },
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        count: None,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        count: None,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                    },
                ],
            });

        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Raytracer Pipeline Layout"),
                immediate_size: 0,
                bind_group_layouts: &[Some(&bind_group_layout)],
            });

        let shader_module =
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Raytracer Shader"),
                source: wgpu::ShaderSource::Wgsl(shader_source.into()),
            });

        let compute_pipeline =
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("Raytracer Pipeline"),
                layout: Some(&pipeline_layout),
                module: &shader_module,
                entry_point: Some("main"),
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                cache: None,
            });

        debug!("GPU initialized with forever pipeline resources");
        Self {
            device,
            queue,
            bind_group_layout,
            pipeline_layout,
            compute_pipeline,
        }
    }

    fn create_bind_group(
        &self,
        texture_view: &wgpu::TextureView,
        camera: &CameraBuffer,
        spheres: &SphereWorld,
    ) -> wgpu::BindGroup {
        self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Raytracer Bind Group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: camera.buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: spheres.buffer.as_entire_binding(),
                },
            ],
        })
    }

    pub fn compute(
        &self,
        width: u32,
        height: u32,
        camera: &CameraBuffer,
        spheres: &SphereWorld,
    ) -> Pass {
        debug!("Starting compute pass ({}x{})...", width, height);

        camera.write_to_gpu(&self.queue);

        let texture = initializer::create_storage_texture(&self.device, width, height);
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let bind_group = self.create_bind_group(&texture_view, camera, spheres);

        let workgroup_size = 16u32;
        let workgroups = (
            (width + workgroup_size - 1) / workgroup_size,
            (height + workgroup_size - 1) / workgroup_size,
            1,
        );

        let mut encoder =
            self.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Compute Encoder"),
                });

        {
            let mut compute_pass =
                encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                    label: Some("Raytracer Pass"),
                    timestamp_writes: None,
                });
            compute_pass.set_pipeline(&self.compute_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch_workgroups(workgroups.0, workgroups.1, workgroups.2);
        }

        let bytes_per_row = (width * 16).next_multiple_of(256);
        debug!("bytes_per_row: {}", bytes_per_row);

        let output_buffer = self
            .device
            .create_buffer(&wgpu::BufferDescriptor {
                label: Some("Output Buffer"),
                size: (bytes_per_row * height) as u64,
                usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfoBase {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &output_buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(bytes_per_row),
                    rows_per_image: Some(height),
                },
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );

        self.queue.submit(std::iter::once(encoder.finish()));
        debug!("Compute pass submitted");

        Pass {
            raw_buffer: initializer::readback_buffer(&self.device, &output_buffer),
            stride: bytes_per_row as usize,
        }
    }
}
