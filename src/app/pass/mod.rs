pub mod depth;
pub mod light_cube;
pub mod render;
pub mod shadow;

pub struct Passes {
    pub render: render::Pass,
    pub depth: depth::Pass,
    pub light_cube: light_cube::Pass,
    pub shadow: shadow::Pass,

    pub bindings: Bindings,
}

impl Passes {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        config: &crate::app::config::Config,
        layouts: &[Option<&wgpu::BindGroupLayout>],
        bindings: Bindings,
    ) -> Self {
        Self {
            render: render::Pass::new(device, format, config, layouts),
            depth: depth::Pass::new(device, config.width, config.height, format),
            light_cube: light_cube::Pass::new(device, format, config, layouts),
            shadow: shadow::Pass::new(device, config.width, config.height, layouts),

            bindings,
        }
    }

    pub fn render(
        &mut self,
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        meshes: &[super::gpu::MeshBuffer],
        config: &crate::app::config::Config,
    ) {
        self.shadow.render(encoder, meshes, &self.bindings);

        self.render.render(
            view,
            encoder,
            &self.depth.texture.view,
            &mut self.light_cube,
            &mut self.shadow,
            meshes,
            &self.bindings,
            config,
        );

        if config.debug_depth_show {
            self.depth.render(view, encoder);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Bindings {
    pub globals: wgpu::BindGroup,
    pub camera: wgpu::BindGroup,
    pub light: wgpu::BindGroup,
    pub texture: wgpu::BindGroup,
}

impl Bindings {
    pub fn apply(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_bind_group(0, Some(&self.globals), &[]);
        render_pass.set_bind_group(1, Some(&self.camera), &[]);
        render_pass.set_bind_group(2, Some(&self.light), &[]);
        render_pass.set_bind_group(3, Some(&self.texture), &[]);
    }
}
