mod buffers;
mod camera;
mod gpu;
mod initializer;
mod output;
mod utils;
use log::debug;
use std::{env, fs};

fn main() {
    env_logger::init();

    let width = 800;
    let height = 400;

    debug!("Image size: {}x{}", width, height);

    // to create a CPU struct to hold the objects
    // but for now we'll just define it here

    let camera = camera::Camera::new(width as f32 / height as f32, 60.0);

    let gpu = gpu::Gpu::new(&camera);

    let pass_from_gpu = gpu.compute("images/test.png", "src/shaders/main.wgsl", width, height);

    debug!("Generating image...");
    _ = pass_from_gpu.to_image(width, height).save("test.png");
    debug!("Image saved to test.png");

    debug!("Done");
}
