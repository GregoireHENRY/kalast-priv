struct Globals {
    color: vec3<f32>,
    color_mode: u32,
    ambient_strength: f32,
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
    view_proj: mat4x4<f32>,
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
    @location(12) normal_row_0: vec3<f32>,
    @location(13) normal_row_1: vec3<f32>,
    @location(14) normal_row_2: vec3<f32>,
    // @location(16) color: vec3<f32>,
    // @location(17) color_mode: u32,
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
    @location(2) world_normal: vec3<f32>,
    @location(3) world_pos: vec3<f32>,
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

    let normal_matrix = mat3x3<f32>(
        instance.normal_row_0.xyz,
        instance.normal_row_1.xyz,
        instance.normal_row_2.xyz,
    );

    var out: VertexOutput;
    out.tex = vertex.tex;

    // if instance.color_mode == 0 {
    //     out.color = vertex.color;
    // } else {
    //     out.color = instance.color;
    // }

    out.color = vertex.color;

    // out.world_normal = normal_matrix * vertex.normal;
    out.world_normal = normalize(normal_matrix * vertex.normal);

    var world_pos = model_matrix * vec4<f32>(vertex.pos, 1.0);
    out.world_pos = world_pos.xyz;

    out.clip_position = camera.view_proj * world_pos;
    return out;
}

@group(3) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(3) @binding(1)
var s_diffuse: sampler;

@group(4) @binding(0)
var t_shadow: texture_depth_2d;
@group(4) @binding(1)
var s_shadow: sampler_comparison;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var object_color: vec4<f32>;

    if globals.color_mode == 0 {
        object_color = vec4<f32>(in.color, 1.0);
    } else if globals.color_mode == 1 {
        return vec4<f32>(in.color, 1.0);
    } else if globals.color_mode == 2 {
        return vec4<f32>(globals.color.x, globals.color.y, globals.color.z, 1.0);
    } else if globals.color_mode == 3 {
        object_color = textureSample(t_diffuse, s_diffuse, in.tex);
    } else {
        return vec4<f32>(1.0, 1.0, 1.0, 1.0);
    }

    // shadow
    // maybe needed to flip Y
    // let uv = vec2<f32>(proj.x, -proj.y) * 0.5 + 0.5;
    let light_space = light.view_proj * vec4<f32>(in.world_pos, 1.0);
    let proj = light_space.xyz / light_space.w;
    let uv = proj.xy * 0.5 + 0.5;
    let depth = proj.z * 0.5 + 0.5;
    let bias = 0.005; // to avoid acne
    let shadow = textureSampleCompare(
        t_shadow,
        s_shadow,
        uv,
        depth - bias
    );

    // ambient
    let ambient_color = light.color * globals.ambient_strength;

    // diffuse
    let light_dir = normalize(light.pos - in.world_pos);
    let diffuse_strength = max(dot(in.world_normal, light_dir), 0.0);
    // let diffuse_color = light.color * diffuse_strength;
    let diffuse_color = light.color * diffuse_strength * shadow;

    let result = (ambient_color + diffuse_color) * object_color.xyz;

    return vec4<f32>(result, object_color.a);
}