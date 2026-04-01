use crate::app::gpu;

pub struct Pass {
    pub pipeline: gpu::RenderPipeline,
}

impl Pass {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        config: &crate::app::config::Config,
        layouts: &[Option<&wgpu::BindGroupLayout>],
    ) -> Self {
        let pipeline = gpu::RenderPipeline::new(
            &device,
            format,
            config.render_back_face,
            gpu::SHADER_MESH_SHADOW,
            layouts,
            true,
            true,
        );

        Self { pipeline }
    }

    pub fn resize(&self) {}

    pub fn render(
        &self,
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        depth_view: &wgpu::TextureView,
        light: &super::light_cube::Pass,
        shadow: &super::shadow::Pass,
        meshes: &[gpu::MeshBuffer],
        bindings: &super::Bindings,
        config: &crate::app::config::Config,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(config.background),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: depth_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });

        if config.debug_light_cube_show {
            light.render(&mut render_pass, &meshes[0], bindings);
        }

        render_pass.set_pipeline(&self.pipeline.inner);

        bindings.apply(&mut render_pass);

        for mesh in &meshes[1..] {
            mesh.render(&mut render_pass);
        }
    }
}
