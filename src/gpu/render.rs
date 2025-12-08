pub fn stencil_state(format_depth: Option<wgpu::TextureFormat>) -> Option<wgpu::DepthStencilState> {
    format_depth.map(|format| {
        let stencil = match format {
            super::texture::Texture::DEPTH_FORMAT => wgpu::StencilState::default(),
            super::texture::Texture::DEPTH_AND_STENCIL_FORMAT => {
                let stencil_state = wgpu::StencilFaceState {
                    compare: wgpu::CompareFunction::Always,
                    fail_op: wgpu::StencilOperation::Keep,
                    depth_fail_op: wgpu::StencilOperation::Keep,
                    pass_op: wgpu::StencilOperation::IncrementClamp,
                };
                wgpu::StencilState {
                    front: stencil_state,
                    back: stencil_state,
                    read_mask: 0xFF,
                    write_mask: 0xFF,
                }
            }
            _ => unimplemented!(),
        };

        wgpu::DepthStencilState {
            format,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::LessEqual,
            bias: wgpu::DepthBiasState::default(),
            stencil,
        }
    })
}

pub fn create_render_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    format_color: wgpu::TextureFormat,
    format_depth: Option<wgpu::TextureFormat>,
    vertex_layouts: &[wgpu::VertexBufferLayout],
    shader: wgpu::ShaderModuleDescriptor,
    topology: wgpu::PrimitiveTopology,
    enable_back_face: bool,
    wireframe: bool,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(shader);
    let polygon_mode = wireframe
        .then(|| wgpu::PolygonMode::Line)
        .unwrap_or_else(|| wgpu::PolygonMode::Fill);

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: vertex_layouts,
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: format_color,
                blend: Some(wgpu::BlendState {
                    alpha: wgpu::BlendComponent::REPLACE,
                    color: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: (!enable_back_face).then(|| wgpu::Face::Back),
            polygon_mode,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: stencil_state(format_depth),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    })
}
