use crate::app::gpu;

pub struct Pass {
    pub pipeline: gpu::RenderPipeline,
}

impl Pass {
    pub fn new(device: &wgpu::Device, layouts: &[Option<&wgpu::BindGroupLayout>]) -> Self {
        let pipeline = gpu::RenderPipeline::new(
            &device,
            gpu::DEPTH_FORMAT,
            None, // Some(wgpu::Face::Front),
            gpu::SHADER_SHADOW,
            &layouts,
            true,
            false,
        );

        Self { pipeline }
    }

    // pub fn resize(&self) {}

    pub fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        shadow: &gpu::Texture,
        meshes: &[gpu::MeshBuffer],
        bindings: &super::Bindings,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &shadow.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });

        render_pass.set_pipeline(&self.pipeline.inner);

        bindings.for_shadow(&mut render_pass);

        for mesh in &meshes[1..] {
            mesh.render(&mut render_pass);
        }
    }
}
