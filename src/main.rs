mod camera;
mod gpu;
mod initializer;
mod output;
mod spheres;
mod utils;

use log::debug;

fn main() {
    env_logger::init();

    let width = 800;
    let height = 400;

    debug!("Image size: {}x{}", width, height);

    let camera = camera::Camera::new(width as f32 / height as f32, 60.0);
    let spheres_data = vec![
        spheres::Sphere::new([0.0, 0.0, -1.0], 0.5),
        spheres::Sphere::new([1.0, 0.0, -1.0], 0.3),
    ];

    let shader_source = gpu::load_shader("src/shaders/main.wgsl");
    let gpu = pollster::block_on(gpu::Gpu::new(&shader_source));

    let camera_buffer = camera::CameraBuffer::new(&camera, &gpu.device);
    let mut sphere_world = spheres::SphereWorld::new(spheres_data, &gpu.device);
    sphere_world.update_buffers(&gpu.device, &gpu.queue);

    let pass = gpu.compute(width, height, &camera_buffer, &sphere_world);

    debug!("Generating image...");
    pass.to_image(width, height).save("test.png").unwrap();
    debug!("Image saved to test.png");

    debug!("Done");
}
