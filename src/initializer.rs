use crate::buffers;
use crate::buffers::CameraBuffer;
use log::debug;

use crate::camera::{Camera, CameraUniform};

pub async fn create_gpu_context() -> (wgpu::Device, wgpu::Queue, wgpu::Adapter, wgpu::Instance) {
    debug!("Creating GPU instance...");
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        // Be using Vulkan since on Linux
        backends: wgpu::Backends::VULKAN,

        // default values for debugging
        flags: wgpu::InstanceFlags::default(),
        backend_options: wgpu::BackendOptions::default(),
        // also default values for memory budget thresholds
        memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
        display: None,
    });

    debug!("Requesting adapter...");
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance, // Pick the 3050
            // to be added for a window
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .await
        .expect("Failed to find an appropriate adapter");
    debug!("Using adapter: {:?}", adapter);

    debug!("Requesting device and queue...");
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            memory_hints: wgpu::MemoryHints::default(),
            experimental_features: wgpu::ExperimentalFeatures::default(),
            trace: wgpu::Trace::default(),
        })
        .await
        .expect("Failed to create device");
    debug!("GPU context created successfully");
    return (device, queue, adapter, instance);
}

pub fn create_storage_texture(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    // path: &str,
    width: u32,
    height: u32,
    camera_uniform: &CameraUniform,
    camera_buffer: &buffers::CameraBuffer,
) -> (wgpu::Texture, wgpu::BindGroupLayout) {
    debug!("Creating storage texture ({}x{})...", width, height);
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        size: wgpu::Extent3d {
            width: width,
            height: height,
            depth_or_array_layers: 1,
        },

        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba32Float,
        label: Some("texture"),
        mip_level_count: 1,
        sample_count: 1,
        usage: wgpu::TextureUsages::STORAGE_BINDING // for shaders  to write to it
            | wgpu::TextureUsages::COPY_DST // from the CPU
            | wgpu::TextureUsages::COPY_SRC, // for moving it out of the file
        view_formats: &[],
    });
    debug!("Storage texture created");

    debug!("Creating bind group layout...");
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            wgpu::BindGroupLayoutEntry {
                // Binding 0
                // texture
                binding: 0,

                visibility: wgpu::ShaderStages::COMPUTE,

                count: None,

                ty: wgpu::BindingType::StorageTexture {
                    format: wgpu::TextureFormat::Rgba32Float,

                    access: wgpu::StorageTextureAccess::WriteOnly,

                    view_dimension: wgpu::TextureViewDimension::D2,
                },
            },
            // Binding 1
            // camera
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
        ],
    });

    debug!("Writing camera uniform to buffer...");
    queue.write_buffer(
        &camera_buffer.buffer_proper,
        0,
        bytemuck::bytes_of(camera_uniform),
    );

    debug!("Storage texture and bind group layout created");
    return (texture, bind_group_layout);
}

pub fn build_compute_pipeline(
    device: &wgpu::Device,
    bind_group_layout: &wgpu::BindGroupLayout,
    shader: &str,
) -> wgpu::ComputePipeline {
    debug!("Building compute pipeline...");

    debug!("Creating camera uniform buffer...");

    debug!("Creating pipeline layout...");
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Compute Pipeline Layout"),
        immediate_size: 0,
        bind_group_layouts: &[Some(bind_group_layout)],
    });

    debug!("Compiling shader module...");
    let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Raytracer Shader"),
        source: wgpu::ShaderSource::Wgsl(shader.into()),
    });

    debug!("Creating compute pipeline...");
    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Raytracer Pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader_module,
        entry_point: Some("main"),
        compilation_options: wgpu::PipelineCompilationOptions::default(),
        cache: None,
    });
    debug!("Compute pipeline built");
    compute_pipeline
}

pub fn dispatch_compute_pass(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    pipeline: &wgpu::ComputePipeline,
    bind_group_layout: &wgpu::BindGroupLayout,
    texture: &wgpu::Texture,
    workgroups: (u32, u32, u32),
    width: u32,
    height: u32,
    camera_buffer: &CameraBuffer,
) -> (wgpu::Buffer, usize) {
    debug!(
        "Dispatching compute pass with workgroups ({}, {}, {})...",
        workgroups.0, workgroups.1, workgroups.2
    );

    debug!("Creating texture view...");
    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    debug!("Creating bind group...");
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Raytracer Bind Group"),
        layout: bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                // binding: 0,
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&texture_view),
            },
            // Binding 1
            wgpu::BindGroupEntry {
                binding: 1,
                resource: camera_buffer.buffer_proper.as_entire_binding(),
            },
        ],
    });

    debug!("Creating command encoder...");
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Compute Encoder"),
    });

    debug!("Beginning compute pass...");
    {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Raytracer Pass"),
            timestamp_writes: None,
        });

        compute_pass.set_pipeline(pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.dispatch_workgroups(workgroups.0, workgroups.1, workgroups.2);
    }
    debug!("Compute pass dispatched");

    let _ = device.create_buffer(&wgpu::BufferDescriptor {
        // TODO : use this result
        label: Some("Raytracer Buffer"),
        size: 0,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let bytes_per_row = (width * 16).next_multiple_of(256);
    debug!("bytes_per_row : {}", bytes_per_row);
    debug!("Is it a multiple of 256 : {}", bytes_per_row % 256 == 0);

    debug!("Creating output buffer...");
    let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Raytracer Buffer"),
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

    debug!("Copying texture to buffer...");
    queue.submit(std::iter::once(encoder.finish()));
    debug!("Queue submission done");
    return (output_buffer, (bytes_per_row as usize));
}

pub fn from_buffer_to_vec(device: &wgpu::Device, buffer: &wgpu::Buffer) -> Vec<f32> {
    debug!("Mapping buffer for reading...");
    buffer.map_async(wgpu::MapMode::Read, .., |result| match result {
        Ok(_) => {
            debug!("Success reading buffer");
        }
        Err(e) => {
            debug!("Error reading: {:?}", e);
        }
    });

    let result = device.poll(wgpu::PollType::Wait {
        submission_index: None, // or Some(index) from a queue.submit() call
        timeout: None,          // or Some(Duration::from_secs(5))
    });

    match result {
        Ok(_) => {
            debug!("Success Polling buffer");
        }
        Err(e) => {
            debug!("Error Polling buffer {:?}", e);
        }
    }

    let floats: Vec<f32> = {
        let buffer_view = buffer.get_mapped_range(..);
        let slice: &[f32] = bytemuck::cast_slice(&buffer_view);
        slice.to_vec() // copy out before buffer_view drops
    };
    buffer.unmap();
    floats
}
