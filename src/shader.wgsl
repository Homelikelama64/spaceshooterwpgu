struct VertexInput {
    @builtin(vertex_index) vertex_index: u32,
    @builtin(instance_index) quad_index: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) quad_index: u32,
    @location(1) uv: vec2<f32>,
};

struct Camera {
    view_height: f32,
    aspect: f32,
};

@group(0)
@binding(0)
var<uniform> camera: Camera;

struct Quad {
    position: vec2<f32>,
    size: vec2<f32>,
    color: vec4<f32>,
};

@group(1)
@binding(0)
var<storage, read> quads: array<Quad>;

@vertex
fn vertex(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.quad_index = input.quad_index;

    output.uv = vec2<f32>(
        f32((input.vertex_index >> 0u) & 1u),
        f32((input.vertex_index >> 1u) & 1u),
    );

    let quad = quads[input.quad_index];
    let world_position = (output.uv - 0.5) * quad.size + quad.position;

    output.clip_position = vec4<f32>(world_position / (camera.view_height * vec2<f32>(camera.aspect, 1.0)), 0.0, 1.0);

    return output;
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let quad = quads[input.quad_index];
    return quad.color;
}
