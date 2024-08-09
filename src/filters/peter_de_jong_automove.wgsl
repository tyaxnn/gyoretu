struct Status {
    width: u32,
    height: u32,
    frame_read : u32,
    elapsed_frame : u32,
    spare_2 : f32,
};

struct Parameter{
    aamp : f32,
    bamp : f32,
    camp : f32,
    damp : f32,
    av : f32,
    bv : f32,
    cv : f32,
    dv : f32,
    mul_x : f32,
    mul_y : f32,
    clear_strength : f32,
}


@group(0) @binding(0) var<uniform> status: Status;
@group(0) @binding(1) var outputTex: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(3) var<storage, read> intermediate_r: array<vec4<f32>>;
@group(0) @binding(4) var<storage, read_write> intermediate_w: array<vec4<f32>>;
@group(0) @binding(5) var<uniform> parameter: Parameter;

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {

    let x = f32(global_id.x);
    let y = f32(global_id.y);
    let w = f32(status.width);
    let h = f32(status.height);

    let a = parameter.aamp * sin(parameter.av * f32(status.frame_read) * 0.01);
    let b = parameter.bamp * sin(parameter.bv * f32(status.frame_read) * 0.01);
    let c = parameter.camp * sin(parameter.cv * f32(status.frame_read) * 0.01);
    let d = parameter.damp * sin(parameter.dv * f32(status.frame_read) * 0.01);

    let x_prime = (sin(a * y/h) - cos(b * x/w) + 2.) * w * parameter.mul_x / 4.;
    let y_prime = (sin(c * x/w) - cos(d * y/h) + 2.) * h * parameter.mul_y / 4.;

    var pre_color = intermediate_r[index_xy(global_id.xy)];
    var color : vec4<f32> = vec4<f32>(1.0, 1.0, 1.0, 1.0);

    if fine(pre_color) {

        let index : vec2<u32> = vec2<u32>(u32(x_prime),u32(y_prime));

        pre_color.w = max(pre_color.w - parameter.clear_strength,0.);

        intermediate_w[index_xy(index)] = color;
        textureStore(outputTex, vec2<i32>(index), color);

        intermediate_w[index_xy(global_id.xy)] = pre_color;
        textureStore(outputTex, vec2<i32>(global_id.xy), pre_color);
    }
    
}

fn index_xy(v : vec2<u32>) -> u32 {
    return v.x + v.y * status.width;
}

fn index_x_y(x : u32, y : u32) -> u32 {
    return x + y * status.width;
}

fn fine(input : vec4<f32>) -> bool {
    if input.x == f32(1.) && input.y == f32(1.) && input.z == f32(1.) && input.w != 0.0{
        return true;
    }
    else {
        return false;
    }
}