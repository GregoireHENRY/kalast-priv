pub struct TextRendererMsdf<'a> {
    pub font: wgpu::BindGroupLayout,
    pub text: wgpu::BindGroupLayout,
    pub pipeline: wgpu::RenderPipeline,
    pub sampler: wgpu::Sampler,
    pub camera_buffer: wgpu::Buffer,
    pub render: wgpu::RenderBundleEncoderDescriptor<'a>,
    pub camera_array: [f32; 32],
}

impl<'a> TextRendererMsdf<'a> {
    #[allow(unused_variables, unreachable_code)]
    pub fn new(
        device: &wgpu::Device,
        format_color: wgpu::TextureFormat,
        format_depth: wgpu::TextureFormat,
    ) -> Self {
        let idk = 0;

        let render = wgpu::RenderBundleEncoderDescriptor {
            color_formats: &[Some(format_color)],
            depth_stencil: Some(wgpu::RenderBundleDepthStencil {
                format: format_depth,
                depth_read_only: true,
                stencil_read_only: true,
            }),
            sample_count: idk,
            multiview: None,
            ..Default::default()
        };

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Linear,
            anisotropy_clamp: 16,
            ..Default::default()
        });

        let camera_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: todo!(),
            size: todo!(),
            usage: todo!(),
            mapped_at_creation: todo!(),
        });

        Self {
            font: todo!(),
            text: todo!(),
            pipeline: todo!(),
            sampler,
            camera_buffer: todo!(),
            render: render,
            camera_array: todo!(),
        }
    }
}

/*
  constructor(
    public device: GPUDevice,
    colorFormat: GPUTextureFormat,
    depthFormat: GPUTextureFormat
  ) {
    this.renderBundleDescriptor = {
      colorFormats: [colorFormat],
      depthStencilFormat: depthFormat,
    };

    this.sampler = device.createSampler({
      label: 'MSDF text sampler',
      minFilter: 'linear',
      magFilter: 'linear',
      mipmapFilter: 'linear',
      maxAnisotropy: 16,
    });

    this.cameraUniformBuffer = device.createBuffer({
      label: 'MSDF camera uniform buffer',
      size: this.cameraArray.byteLength,
      usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.UNIFORM,
    });

    this.fontBindGroupLayout = device.createBindGroupLayout({
      label: 'MSDF font group layout',
      entries: [
        {
          binding: 0,
          visibility: GPUShaderStage.FRAGMENT,
          texture: {},
        },
        {
          binding: 1,
          visibility: GPUShaderStage.FRAGMENT,
          sampler: {},
        },
        {
          binding: 2,
          visibility: GPUShaderStage.VERTEX,
          buffer: { type: 'read-only-storage' },
        },
      ],
    });

    this.textBindGroupLayout = device.createBindGroupLayout({
      label: 'MSDF text group layout',
      entries: [
        {
          binding: 0,
          visibility: GPUShaderStage.VERTEX,
          buffer: {},
        },
        {
          binding: 1,
          visibility: GPUShaderStage.VERTEX | GPUShaderStage.FRAGMENT,
          buffer: { type: 'read-only-storage' },
        },
      ],
    });

    const shaderModule = device.createShaderModule({
      label: 'MSDF text shader',
      code: msdfTextWGSL,
    });

    this.pipelinePromise = device.createRenderPipelineAsync({
      label: `msdf text pipeline`,
      layout: device.createPipelineLayout({
        bindGroupLayouts: [this.fontBindGroupLayout, this.textBindGroupLayout],
      }),
      vertex: {
        module: shaderModule,
        entryPoint: 'vertexMain',
      },
      fragment: {
        module: shaderModule,
        entryPoint: 'fragmentMain',
        targets: [
          {
            format: colorFormat,
            blend: {
              color: {
                srcFactor: 'src-alpha',
                dstFactor: 'one-minus-src-alpha',
              },
              alpha: {
                srcFactor: 'one',
                dstFactor: 'one',
              },
            },
          },
        ],
      },
      primitive: {
        topology: 'triangle-strip',
        stripIndexFormat: 'uint32',
      },
      depthStencil: {
        depthWriteEnabled: false,
        depthCompare: 'less',
        format: depthFormat,
      },
    });
  }
*/
