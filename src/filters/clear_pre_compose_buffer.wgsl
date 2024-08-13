struct Status {
    width: u32,
    height: u32,
    frame_read : u32,
    win_width : u32,
    spare_2 : f32,
};

struct Parameter {
    index : f32,
}

@group(0) @binding(0) var<uniform> status: Status;
@group(0) @binding(1) var outputTex: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(3) var<storage, read> intermediate_r: array<vec4<f32>>;
@group(0) @binding(4) var<storage, read_write> intermediate_w: array<vec4<f32>>;
@group(0) @binding(5) var<uniform> parameter: Parameter;
@group(0) @binding(6) var<storage, read_write> pre_compose: binding_array<array<vec4<f32>>>;

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {

    let color : vec4<f32> = intermediate_r[index_xy(global_id.xy)];

    let color_null = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    
    if global_id.x < status.width{
        let indexu32 = well_indexed(parameter.index);

        pre_compose[indexu32][index_xy(global_id.xy)] = color_null;

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

fn well_indexed(index : f32) -> u32 {
    if index < 0. {
        return u32(0);
    }
    else if index > 9. {
        return u32(9);
    }
    else {
        return u32(index);
    }
}