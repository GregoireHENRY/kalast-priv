use crate::{Mat4, Vec3};

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Globals {
    // global color used in fragment if color mode is 1
    pub color: Vec3,

    // Control fragment color
    // - 0: vertex/instance color + lighting
    // - 1: vertex/instance color, no lighting, show raw color
    // - 2: globals color
    // - 3: texture sample
    // - else: white
    pub color_mode: u32,

    pub ambient_strength: f32,

    pub extra: u32,

    pub _padding1: u32,
    pub _padding2: u32,
    pub _padding3: u32,
    // pub _padding4: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Camera {
    pub view_proj: Mat4,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Light {
    pub view_proj: Mat4,

    pub pos: Vec3,
    pub _padding: u32,

    pub color: Vec3,
    pub _padding2: u32,
}

pub struct Uniforms {
    pub globals: super::gpu::UniformBuffer<Globals>,
    pub camera: super::gpu::UniformBuffer<Camera>,
    pub light: super::gpu::UniformBuffer<Light>,
    pub textures: Vec<super::gpu::Texture>,
}

impl Uniforms {
    pub fn bind_group_layouts(&self) -> Vec<Option<&wgpu::BindGroupLayout>> {
        vec![
            Some(&self.globals.layout),
            Some(&self.camera.layout),
            Some(&self.light.layout),
            Some(&self.textures[0].layout.as_ref().unwrap()),
        ]
    }

    pub fn bind_groups(&self, device: &wgpu::Device) -> super::pass::Bindings {
        super::pass::Bindings {
            globals: self.globals.bind_group(device),
            camera: self.camera.bind_group(device),
            light: self.light.bind_group(device),
            texture: self.textures[0].bind_group(device).unwrap(),
        }
    }
}
