pub fn gen_image(mut pixels_vec: Vec<f32>, width: u32, height: u32) -> image::RgbaImage {
    pixels_vec.iter_mut().for_each(|x| *x = *&x.clamp(0.0, 1.0));
    println!("pixels_vec len : {:?}", pixels_vec.len());
    println!("image resolution : {:?}", width * height);
    let pixel_size = ((pixels_vec.len() as f64) / ((width * height) as f64)) as usize;
    println!("Pixel size : {:?}", pixel_size);

    let mut image = image::RgbaImage::new(width, height);
    for (i, pixel) in pixels_vec.chunks(pixel_size).enumerate() {
        let r = (pixel[0] * 255.0) as u8;
        let g = (pixel[1] * 255.0) as u8;
        let b = (pixel[2] * 255.0) as u8;
        let a = (pixel[3] * 255.0) as u8;
        let x = (i as u32) % width;
        let y = (i as u32) / width;
        image.put_pixel(x, y, image::Rgba([r, g, b, a]));
    }
    return image;
}

pub fn save_image(image: image::RgbaImage, path: &str) {
    image.save(path).unwrap();
}
