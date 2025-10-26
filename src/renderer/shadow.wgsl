// Simple depth-only shader for shadow map rendering

struct VSInput {
    @location(0) position: vec3<f32>,
};

struct VSOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_pos: vec3<f32>,
};

@group(0) @binding(0)
var<uniform> light_view_proj: mat4x4<f32>;

@vertex
fn vs_main(input: VSInput) -> VSOutput {
    var out: VSOutput;
    out.world_pos = input.position;
    out.clip_position = light_view_proj * vec4<f32>(input.position, 1.0);
    return out;
}

@fragment
fn fs_main() {
    // depth-only pass; nothing to output
}
