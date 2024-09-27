struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) world_position: vec2<f32>,
};

struct Camera {
    position: vec2<f32>,
    view_height: f32,
    aspect: f32,
};

@group(0)
@binding(0)
var<uniform> camera: Camera;

@vertex
fn vertex(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    output.clip_position = vec4<f32>(
        f32((input.vertex_index >> 0u) & 1u) * 2.0 - 1.0,
        f32((input.vertex_index >> 1u) & 1u) * 2.0 - 1.0,
        0.0,
        1.0,
    );
    let scale = 0.5;
    output.world_position = camera.position * scale + output.clip_position.xy * (camera.view_height * scale * vec2<f32>(camera.aspect, 1.0));

    return output;
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let offset_x = f32(100000);
    let offset_y = f32(10000);
    var output = vec4<f32>(
        0.0,
        0.0,
        0.0,
        1.0,
    );

    var state = bitcast<u32>(i32(input.world_position.x + offset_x)) * bitcast<u32>(i32(input.world_position.y + offset_y));

    let value = random_value(&state);
    if random_value(&state) < 1.0 / camera.view_height {
        output = vec4<f32>(
            value,
            value,
            value,
            1.0,
        );
    }


    return output;
}

fn random_value(state: ptr<function, u32>) -> f32 {
    *state = *state * 747796405u + 2891336453u;
    var result = ((*state >> ((*state >> 28u) + 4u)) ^ *state) * 277803737u;
    result = (result >> 22u) ^ result;
    return f32(result) / 4294967295.0;
}
