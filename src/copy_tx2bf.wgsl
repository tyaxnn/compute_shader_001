struct Status {
    width: u32,
    height: u32,
    frame_read : u32,
    spare_1 : u32,
    spare_2 : f32,
};


@group(0) @binding(0) var<uniform> status: Status;
@group(0) @binding(1) var outputTex: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(2) var input_texture : binding_array<texture_2d<f32>>;
@group(0) @binding(3) var<storage, read> intermediate_r: array<vec4<f32>>;
@group(0) @binding(4) var<storage, read_write> intermediate_w: array<vec4<f32>>;

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {

    let dimensions = textureDimensions(input_texture[status.frame_read]);

    var position = vec2<i32>(global_id.xy);

    var color : vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 1.0);

    let fragCoord: vec2<f32> = vec2<f32>(global_id.xy) / vec2<f32>(f32(status.width), f32(status.height)) - vec2<f32>(0.5, 0.5);

    color = textureLoad(input_texture[status.frame_read], vec2<i32>(position.x, position.y),0);
    
    intermediate_w[global_id.x + status.width * global_id.y] = color;
        
    textureStore(outputTex, vec2<i32>(global_id.xy), color);

}
