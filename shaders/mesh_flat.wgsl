struct Globals {
    color: vec3<f32>,
    color_mode: u32,

    extra: u32,
};
@group(0) @binding(0)
var<uniform> globals: Globals;

struct Camera {
    view_proj: mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> camera: Camera;

struct Light {
    pos: vec3<f32>,
    color: vec3<f32>,
};
@group(2) @binding(0)
var<uniform> light: Light;

struct InstanceInput {
    @location(8) mat_row_0: vec4<f32>,
    @location(9) mat_row_1: vec4<f32>,
    @location(10) mat_row_2: vec4<f32>,
    @location(11) mat_row_3: vec4<f32>,
    @location(12) color: vec3<f32>,
    @location(13) color_mode: u32,
};

struct VertexInput {
    @location(0) pos: vec3<f32>,
    @location(1) tex: vec2<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) tangent: vec3<f32>,
    @location(4) bitangent: vec3<f32>,
    @location(5) color: vec3<f32>,
    @location(6) color_mode: u32,
    @location(7) extra: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex: vec2<f32>,
    @location(1) color: vec3<f32>,
};

@vertex
fn vs_main(
    vertex: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.mat_row_0,
        instance.mat_row_1,
        instance.mat_row_2,
        instance.mat_row_3,
    );

    var out: VertexOutput;
    out.tex = vertex.tex;

    if instance.color_mode == 0 {
        out.color = vertex.color;
    } else {
        out.color = instance.color;
    }

    out.clip_position = camera.view_proj * model_matrix * vec4<f32>(vertex.pos, 1.0);
    return out;
}

@group(3) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(3) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if globals.color_mode == 0 {
        return vec4<f32>(in.color, 1.0);
    } else if globals.color_mode == 1 {
        return vec4<f32>(globals.color.x, globals.color.y, globals.color.z, 1.0);
    } else if globals.color_mode == 2 {
        return textureSample(t_diffuse, s_diffuse, in.tex);
    } else if globals.color_mode == 3 {
        return vec4<f32>(light.color.x, light.color.y, light.color.z, 1.0);
    } else {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }
}