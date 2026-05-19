@group(0) @binding(0) 

var output_texture: texture_storage_2d<rgba32float, write>;


@compute @workgroup_size(16, 16, 1)




fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    // 'id.xy' is the unique (x, y) coordinate for this specific worker

    let uv = vec2<f32>(f32(id.x) / f32(textureDimensions(output_texture).x), f32(id.y) / f32(textureDimensions(output_texture).y));

    let x = cos(50.0 * (uv.x - 0.5))/((uv.x - 0.5) + 0.1);
    let y = sin(50.0 * (uv.y - 0.5))/((uv.y - 0.5) + 0.1);
    let background_color = vec4<f32>(x, y, uv.x, 1.0);
    textureStore(output_texture, id.xy, background_color);

}
