use crate::{Mat4, Vec2, Vec3};

const DEPTH_VERTICES: &[crate::mesh::Vertex] = &[
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

const DEPTH_INDICES: &[u32] = &[0, 1, 2, 0, 2, 3];

pub struct DepthPass {
    pub texture: super::gpu::Texture,
    pub mesh: super::gpu::MeshBuffer,

    pub pipeline: super::gpu::RenderPipeline,
}

impl DepthPass {
    pub fn new(device: &wgpu::Device, surface_config: &wgpu::SurfaceConfiguration) -> Self {
        let texture = super::gpu::Texture::create_depth_texture_non_comparison_sampler(
            device,
            surface_config.width,
            surface_config.height,
        );

        let mesh =
            super::gpu::MeshBuffer::new(device, DEPTH_VERTICES, DEPTH_INDICES, Mat4::IDENTITY);

        let pipeline = super::gpu::RenderPipeline::new(
            &device,
            surface_config.format,
            false,
            super::gpu::SHADER_DEPTH_RENDER,
            &[Some(&texture.bind.as_ref().unwrap().layout)],
            false,
        );

        Self {
            texture,
            mesh,
            pipeline,
        }
    }

    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        self.texture =
            super::gpu::Texture::create_depth_texture_non_comparison_sampler(device, width, height);
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.texture.bind.as_ref().unwrap().layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.texture.sampler),
                },
            ],
            label: None,
        });

        self.texture.bind.as_mut().unwrap().bind_group = bind_group;
    }

    pub fn render(&self, view: &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
            multiview_mask: None,
            label: None,
        });
        render_pass.set_pipeline(&self.pipeline.inner);
        render_pass.set_bind_group(0, &self.texture.bind.as_ref().unwrap().bind_group, &[]);
        self.mesh.render(&mut render_pass);
    }
}
