use crate::app::gpu;

pub struct Pass {
    pub pipeline: gpu::RenderPipeline,
}

impl Pass {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        layouts: &[Option<&wgpu::BindGroupLayout>],
    ) -> Self {
        let pipeline = gpu::RenderPipeline::new(
            &device,
            format,
            None,
            gpu::SHADER_LIGHT_RENDER,
            layouts,
            true,
            true,
        );

        Self { pipeline }
    }

    // pub fn resize(&self) {}

    pub fn render(
        &self,
        render_pass: &mut wgpu::RenderPass,
        mesh: &gpu::MeshBuffer,
        bindings: &super::Bindings,
    ) {
        // Render a cube to view position of the light.
        // We re-use RenderPass of main rendering.
        render_pass.set_pipeline(&self.pipeline.inner);

        bindings.all(render_pass);

        mesh.render(render_pass);
    }
}
