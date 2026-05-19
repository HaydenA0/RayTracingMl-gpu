mod initializer;
mod output;
use std::{env, fs};

// small function to load a shader source code

fn load_shader(shader_path: &str) -> String {
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

fn main() {
    let width = 801;
    let height = 400;

    let (device, queue, _, _) = pollster::block_on(initializer::create_gpu_context());

    let (texture, bind_group_layout) = initializer::create_storage_texture(
        &device,
        &queue,
        "src/textures/test.png",
        width,
        height,
    );

    let shader_source_code = load_shader("src/main.wgsl");

    let compute_pipeline =
        initializer::build_compute_pipeline(&device, &bind_group_layout, &shader_source_code);

    let workgroup_size = 16u32;
    let (output_buffer, stride) = initializer::dispatch_compute_pass(
        &device,
        &queue,
        &compute_pipeline,
        &bind_group_layout,
        &texture,
        (
            (width + workgroup_size - 1) / workgroup_size,  // 50
            (height + workgroup_size - 1) / workgroup_size, // 25
            1,
        ),
        width,
        height,
    );

    let pixels_vec = initializer::from_buffer_to_vec(&device, &output_buffer);

    let image = output::gen_image(pixels_vec, width, height, stride);
    output::save_image(image, "test.png");

    println!("Done");
}
