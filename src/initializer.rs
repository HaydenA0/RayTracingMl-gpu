pub async fn create_gpu_context() -> (wgpu::Device, wgpu::Queue, wgpu::Adapter, wgpu::Instance) {
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

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance, // Pick the 3050
            // to be added for a window
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .await
        .expect("Failed to find an appropriate adapter");
    println!("Using adapter: {:?}", adapter);

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
    return (device, queue, adapter, instance);
}

pub fn create_storage_texture(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    path: &str,
    width: u32,
    height: u32,
) -> (wgpu::Texture, wgpu::BindGroupLayout) {
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

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[wgpu::BindGroupLayoutEntry {
            // Binding 0
            //
            binding: 0,

            visibility: wgpu::ShaderStages::COMPUTE,

            count: None,

            ty: wgpu::BindingType::StorageTexture {
                format: wgpu::TextureFormat::Rgba32Float,

                access: wgpu::StorageTextureAccess::WriteOnly,

                view_dimension: wgpu::TextureViewDimension::D2,
            },
        }],
    });

    return (texture, bind_group_layout);
}

pub fn build_compute_pipeline(
    device: &wgpu::Device,
    bind_group_layout: &wgpu::BindGroupLayout,
    shader: &str,
) -> wgpu::ComputePipeline {
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Compute Pipeline Layout"),
        immediate_size: 0,
        bind_group_layouts: &[Some(bind_group_layout)],
    });

    let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Raytracer Shader"),
        source: wgpu::ShaderSource::Wgsl(shader.into()),
    });

    device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Raytracer Pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader_module,
        entry_point: Some("main"),
        compilation_options: wgpu::PipelineCompilationOptions::default(),
        cache: None,
    })
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
) -> (wgpu::Buffer, usize) {
    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Raytracer Bind Group"),
        layout: bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::TextureView(&texture_view),
        }],
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Compute Encoder"),
    });

    {
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Raytracer Pass"),
            timestamp_writes: None,
        });

        compute_pass.set_pipeline(pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.dispatch_workgroups(workgroups.0, workgroups.1, workgroups.2);
    }

    let _ = device.create_buffer(&wgpu::BufferDescriptor {
        // TODO : use this result
        label: Some("Raytracer Buffer"),
        size: 0,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let bytes_per_row = (width * 16).next_multiple_of(256);
    println!("bytes_per_row : {}", bytes_per_row);
    println!("Is it a multiple of 256 : {}", bytes_per_row % 256 == 0);

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

    queue.submit(std::iter::once(encoder.finish()));
    return (output_buffer, (bytes_per_row as usize));
}

pub fn from_buffer_to_vec(device: &wgpu::Device, buffer: &wgpu::Buffer) -> Vec<f32> {
    buffer.map_async(wgpu::MapMode::Read, .., |result| match result {
        Ok(_) => {
            println!("Success reading buffer");
        }
        Err(e) => {
            println!("Error reading: {:?}", e);
        }
    });

    let result = device.poll(wgpu::PollType::Wait {
        submission_index: None, // or Some(index) from a queue.submit() call
        timeout: None,          // or Some(Duration::from_secs(5))
    });

    match result {
        Ok(_) => {
            println!("Success Polling buffer");
        }
        Err(e) => {
            println!("Error Polling buffer {:?}", e);
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
