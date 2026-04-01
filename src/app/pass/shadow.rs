use crate::app::gpu;

pub struct Pass {
    pub texture: gpu::Texture,
    pub bind_group: wgpu::BindGroup,

    pub pipeline: gpu::RenderPipeline,
}

impl Pass {
    pub fn new(
        device: &wgpu::Device,
        width: u32,
        height: u32,
        layouts: &[Option<&wgpu::BindGroupLayout>],
    ) -> Self {
        let texture =
            gpu::Texture::create_depth_texture_with_comparison_sampler(device, width, height);
        
        let mut layouts = layouts.to_vec();
        layouts.push(Some(texture.layout.as_ref().unwrap()));
        let bind_group = texture.bind_group(device).unwrap();

        let pipeline = gpu::RenderPipeline::new(
            &device,
            gpu::DEPTH_FORMAT,
            false,
            gpu::SHADER_LIGHT_RENDER,
            &layouts,
            true,
            false,
        );

        Self { pipeline, texture, bind_group }
    }

    // pub fn resize(&self) {}

    pub fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        meshes: &[gpu::MeshBuffer],
        bindings: &super::Bindings,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.texture.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            ..Default::default()
        });

        render_pass.set_pipeline(&self.pipeline.inner);

        bindings.apply(&mut render_pass);
        render_pass.set_bind_group(4, Some(&self.bind_group), &[]);

        for mesh in &meshes[1..] {
            mesh.render(&mut render_pass);
        }
    }
}
