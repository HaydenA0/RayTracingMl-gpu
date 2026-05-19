pub fn gen_image(
    pixels_vec: &Vec<f32>,
    width: u32,
    height: u32,
    stride: usize,
) -> image::RgbaImage {
    let mut image = image::RgbaImage::new(width, height);

    let floats_per_row = stride / 4;

    for y in 0..height {
        let row_start = y as usize * floats_per_row;
        for x in 0..width {
            let pixel_start = row_start + x as usize * 4;
            let r = (pixels_vec[pixel_start].clamp(0.0, 1.0) * 255.0) as u8;
            let g = (pixels_vec[pixel_start + 1].clamp(0.0, 1.0) * 255.0) as u8;
            let b = (pixels_vec[pixel_start + 2].clamp(0.0, 1.0) * 255.0) as u8;
            let a = (pixels_vec[pixel_start + 3].clamp(0.0, 1.0) * 255.0) as u8;
            image.put_pixel(x, y, image::Rgba([r, g, b, a]));
        }
    }

    image
}
