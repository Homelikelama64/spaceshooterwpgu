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
    position: vec2<f32>,
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
    rotation: f32,
};

@group(1)
@binding(0)
var<storage, read> quads: array<Quad>;

@group(2)
@binding(0)
var texture: texture_2d<f32>;

@group(2)
@binding(1)
var texture_sampler: sampler;

@vertex
fn vertex(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.quad_index = input.quad_index;

    output.uv = vec2<f32>(
        f32((input.vertex_index >> 0u) & 1u),
        f32((input.vertex_index >> 1u) & 1u),
    );

    let quad = quads[input.quad_index];
    let world_position = rotate_vector((output.uv - 0.5) * quad.size, quad.rotation);

    output.clip_position = vec4<f32>((world_position + quad.position - camera.position) / (camera.view_height * vec2<f32>(camera.aspect, 1.0)), 0.0, 1.0);

    return output;
}

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    let quad = quads[input.quad_index];
    let texture_color = textureSample(texture, texture_sampler, input.uv);
    return texture_color * quad.color;
}

fn rotate_vector(vector: vec2<f32>, rotation: f32) -> vec2<f32> {
    return vec2<f32>(
        cos(rotation) * vector.x - sin(rotation) * vector.y,
        sin(rotation) * vector.x + cos(rotation) * vector.y,
    );
}
