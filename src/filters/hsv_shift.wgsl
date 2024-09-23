struct Status {
    width: u32,
    height: u32,
    frame_read : u32,
    win_width : u32,
    spare_2 : f32,
};

struct Parameter {
    h_shift : f32,
    s_shift : f32,
    v_shift : f32,
}


@group(0) @binding(0) var<uniform> status: Status;
@group(0) @binding(1) var outputTex: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(3) var<storage, read> intermediate_r: array<vec4<f32>>;
@group(0) @binding(4) var<storage, read_write> intermediate_w: array<vec4<f32>>;
@group(0) @binding(5) var<uniform> parameter: Parameter;

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    
    let pre_color = intermediate_r[index_xy(global_id.xy)];

    var color_hsv = rgb_to_hsv(pre_color.xyz);

    color_hsv.x = fract(color_hsv.x + parameter.h_shift);

    color_hsv.y = parabolic_interpolation(color_hsv.y, parameter.s_shift);

    color_hsv.z = parabolic_interpolation(color_hsv.z, parameter.v_shift);

    let color = vec4<f32>(hsv_to_rgb(color_hsv),pre_color.a);
    
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

fn rgb_to_hsv(rgb : vec3<f32>) -> vec3<f32> {
    let r = rgb.x;
    let g = rgb.y;
    let b = rgb.z;

    var h = 0.;
    var s = 0.;
    var v = 0.;
    var mul = 1.;

    var min = 0.;
    var mid = 0.;
    var max = 0.;

    //立方体を6つに分割
    if g < r && b < r{
        max = r;
        if g < b {
            h = 1.;
            mul = -1.;

            mid = b;
            min = g;
        }else {
            mid = g;
            min = b;
        }
    } 
    else if r <= g && b < g{
        max = g;
        h = 1./3.;
        if b < r {
            mul = -1.;

            mid = r;
            min = b;
        }
        else {
            mid = b;
            min = r;
        }
    }
    else {
        max = b;
        h = 2./3.;
        if r < g{
            mul = -1.;

            mid = g;
            min = r;
        }
        else {
            mid = r;
            min = g;
        }
    }

    //O_alpha を立方体に投影 (r,g,b) -> (1, mid_beta, min_beta) (r>g>bの場合の例)

    var min_beta = 0.;
    var mid_beta = 0.;

    v = max;
    if v != 0 {
        min_beta = min / v;
        mid_beta = mid / v;
    }

    //W_beta を立方体の辺に投影 (1, mid_beta, min_beta) -> (1, gamma, 0) (r>g>bの場合の例)

    var gamma = 1.;

    s = 1. - min_beta;

    if min_beta != 1.{
        gamma = 1. - (1. - mid_beta) / s;
    }

    //色相を計算する

    h = h + mul * gamma / 6.;

    return vec3<f32>(h,s,v);

}

fn hsv_to_rgb(hsv : vec3<f32>) -> vec3<f32> {

    let h = hsv.x;
    let s = hsv.y;
    let v = hsv.z;

    var gamma : vec3<f32> = vec3<f32>(0., 0., 0.);

    if h < 1./6. {
        gamma = vec3<f32>(1., h * 6., 0.);
    }
    else if h < 2./6. {
        gamma = vec3<f32>(2. - h * 6., 1., 0.);
    }
    else if h < 3./6. {
        gamma = vec3<f32>(0., 1., h * 6 - 2.);
    }
    else if h < 4./6. {
        gamma = vec3<f32>(0., 4. - h * 6, 1.);
    }    
    else if h < 5./6. {
        gamma = vec3<f32>(h * 6. - 4., 0., 1.);
    }
    else {
        gamma = vec3<f32>(1., 0., 6. - 6. * h);
    }

    let beta = gamma * s + vec3<f32>(1., 1., 1.) * (1. - s);

    let alpha = beta * v;

    return alpha;
}

fn parabolic_interpolation(p : f32, x : f32) -> f32 {
    let a = 1./2. - p;
    let b = 1./2.;
    let c = p;

    let y = a * x * x + b * x + c;

    return y;
}
