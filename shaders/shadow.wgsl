struct Light {
    view_proj: mat4x4<f32>,
};
@group(2) @binding(0)
var<uniform> light: Light;

struct InstanceInput {
    @location(8) mat_row_0: vec4<f32>,
    @location(9) mat_row_1: vec4<f32>,
    @location(10) mat_row_2: vec4<f32>,
    @location(11) mat_row_3: vec4<f32>,
};

struct VertexInput {
    @location(0) pos: vec3<f32>,
};

@vertex
fn vs_main(
    vertex: VertexInput,
    instance: InstanceInput,
) -> @builtin(position) vec4<f32> {
    let model_matrix = mat4x4<f32>(
        instance.mat_row_0,
        instance.mat_row_1,
        instance.mat_row_2,
        instance.mat_row_3,
    );

    return light.view_proj * model_matrix * vec4<f32>(vertex.pos, 1.0);
}
