struct Status {
    width: u32,
    height: u32,
    frame_read : u32,
    win_width : u32,
    spare_2 : f32,
};

struct Parameter {
    parallel_x : f32,
    parallel_y : f32,
    expantion_x : f32,
    expantion_y : f32,
    rotate : f32,
}

const PI: f32 = 3.141592653;


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

    let x_pre_rotate = (x - w/2.) * cos(parameter.rotate * PI * 2.) - (y - h/2.) * sin(parameter.rotate * PI * 2.);
    let y_pre_rotate = (x - w/2.) * sin(parameter.rotate * PI * 2.) + (y - h/2.) * cos(parameter.rotate * PI * 2.);

    let x_prime = x_pre_rotate / parameter.expantion_x + w/2. - parameter.parallel_x;

    let y_prime = y_pre_rotate / parameter.expantion_y + h/2. - parameter.parallel_y;

    let color = anti_aliasing(x_prime,y_prime);

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

fn anti_aliasing(x : f32, y : f32) -> vec4<f32> {

    if x < 0. || y < 0. {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }
    else {
        let xu32 = u32(x);
        let yu32 = u32(y);

        let s = x - f32(xu32);
        let t = y - f32(yu32);

        var o_00 = vec4<f32>(0.0, 0.0, 0.0, 0.0);
        var o_01 = vec4<f32>(0.0, 0.0, 0.0, 0.0);
        var o_10 = vec4<f32>(0.0, 0.0, 0.0, 0.0);
        var o_11 = vec4<f32>(0.0, 0.0, 0.0, 0.0);


        if is_ranged(xu32,yu32){
            o_00 = intermediate_r[index_x_y(xu32,yu32)];
        }
        if is_ranged(xu32,yu32 + 1){
            o_01 = intermediate_r[index_x_y(xu32,yu32 + 1)];
        }
        if is_ranged(xu32 + 1,yu32){
            o_10 = intermediate_r[index_x_y(xu32 + 1,yu32)];
        }
        if is_ranged(xu32 + 1,yu32 + 1){
            o_11 = intermediate_r[index_x_y(xu32 + 1,yu32 + 1)];
        }

        return o_11 * s * t + o_10 * s * (1.-t) + o_01 * (1.-s) * t + o_00 * (1.-s) * (1.-t);
    }

}

fn is_ranged(x : u32, y : u32) -> bool {
    if 0 <= x && x <= status.width && 0 <= y && y <= status.height{
        return true;
    }
    else{
        return false;
    }
}

fn square(x : f32) -> f32 {
    return x * x;
}