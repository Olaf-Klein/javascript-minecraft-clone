// Vertex shader
struct CameraUniform {
    view_proj: mat4x4<f32>,
    view_pos: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) color: vec3<f32>,
    @location(3) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) color: vec3<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) world_pos: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.world_pos = model.position;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    out.tex_coords = model.tex_coords;
    out.color = model.color;
    out.normal = model.normal;
    return out;
}

@group(1) @binding(0)
var atlas_texture: texture_2d<f32>;

@group(1) @binding(1)
var atlas_sampler: sampler;
@group(2) @binding(0)
var shadow_map: texture_depth_2d;
@group(2) @binding(1)
var shadow_sampler: sampler_comparison;

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let light_dir = normalize(vec3<f32>(0.5, 1.0, 0.3));
    let ambient = 0.4;
    let diffuse = max(dot(normalize(in.normal), light_dir), 0.0) * 0.6;
    var lighting = ambient + diffuse;

    // Simple shadow lookup (placeholder): project world_pos by light matrix supplied as part of camera
    // NOTE: real integration requires a light-view-proj uniform; for now we assume the shader receives
    // shadow coordinates in in.world_pos.xyzw via a precomputed pipeline. This is placeholder logic.
    // A robust implementation will pass a light matrix and compute shadow_uv and compare.
    // Here we sample shadow_map with a comparison sampler at the fragment's xy (naive) just to show binding.
    // This will likely be replaced with proper shadow coords.
    let shadow_uv = in.tex_coords; // placeholder
    let shadow_sample = textureSampleCompare(shadow_map, shadow_sampler, shadow_uv, 0.5);
    if shadow_sample < 0.5 {
        lighting = lighting * 0.6; // darken in shadow
    }
    
    let sampled = textureSample(atlas_texture, atlas_sampler, in.tex_coords);
    let base_color = sampled.rgb * in.color;
    let final_color = base_color * lighting;
    return vec4<f32>(final_color, sampled.a);
}
