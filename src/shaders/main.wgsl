@group(0) @binding(0) 

var output_texture: texture_storage_2d<rgba32float, write>;

struct CameraUniform {
    origin: vec3<f32>,
    padding1: f32, // Matches padding1
    basis_u: vec3<f32>,
    padding2: f32, // Matches padding2
    basis_v: vec3<f32>,
    padding3: f32, // Matches padding3
    basis_w: vec3<f32>,
    aspect_ratio: f32,
};

struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>,
};

fn ray_at(r: Ray, t: f32) -> vec3<f32> {
    return r.origin + r.direction * t;
}

fn get_direction(r: Ray) -> vec3<f32> {
    return r.direction;
}

fn get_origin(r: Ray) -> vec3<f32> {
    return r.origin;
}

fn new_ray(origin: vec3<f32>, direction: vec3<f32>) -> Ray {
    return Ray(origin, direction);
}


@group(0) @binding(1)
var<uniform> camera: CameraUniform;


@compute @workgroup_size(16, 16, 1)




fn main(@builtin(global_invocation_id) id: vec3<u32>) {

    let dims = vec2<f32>(textureDimensions(output_texture));
    let uv = vec2<f32>(id.xy) / dims;

    let centered_u = uv.x - 0.5;
    let centered_v = uv.y - 0.5;

    let ray_direction = normalize(
        (centered_u * camera.aspect_ratio * camera.basis_u) +
        (centered_v * camera.basis_v) +
        camera.basis_w
    );



    let color = vec4<f32>(
        ray_direction * 0.5 + vec3<f32>(0.5),
        1.0
    );

    textureStore(output_texture, vec2<i32>(id.xy), color);
}
