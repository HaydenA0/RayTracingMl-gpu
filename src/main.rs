mod buffers;
mod camera;
mod gpu;
mod initializer;
mod output;
mod utils;
use std::{env, fs};

fn main() {
    let width = 800;
    let height = 400;

    let gpu = gpu::Gpu::new();

    let pass_from_gpu = gpu.compute("images/test.png", "src/shaders/main.wgsl", width, height);

    _ = pass_from_gpu.to_image(width, height).save("test.png");

    println!("Done");
}
