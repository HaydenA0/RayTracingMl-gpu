@group(0) @binding(0)
var output_texture: texture_storage_2d<rgba32float, write>;

struct CameraUniform {
    origin: vec3<f32>,
    padding1: f32,
    basis_u: vec3<f32>,
    padding2: f32,
    basis_v: vec3<f32>,
    padding3: f32,
    basis_w: vec3<f32>,
    aspect_ratio: f32,
};

struct Sphere {
    center: vec3<f32>,
    radius: f32,
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

fn check_intersection(ray: Ray, sphere: Sphere) -> bool {
    let oc = ray.origin - sphere.center;
    let a = dot(ray.direction, ray.direction);
    let b = dot(oc, ray.direction);
    let c = dot(oc, oc) - sphere.radius * sphere.radius;
    let discriminant = b * b - a * c;

    if discriminant > 0.0 {
        let t = (-b - sqrt(discriminant)) / a;
        if t > 0.0 {
            return true;
        }
    }
    return false;

}

@group(0) @binding(1)
var<uniform> camera: CameraUniform;

@group(0) @binding(2)
var<storage, read> spheres: array<Sphere>;

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

    let ray_origin = camera.origin;
    let ray = new_ray(ray_origin, ray_direction);

    let spheres_count = arrayLength(&spheres);

    var color = vec4<f32>(0.0, 0.0, 0.0, 1.0);

    for (var i = 0u; i < spheres_count; i = i + 1u) {
        let sphere = spheres[i];

        if (check_intersection(ray, sphere)) {
            color = vec4<f32>(0.5, 0.1, 0.1, 1.0);
        } else {
            color = vec4<f32>(0.1, 0.1, 0.1, 1.0);
        }
    }

    textureStore(output_texture, vec2<i32>(id.xy), color);
}
