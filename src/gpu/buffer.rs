/*
use bytemuck::{Pod, Zeroable};

pub const VERTICES_TRIANGLE: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];

pub const VERTICES_PENTAGON: &[Vertex] = &[
    Vertex {
        position: [-0.0868241, 0.49240386, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
        color: [0.5, 0.0, 0.5],
    },
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        color: [0.5, 0.0, 0.5],
    },
];
*/

/*
pub const VERTICES_PENTAGON: &[Vertex] = &[
    Vertex {
        position: [-0.0868241, 0.49240386, 0.0],
        tex: [0.4131759, 0.00759614],
    },
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
        tex: [0.0048659444, 0.43041354],
    },
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        tex: [0.28081453, 0.949397],
    },
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
        tex: [0.85967, 0.84732914],
    },
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        tex: [0.9414737, 0.2652641],
    },
];

pub const INDICES_PENTAGON: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

pub const DEPTH_VERTICES_FULL: &[Vertex] = &[
    Vertex {
        position: [-1.0, -1.0, 0.0],
        tex: [0.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0, 0.0],
        tex: [1.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0, 0.0],
        tex: [1.0, 0.0],
    },
    Vertex {
        position: [-1.0, 1.0, 0.0],
        tex: [0.0, 0.0],
    },
];
*/

use wgpu::util::DeviceExt;

use crate::{Vec2, Vec3};

pub const DEPTH_VERTICES_QUARTER_TOP_RIGHT: [crate::mesh::Vertex; 4] = [
    crate::mesh::Vertex {
        pos: Vec3::new(0.0, 0.0, 0.0),
        tex: Vec2::new(0.0, 1.0),
        ..crate::mesh::Vertex::default()
    },
    crate::mesh::Vertex {
        pos: Vec3::new(1.0, 0.0, 0.0),
        tex: Vec2::new(1.0, 1.0),
        ..crate::mesh::Vertex::default()
    },
    crate::mesh::Vertex {
        pos: Vec3::new(1.0, 1.0, 0.0),
        tex: Vec2::new(1.0, 0.0),
        ..crate::mesh::Vertex::default()
    },
    crate::mesh::Vertex {
        pos: Vec3::new(0.0, 1.0, 0.0),
        tex: Vec2::new(0.0, 0.0),
        ..crate::mesh::Vertex::default()
    },
];

pub const DEPTH_INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];

pub struct UniformBind<U>
// where U: bytemuck::NoUninit
{
    pub uniform: U,
    pub buffer: wgpu::Buffer,
    pub layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl<U> UniformBind<U>
where
    U: bytemuck::NoUninit,
{
    pub fn write(&self, queue: &wgpu::Queue) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.uniform]));
    }
}

pub trait UniformBindTrait
where
    Self: bytemuck::NoUninit,
{
    fn register(self, device: &wgpu::Device, binding: u32) -> UniformBind<Self> {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[self]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: None,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding,
                resource: buffer.as_entire_binding(),
            }],
            label: None,
        });

        UniformBind {
            uniform: self,
            buffer,
            layout,
            bind_group,
        }
    }
}
