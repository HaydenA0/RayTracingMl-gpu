// from an old project (For CPU)
//
// to ignore most of these functions

use crate::vector3::Vector3;
use crate::vector3::dot;
use indicatif::ProgressBar;
// well something WILL need a progress bar
use indicatif::ProgressStyle;
use std::f32::consts::PI;

// use rand::Rng;
use std::cell::RefCell;

pub const INFINITY: f32 = f32::INFINITY;
pub const EPSILON: f32 = 1e-6;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn radians_to_degrees(radians: f32) -> f32 {
    radians * 180.0 / PI
}

// thread_local! {
//     static RNG: RefCell<rand::rngs::ThreadRng> = RefCell::new(rand::thread_rng());
// }

// pub fn generate_random_float_unit() -> f32 {
//     RNG.with(|rng| rng.borrow_mut().r#gen::<f32>())
// }
//
// pub fn generate_random_float_in_range(min: f32, max: f32) -> f32 {
//     RNG.with(|rng| rng.borrow_mut().gen_range(min..max))
// }

// pub fn sample_from_unit_square() -> Vector3 {
//     return Vector3::new(
//         generate_random_float_unit() - 0.5,
//         generate_random_float_unit() - 0.5,
//         0.0,
//     );
// }

pub fn reflect(v: Vector3, normal: Vector3) -> Vector3 {
    return v - normal * 2.0 * dot(v, normal);
}

pub fn setup_progress_bar(height: u32) -> indicatif::ProgressBar {
    let progress_bar = ProgressBar::new(height as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}][{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .expect("Invalid template")
            .progress_chars("#>-"),
    );
    progress_bar
}
