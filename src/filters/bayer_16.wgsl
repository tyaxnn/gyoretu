struct Status {
    width: u32,
    height: u32,
    frame_read : u32,
    spare_1 : u32,
    spare_2 : f32,
};

struct Parameter {
    r1 : f32,
    g1 : f32,
    b1 : f32,
    a1 : f32,
    r2 : f32,
    g2 : f32,
    b2 : f32,
    a2 : f32,
}


@group(0) @binding(0) var<uniform> status: Status;
@group(0) @binding(1) var outputTex: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(3) var<storage, read> intermediate_r: array<vec4<f32>>;
@group(0) @binding(4) var<storage, read_write> intermediate_w: array<vec4<f32>>;
@group(0) @binding(5) var<uniform> parameter: Parameter;

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {

    var color : vec4<f32> = vec4<f32>(parameter.r1,parameter.g1,parameter.b1, parameter.a1);

    let mono_color_f = dot(intermediate_r[index_xy(global_id.xy)].xyz,vec3<f32>(1./3.,1./3.,1./3.));

    var bayer_16 : array<u32,16> = array<u32,16>(0,8,2,10,12,4,14,6,3,11,1,9,15,7,13,5);
    
    let idx = global_id.x % 4 * 4 + global_id.y % 4;

    if u32(mono_color_f * 256.) > 16 * bayer_16[idx]{
        color = vec4<f32>(parameter.r2, parameter.g2,parameter.b2, parameter.a2);
    }
    
    intermediate_w[index_xy(global_id.xy)] = color;
        
    textureStore(outputTex, vec2<i32>(global_id.xy), color);

}

fn index_xy(v : vec2<u32>) -> u32 {
    return v.x + v.y * status.width;
}

fn index_x_y(x : u32, y : u32) -> u32 {
    return x + y * status.width;
}
