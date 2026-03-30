use std::sync::Arc;

use crate::Float;

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Globals {
    // 0: flat
    // 1: texture
    // 2: diffuse lighting
    pub color_mode: u32,

    pub extra: u32,
}

pub struct Window {
    pub window: Arc<winit::window::Window>,
    pub instance: wgpu::Instance,
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub is_surface_configured: bool,

    pub pipelines: super::gpu::Pipelines,
    pub texture: super::gpu::Texture,
    pub meshes: Vec<super::gpu::MeshBuffer>,
    pub camera: super::gpu::UniformBuffer<glam::Mat4>,
    pub globals: super::gpu::UniformBuffer<Globals>,
    pub depth: super::depth::DepthPass,
}

impl Window {
    pub async fn new(
        display: winit::event_loop::OwnedDisplayHandle,
        window: Arc<winit::window::Window>,
        config: &crate::app::config::Config,
        simulation: &crate::app::simulation::Simulation,
    ) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_with_display_handle(
            Box::new(display),
        ));
        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        // The adapted above isn't guaranteed to work on all devices.
        // In such case, use the adapter auto selection below.
        // let adapter = instance
        //     .enumerate_adapters(wgpu::Backends::all())
        //     .await.iter()
        //     .filter(|adapter| {
        //         adapter.is_surface_supported(&surface)
        //     })
        //     .next()
        //     .unwrap();

        let features_wgpu = wgpu::FeaturesWGPU::empty();
        // features_wgpu.insert(wgpu::FeaturesWGPU::POLYGON_MODE_LINE);

        let features_webgpu = wgpu::FeaturesWebGPU::empty();
        // features_webgpu.insert(wgpu::FeaturesWebGPU::DEPTH32FLOAT_STENCIL8);

        // Features::NON_FILL_POLYGON_MODE
        // Features::POLYGON_MODE_LINE
        // Features::POLYGON_MODE_POINT
        // Features::DEPTH_CLIP_CONTROL
        // Requires Features::CONSERVATIVE_RASTERIZATION

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                required_features: wgpu::Features {
                    features_wgpu,
                    features_webgpu,
                },
                ..Default::default()
            })
            .await
            .unwrap();

        let caps = surface.get_capabilities(&adapter);

        let format = caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(caps.formats[0]);

        let size = window.inner_size();

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: format,
            width: size.width,
            height: size.height,
            present_mode: caps.present_modes[0],
            desired_maximum_frame_latency: 2,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
        };

        // List of supported configurations by the adapter, device, surface.
        if config.debug_window {
            println!("[WINDOW] adapter features: {}", adapter.features());
            println!("[WINDOW] device features: {}", device.features());
            println!(
                "[WINDOW] surface capabilities present modes: {:?}",
                caps.present_modes
            );
        }

        let texture = super::gpu::Texture::new_image_from_bytes(
            &device,
            &queue,
            include_bytes!("../../res/happy-tree.png"),
        );

        let mut meshes = vec![];
        for body in simulation.bodies.iter().map(|b| b.borrow()) {
            if let Some(mesh) = body.mesh.as_ref() {
                let mesh = mesh.borrow();

                if config.debug_window_mesh {
                    for v in &mesh.vertices {
                        println!("v: {}", v.pos);
                    }
                    println!("indices: {:?}", &mesh.indices);
                    println!("mat: {}", body.mat);
                }

                meshes.push(super::gpu::MeshBuffer::new(
                    &device,
                    &mesh.vertices,
                    &mesh.indices,
                    body.mat,
                ))
            }
        }

        let globals = Globals {
            color_mode: config.shader_color_mode,
            extra: config.shader_extra,
        };
        let globals = super::gpu::UniformBuffer::new(&device, globals);

        let camera_buffer = super::gpu::UniformBuffer::new(
            &device,
            simulation
                .camera
                .view_proj(size.width as Float / size.height as Float)
                .unwrap(),
        );

        let pipeline = super::gpu::RenderPipeline::new(
            &device,
            surface_config.format,
            config.enable_back_face,
            super::gpu::SHADER_MESH_MAT,
            &[
                Some(&texture.bind.as_ref().unwrap().layout),
                Some(&camera_buffer.layout),
                Some(&globals.layout),
            ],
            true,
        );

        let pipelines = super::gpu::Pipelines {
            main: pipeline,
            more: vec![],
        };

        // let depth = super::gpu::Texture::create_depth_texture_with_comparison_sampler(
        //     &device,
        //     size.width,
        //     size.height,
        // );

        let depth = super::depth::DepthPass::new(&device, &surface_config);

        Self {
            window,
            instance,
            surface,
            device,
            queue,
            surface_config,
            is_surface_configured: false,

            pipelines,
            texture,
            meshes,
            camera: camera_buffer,
            globals,
            depth,
        }
    }

    pub fn get_window(&self) -> &winit::window::Window {
        &self.window
    }

    pub fn configure_surface(&self) {
        // todo
    }

    pub fn center_cursor(&self) {
        let width = self.surface_config.width;
        let height = self.surface_config.height;
        let mid = (width / 2, height / 2);
        self.window
            .set_cursor_position(winit::dpi::PhysicalPosition::new(mid.0, mid.1))
            .unwrap();
    }

    pub fn reset_cursor(&self) {
        self.center_cursor();
        self.window.set_cursor_visible(true);
        self.window
            .set_cursor_grab(winit::window::CursorGrabMode::None)
            .unwrap();
    }

    pub fn resize(&mut self, width: u32, height: u32, config: &crate::app::config::Config) {
        self.surface_config.width = width;
        self.surface_config.height = height;
        self.surface.configure(&self.device, &self.surface_config);

        // self.depth = super::gpu::Texture::create_depth_texture_with_comparison_sampler(
        //     &self.device,
        //     size.width,
        //     size.height,
        // );

        self.depth.resize(&self.device, width, height);

        let is_surface_configured = self.is_surface_configured;
        self.is_surface_configured = true;
        if !is_surface_configured && self.is_surface_configured {
            if config.debug_window {
                println!("[WINDOW] surface is now configured")
            }
        }
    }

    pub fn update(&mut self, simulation: &crate::app::simulation::Simulation) {
        let width = self.surface_config.width;
        let height = self.surface_config.height;
        self.queue.write_buffer(
            &self.camera.inner,
            0,
            bytemuck::cast_slice(&[simulation
                .camera
                .view_proj(width as Float / height as Float)
                .unwrap()]),
        );
    }

    pub fn render(&mut self, config: &crate::app::config::Config) {
        self.window.request_redraw();

        if !self.is_surface_configured {
            if config.debug_window {
                println!("[WINDOW] surface is not configured yet")
            }
            return;
        }

        let texture = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture) => texture,
            wgpu::CurrentSurfaceTexture::Occluded | wgpu::CurrentSurfaceTexture::Timeout => return,
            wgpu::CurrentSurfaceTexture::Suboptimal(_) | wgpu::CurrentSurfaceTexture::Outdated => {
                if config.debug_window {
                    println!(
                        "[WINDOW] surface texture is suboptimal or outdated, need to reconfigure"
                    )
                }
                self.configure_surface();
                return;
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                unreachable!("No error scope registered, so validation errors will panic")
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                if config.debug_window {
                    println!("[WINDOW] surface texture has been lost, need to recreate")
                }
                self.surface = self.instance.create_surface(self.window.clone()).unwrap();
                self.configure_surface();
                return;
            }
        };

        let view = texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        // render pass in {} for mut borrow encoder to release before calling finish
        {
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
                    view: &self.depth.texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });

            render_pass.set_pipeline(&self.pipelines.main.inner);
            render_pass.set_bind_group(0, &self.texture.bind.as_ref().unwrap().bind_group, &[]);
            render_pass.set_bind_group(1, &self.globals.bind_group, &[]);
            render_pass.set_bind_group(2, &self.camera.bind_group, &[]);

            for mesh in &self.meshes {
                mesh.render(&mut render_pass);
            }
        }

        if config.debug_depth_show {
            self.depth.render(&view, &mut encoder);
        }

        self.queue.submit([encoder.finish()]);
        texture.present();
    }
}
