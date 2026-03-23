pub struct Text {
    pub brush: wgpu_text::TextBrush,
    pub sections: Vec<wgpu_text::glyph_brush::OwnedSection>,
}

impl Text {
    #[allow(unused_variables)]
    pub fn new(
        font: &str,
        configs: &[super::config::ConfigText],
        device: &wgpu::Device,
        size: &winit::dpi::PhysicalSize<u32>,
        format: wgpu::TextureFormat,
    ) -> Self {
        let data = std::fs::read(font).unwrap();
        let font = wgpu_text::glyph_brush::ab_glyph::FontArc::try_from_vec(data).unwrap();

        let depth_stencil =
            super::render::stencil_state(Some(super::texture::Texture::DEPTH_FORMAT));

        let brush = wgpu_text::BrushBuilder::using_font(font)
            .with_depth_stencil(depth_stencil)
            .build(device, size.width, size.height, format);

        let sections = configs
            .iter()
            .map(|c| {
                let ha = match c.ha.as_str() {
                    "right" => wgpu_text::glyph_brush::HorizontalAlign::Right,
                    "left" => wgpu_text::glyph_brush::HorizontalAlign::Left,
                    "center" => wgpu_text::glyph_brush::HorizontalAlign::Center,
                    ha => panic!("Horizontal alignment {} not recognized, expecting one of: right, left, center.", ha),
                };

                wgpu_text::glyph_brush::OwnedSection::default()
                    .with_layout(wgpu_text::glyph_brush::Layout::default().h_align(ha))
                    .with_screen_position(c.pos)
                    .add_text(wgpu_text::glyph_brush::OwnedText::new(&c.text).with_color(c.color))
            })
            .collect();

        Self { brush, sections }
    }

    pub fn resize(&mut self, width: u32, height: u32, queue: &wgpu::Queue) {
        self.brush.resize_view(width as f32, height as f32, queue);
    }

    pub fn queue(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        self.brush.queue(&device, &queue, &self.sections).unwrap();
    }

    pub fn draw<'pass>(&'pass mut self, rpass: &mut wgpu::RenderPass<'pass>) {
        self.brush.draw(rpass);
    }
}

pub fn info(iteration: usize, fps: usize) -> Vec<wgpu_text::glyph_brush::OwnedSection> {
    let text = format!("kalast\niteration: {}\nfps: {}", iteration, fps);
    vec![
        wgpu_text::glyph_brush::OwnedSection::default()
            .with_layout(
                wgpu_text::glyph_brush::Layout::default()
                    .h_align(wgpu_text::glyph_brush::HorizontalAlign::Left),
            )
            .with_screen_position((0.0, 0.0))
            .add_text(
                wgpu_text::glyph_brush::OwnedText::new(text).with_color([1.0, 1.0, 1.0, 1.0]),
            ),
    ]
}
