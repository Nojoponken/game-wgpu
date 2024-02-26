
struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
};


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
    instance: InstanceInput,
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    var out: VertexOutput;
    out.clip_position = camera.view_proj * model_matrix * vec4<f32>(model.position, 1.0);
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

struct FragmentOutput{
    @location(0) color: vec4<f32>,
}

@fragment
fn fs_main(in: VertexOutput) -> FragmentOutput {
    var light: Light;
    light.direction = normalize(vec3<f32>(1.0, 3.0, 2.0));
    light.color = vec3<f32>(1.0, 1.0, 1.0);

    var ambient = vec3(1.0, 1.0, 1.0);

    var diffuseStrength = max(dot(in.normal, light.direction), 0.0);
    var diffuse = diffuseStrength * light.color;

    var lighting =  (ambient * 0.3 + diffuse * 0.7) * (in.ao*0.9+0.1);

    var shaded = textureSample(t_diffuse, s_diffuse, in.tex_coords) * vec4(lighting, 1.0) ;

    var depth = in.clip_position.z / in.clip_position.w;
    var uvx = (in.clip_position.x/40)-10;
    var corrected_depth = sqrt(depth*depth + uvx*uvx);
    var moved_depth = 2.0*max(corrected_depth/32.0 - 0.5, 0.0);
    var fog = moved_depth*sqrt(moved_depth);
    var fog_color =vec4(0.8,0.8,0.8,1.0);

    var out: FragmentOutput;

    out.color = (shaded + fog*fog_color)/(1+fog);

    return out;
}
