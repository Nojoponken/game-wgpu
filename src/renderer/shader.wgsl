// Vertex shader
struct CameraUniform {
    view_position: vec4<f32>,
    view_proj: mat4x4<f32>,
};
@group(1) @binding(0) // 1.
var<uniform> camera: CameraUniform;

struct Light {
    direction: vec3<f32>,
    color: vec3<f32>,
}

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) ao: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) ao: f32,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    out.tex_coords = model.tex_coords;
    out.normal = model.normal;
    out.ao = model.ao;
    return out;
}

// Fragment shader
@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var light: Light;
    light.direction = normalize(vec3<f32>(1.0, 3.0, 2.0));
    light.color = vec3<f32>(1.0, 1.0, 1.0);

    var ambient = vec4(1.0, 1.0, 1.0, 1.0);

    var diffuseStrength = max(dot(in.normal, light.direction), 0.0);
    var diffuse = diffuseStrength * light.color;

    var lighting =  (ambient * 0.3 + vec4(diffuse, 1.0) * 0.7);

    return textureSample(t_diffuse, s_diffuse, in.tex_coords) * lighting * (in.ao*0.9+0.1);
}
