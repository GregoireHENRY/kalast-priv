struct UniformGlobal {
    test: u32,
    ambient_strength: f32,
    diffuse_enable: u32,
    specular_enable: u32,
}
@group(0) @binding(0)
var<uniform> global: UniformGlobal;

struct Camera {
    view_pos: vec4<f32>,
    view: mat4x4<f32>,
    view_proj: mat4x4<f32>,
    inv_view: mat4x4<f32>,
    inv_proj: mat4x4<f32>,
}
@group(1) @binding(0)
var<uniform> camera: Camera;

struct Light {
    pos: vec3<f32>,
    color: vec3<f32>,
}
@group(2) @binding(0)
var<uniform> light: Light;

@group(3) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(3) @binding(1)
var s_diffuse: sampler;
@group(3) @binding(2)
var t_normal: texture_2d<f32>;
@group(3) @binding(3)
var s_normal: sampler;


struct VertexInput {
    @location(0) pos: vec3<f32>,
    @location(1) tex: vec2<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) tangent: vec3<f32>,
    @location(4) bitangent: vec3<f32>,
    @location(5) color: vec3<f32>,
    @location(6) color_mode: u32,
};

struct InstanceInput {
    @location(7) model_0: vec4<f32>,
    @location(8) model_1: vec4<f32>,
    @location(9) model_2: vec4<f32>,
    @location(10) model_3: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) pos: vec3<f32>,
    @location(1) tex: vec2<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) tangent_position: vec3<f32>,
    @location(4) tangent_light_position: vec3<f32>,
    @location(5) tangent_view_position: vec3<f32>,
    @location(6) color: vec3<f32>,
    @location(7) color_mode: u32,
};

@vertex
fn vs_main(
    in: VertexInput, instance: InstanceInput
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_0,
        instance.model_1,
        instance.model_2,
        instance.model_3,
    );
    
    // let world_normal = normalize(normal_matrix * in.normal);
    let world_normal = normalize((model_matrix * vec4<f32>(in.normal, 0.0)).xyz);
    let world_tangent = normalize((model_matrix * vec4<f32>(in.tangent, 0.0)).xyz);
    let world_bitangent = normalize((model_matrix * vec4<f32>(in.bitangent, 0.0)).xyz);
    let tangent_matrix = transpose(mat3x3<f32>(world_tangent, world_bitangent, world_normal));
    let world_position = model_matrix * vec4<f32>(in.pos, 1.0);
    
    var out: VertexOutput;
    out.clip_position = camera.view_proj * world_position;
    out.pos = world_position.xyz;
    out.tex = in.tex;
    out.normal = world_normal;
    out.tangent_position = tangent_matrix * world_position.xyz;
    out.tangent_view_position = tangent_matrix * camera.view_pos.xyz;
    out.tangent_light_position = tangent_matrix * light.pos;
    out.color = in.color;
    out.color_mode = in.color_mode;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // specular: Blinn-Phong model

    if in.color_mode == 1 {
        return vec4(in.color, 1.0);
    }

    var object_color: vec4<f32>;
    var diffuse_strength: f32;
    var specular_strength: f32;

    // if in.tex.x == 0.0 && in.tex.y == 0.0 {
    if true {
        object_color = vec4(1.0);
        let light_dir = normalize(light.pos - in.pos);
        let view_dir = normalize(camera.view_pos.xyz - in.pos);
        let half_dir = normalize(view_dir + light_dir);
        diffuse_strength = max(dot(in.normal, light_dir), 0.0);
        specular_strength = pow(max(dot(in.normal, half_dir), 0.0), 32.0);
    }
    else {
        object_color = textureSample(t_diffuse, s_diffuse, in.tex);
        let object_normal = textureSample(t_normal, s_normal, in.tex);
        let tangent_normal = object_normal.xyz * 2.0 - 1.0;
        let light_dir = normalize(in.tangent_light_position - in.tangent_position);
        let view_dir = normalize(in.tangent_view_position - in.tangent_position);
        let half_dir = normalize(view_dir + light_dir);
        diffuse_strength = max(dot(tangent_normal, light_dir), 0.0);
        specular_strength = pow(max(dot(tangent_normal, half_dir), 0.0), 32.0);
    }

    let ambient_color = global.ambient_strength * light.color;
    let diffuse_color = diffuse_strength * light.color;
    let specular_color = specular_strength * light.color;

    var color: vec3<f32> = ambient_color;

    if global.diffuse_enable == 1 {
        color = color + diffuse_color;
    }
    if global.specular_enable == 1 {
        color = color + specular_color;
    }

    return vec4<f32>(color * object_color.xyz, object_color.a);
}