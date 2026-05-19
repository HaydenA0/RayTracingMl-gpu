use glam::Vec3;
use glam::Vec4;

pub struct Camera {
    pub origin: Vec3,
    pub basis_u: Vec3,
    pub basis_v: Vec3,
    pub basis_w: Vec3,
    pub aspect_ratio: f32,
    pub vertical_field_of_view: f32,
}

impl Camera {
    pub fn new(aspect_ratio: f32, vertical_field_of_view: f32) -> Self {
        Self {
            origin: Vec3::new(0.0, 0.0, 0.0).normalize(),
            basis_u: Vec3::new(-1.0, 0.0, 0.0).normalize(),
            basis_v: Vec3::new(0.0, -1.0, 0.0).normalize(),
            basis_w: Vec3::new(0.0, 0.0, -1.0).normalize(),
            aspect_ratio,
            vertical_field_of_view,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub origin: [f32; 3],
    pub padding1: f32,
    pub basis_u: [f32; 3],
    pub padding2: f32,
    pub basis_v: [f32; 3],
    pub padding3: f32,
    pub basis_w: [f32; 3],
    pub aspect_ratio: f32,
}
