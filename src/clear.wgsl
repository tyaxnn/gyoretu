struct Status {
    width: u32,
    height: u32,
    frame_read : u32,
    spare_1 : u32,
    spare_2 : f32,
};

struct Parameter {
    r : f32,
    g : f32,
    b : f32,
    a : f32,
}


@group(0) @binding(0) var<uniform> status: Status;
@group(0) @binding(1) var outputTex: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(3) var<storage, read> intermediate_r: array<vec4<f32>>;
@group(0) @binding(4) var<storage, read_write> intermediate_w: array<vec4<f32>>;
@group(0) @binding(5) var<uniform> parameter: Parameter;

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {

    let color_pre = intermediate_r[index_xy(global_id.xy)];

    let color_source = vec4<f32>(parameter.r, parameter.g, parameter.b, parameter.a);

    let standalization = color_source.w + (1-color_source.w) * color_pre.w;

    var color_new_rgb = vec3<f32>(0.0,0.0,0.0);

    if standalization != 0{
        color_new_rgb = (color_source.xyz * color_source.w + color_pre.xyz * (1-color_source.w) * color_pre.w) / standalization;
    }

    let color_new_a = standalization;

    var color : vec4<f32> = vec4<f32>(color_new_rgb, color_new_a);
    
    intermediate_w[global_id.x + status.width * global_id.y] = color;
        
    textureStore(outputTex, vec2<i32>(global_id.xy), color);

}

fn index_xy(v : vec2<u32>) -> u32 {
    return v.x + v.y * status.width;
}

fn index_x_y(x : u32, y : u32) -> u32 {
    return x + y * status.width;
}