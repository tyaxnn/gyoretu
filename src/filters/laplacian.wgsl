struct Status {
    width: u32,
    height: u32,
    frame_read : u32,
    win_width : u32,
    spare_2 : f32,
};


@group(0) @binding(0) var<uniform> status: Status;
@group(0) @binding(1) var outputTex: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(3) var<storage, read> intermediate_r: array<vec4<f32>>;
@group(0) @binding(4) var<storage, read_write> intermediate_w: array<vec4<f32>>;

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {

    let x = i32(global_id.x);
    let y = i32(global_id.y);

    let o_00 = intermediate_r[index_xy(global_id.xy)];

    var o_mm = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    var o_mp = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    var o_pm = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    var o_pp = vec4<f32>(0.0, 0.0, 0.0, 0.0);

    if is_ranged(x-1,y-1) {
        o_mm = intermediate_r[index_x_y(global_id.x - 1, global_id.y - 1)];
    }
    if is_ranged(x-1,y+1) {
        o_mp = intermediate_r[index_x_y(global_id.x - 1, global_id.y + 1)];
    }
    if is_ranged(x+1,y-1) {
        o_pm = intermediate_r[index_x_y(global_id.x + 1, global_id.y - 1)];
    }
    if is_ranged(x-1,y-1) {
        o_pp = intermediate_r[index_x_y(global_id.x + 1, global_id.y + 1)];
    }

    let color = (((o_mm + o_mp + o_pm + o_pp) * 0.25 - o_00) + vec4<f32>(1.0, 1.0, 1.0, 1.0)) * 0.5;

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

fn is_ranged(x : i32, y : i32) -> bool {
    if 0 <= x && x <= i32(status.width) && 0 <= y && y <= i32(status.height){
        return true;
    }
    else{
        return false;
    }
}