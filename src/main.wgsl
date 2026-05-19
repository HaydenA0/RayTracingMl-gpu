@group(0) @binding(0) 

var output_texture: texture_storage_2d<rgba32float, write>;


@compute @workgroup_size(16, 16, 1)




fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    // 'id.xy' is the unique (x, y) coordinate for this specific worker

    let uv = vec2<f32>(f32(id.x) / f32(textureDimensions(output_texture).x), f32(id.y) / f32(textureDimensions(output_texture).y));
    let background_color = vec4<f32>(uv.x, uv.y, 0.0, 1.0);
    textureStore(output_texture, id.xy, background_color);

}
