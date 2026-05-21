// from an old project (For CPU)
//
// to ignore most of these functions

use indicatif::ProgressBar;
// well something WILL need a progress bar
use indicatif::ProgressStyle;
use std::f32::consts::PI;

pub const INFINITY: f32 = f32::INFINITY;
pub const EPSILON: f32 = 1e-6;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn radians_to_degrees(radians: f32) -> f32 {
    radians * 180.0 / PI
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
