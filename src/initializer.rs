use log::debug;

pub async fn create_gpu_context() -> (wgpu::Device, wgpu::Queue) {
    debug!("Creating GPU instance...");
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        flags: wgpu::InstanceFlags::default(),
        backend_options: wgpu::BackendOptions::default(),
        memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
        display: None,
    });

    debug!("Requesting adapter...");
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
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
    (device, queue)
}

pub fn create_storage_texture(device: &wgpu::Device, width: u32, height: u32) -> wgpu::Texture {
    debug!("Creating storage texture ({}x{})...", width, height);
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba32Float,
        label: Some("Storage Texture"),
        mip_level_count: 1,
        sample_count: 1,
        usage: wgpu::TextureUsages::STORAGE_BINDING
            | wgpu::TextureUsages::COPY_DST
            | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    debug!("Storage texture created");
    texture
}

pub fn readback_buffer(device: &wgpu::Device, buffer: &wgpu::Buffer) -> Vec<f32> {
    debug!("Mapping buffer for reading...");
    buffer
        .map_async(wgpu::MapMode::Read, .., |result| match result {
            Ok(_) => debug!("Success reading buffer"),
            Err(e) => debug!("Error reading: {:?}", e),
        });

    device
        .poll(wgpu::PollType::Wait {
            submission_index: None,
            timeout: None,
        })
        .expect("Failed to poll device");

    let floats: Vec<f32> = {
        let buffer_view = buffer.get_mapped_range(..);
        bytemuck::cast_slice(&buffer_view).to_vec()
    };
    buffer.unmap();
    floats
}
