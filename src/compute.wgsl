struct Params {
    width: u32,
    height: u32,
    iTime: f32,
    frame_num : u32,
};



@group(0) @binding(0) var<uniform> params: Params;
@group(0) @binding(1) var outputTex: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(2) var input_texture : binding_array<texture_2d<f32>>;

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_ix: vec3<u32>) {

    let time_n = u32(params.iTime * 10) % params.frame_num;
    let dimensions = textureDimensions(input_texture[time_n]);

    var position = vec2<i32>(global_ix.xy);

    var color : vec4<f32> = vec4<f32>(0.0, 0.0, sin(params.iTime) * 0.0, 1.);


    let fragCoord: vec2<f32> = vec2<f32>(global_ix.xy) / vec2<f32>(f32(params.width), f32(params.height))
        - vec2<f32>(0.5, 0.5);


        
    var sum = params.iTime * 0.2 + fragCoord.x;

    if (floor(sum) + 0.01 > sum) {
            color = color + textureLoad(input_texture[time_n], vec2<i32>(position.y, position.y),0);
    }
    else {
        color = color + textureLoad(input_texture[time_n], vec2<i32>(position.x, position.y),0);
    }

    // Shadertoy-like code can go here.
    let fragColor: vec4<f32> = color;

    textureStore(outputTex, vec2<i32>(global_ix.xy), fragColor);
}
