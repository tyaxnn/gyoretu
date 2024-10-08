struct Status {
    width: u32,
    height: u32,
    frame_read : u32,
    win_width : u32,
    spare_2 : f32,
};

struct Parameter {
    s : f32,
    r : f32,
    g : f32,
    b : f32,
}


@group(0) @binding(0) var<uniform> status: Status;
@group(0) @binding(1) var outputTex: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(3) var<storage, read> intermediate_r: array<vec4<f32>>;
@group(0) @binding(4) var<storage, read_write> intermediate_w: array<vec4<f32>>;
@group(0) @binding(5) var<uniform> parameter: Parameter;

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {

    var color : vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    
    color = intermediate_r[index_xy(global_id.xy)];

    let sum_p = parameter.r + parameter.g + parameter.b;

    let mono_color_f = dot(color.xyz,vec3<f32>(parameter.r,parameter.g,parameter.b) / sum_p);

    color = color * (1.- parameter.s) + vec4<f32>(mono_color_f,mono_color_f,mono_color_f,1.) * parameter.s;
    
    if global_id.x < status.width{
        intermediate_w[index_xy(global_id.xy)] = color;
                
        textureStore(outputTex, vec2<i32>(global_id.xy), color);
    }

}

fn index_xy(v : vec2<u32>) -> u32 {
    return v.x + v.y * status.width;
}

fn index_x_y(x : u32, y : u32) -> u32 {
    return x + y * status.width;
}