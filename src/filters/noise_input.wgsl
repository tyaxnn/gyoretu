struct Status {
    width: u32,
    height: u32,
    frame_read : u32,
    win_width : u32,
    spare_2 : f32,
};
struct Parameter {
    density : f32,
}


@group(0) @binding(0) var<uniform> status: Status;
@group(0) @binding(1) var outputTex: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(3) var<storage, read> intermediate_r: array<vec4<f32>>;
@group(0) @binding(4) var<storage, read_write> intermediate_w: array<vec4<f32>>;
@group(0) @binding(5) var<uniform> parameter: Parameter;

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {

    let color_pre = intermediate_r[index_xy(global_id.xy)];

    let xyf32 = vec2<f32>(f32(global_id.x)/f32(status.width),f32(global_id.y)/f32(status.height));
    let noise = my_rand(xyf32,status.frame_read);

    let color_source : vec4<f32> = vec4<f32>(noise, noise, noise, noise);
    
    let standalization = color_source.w + (1-color_source.w) * color_pre.w;

    var color_new_rgb = vec3<f32>(0.0,0.0,0.0);

    if standalization != 0{
        color_new_rgb = (color_source.xyz * color_source.w + color_pre.xyz * (1-color_source.w) * color_pre.w) / standalization;
    }



    let color_new_a = standalization;

    var color : vec4<f32> = vec4<f32>(color_new_rgb, color_new_a);
    
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

fn my_rand(xy : vec2<f32> , seed : u32) -> f32{

    let offset = f32(seed * 50873 % 46619) / 329.86;
    
    let randvalue =  fract(sin(dot(xy,
                         vec2<f32>(12.9898 + offset * 0.17,78.233 + offset * 0.13)))*
        43758.5453123 + offset * 101.);

    if randvalue < 1. - parameter.density{
        return f32(0.);
    }
    else {
        return f32(1.);
    }
}