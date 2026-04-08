use std::sync::Arc;

use glam::Mat4;

use crate::Float;

pub struct Window {
    pub window: Arc<winit::window::Window>,
    pub instance: wgpu::Instance,
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub is_surface_configured: bool,

    // 0: white cube
    // 1..: loaded by user in app.simulation.bodies
    pub meshes: Vec<super::gpu::MeshBuffer>,

    pub uniforms: super::uniform::Uniforms,
    pub passes: super::pass::Passes,
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

        let mut meshes = vec![];

        // TODO: ADD COLOR PER MESH? ask GPT why model matrix is not a uniform
        // if has to stay like that then ignore color per mesh
        // render light cube

        meshes.push(super::gpu::MeshBuffer::new(
            &device,
            &crate::meshes::cube::VERTICES,
            &crate::meshes::cube::INDICES,
            &super::gpu::InstanceInput::default(),
            false,
        ));

        for body in &simulation.bodies {
            if let Some(mesh) = body.mesh.as_ref() {
                if config.debug_window_mesh {
                    for v in &mesh.vertices {
                        println!("v: {}", v.pos);
                    }
                    println!("indices: {:?}", &mesh.indices);
                    println!("mat: {:?}", body.mat);
                }

                let instance = super::gpu::InstanceInput::new(body.mat);

                meshes.push(super::gpu::MeshBuffer::new(
                    &device,
                    &mesh.vertices,
                    &mesh.indices,
                    &instance,
                    mesh.is_flat(),
                ));
            }
        }

        /*
        let texture = super::gpu::Texture::new_image_from_bytes(
            &device,
            &queue,
            include_bytes!("../../res/happy-tree.png"),
        );
        let textures = vec![texture];
        */

        let globals = super::gpu::UniformBuffer::new(
            &device,
            super::uniform::Globals {
                color: super::gpu::color_vec3(&config.global_color),
                color_mode: config.global_color_mode,

                ambient_strength: config.ambient_strength,

                shadow_resolution: config.shadow_resolution,
                shadow_bias_scale: config.shadow_bias_scale,
                shadow_bias_minimum: config.shadow_bias_minimum,
                shadow_normal_offset_scale: config.shadow_normal_offset_scale,
                shadow_pcf: config.shadow_pcf,

                extra: config.global_extra,
                ..Default::default()
            },
        );

        let camera = super::uniform::Camera {
            view_proj: simulation
                .camera
                .view_proj(size.width as Float / size.height as Float)
                .unwrap(),
        };

        let light = {
            let pos = {
                if let Some(d) = config.light_distance {
                    simulation.sun.normalize() * d
                } else {
                    simulation.sun
                }
            };

            let dir = (config.light_target - pos).normalize();
            let view = Mat4::look_to_rh(pos, dir, config.light_up);

            let proj = Mat4::orthographic_rh(
                -config.light_side,
                config.light_side,
                -config.light_side,
                config.light_side,
                config.light_znear,
                config.light_zfar,
            );

            super::uniform::Light {
                view_proj: proj * view,
                pos,
                color: super::gpu::color_vec3(&config.light_color),
                ..Default::default()
            }
        };

        let view = super::gpu::UniformBuffer::new(&device, super::uniform::View { camera, light });

        let shadow = super::gpu::Texture::create_depth_texture_shadow_pass(
            &device,
            config.shadow_resolution,
            config.shadow_resolution,
        );

        let uniforms = super::uniform::Uniforms {
            globals,
            view,
            shadow,
        };

        let passes = super::pass::Passes::new(&device, surface_config.format, &config, &uniforms);

        Self {
            window,
            instance,
            surface,
            device,
            queue,
            surface_config,
            is_surface_configured: false,

            meshes,
            uniforms,
            passes,
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

        self.passes.depth.resize(&self.device, width, height);

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

        self.uniforms.view.uniform.camera.view_proj = simulation
            .camera
            .view_proj(width as Float / height as Float)
            .unwrap();

        self.queue.write_buffer(
            &self.uniforms.view.buffer,
            0,
            bytemuck::bytes_of(&self.uniforms.view.uniform),
        );

        // skip light cube
        for ii in 0..simulation.bodies.len() {
            let instance = super::gpu::InstanceInput::new(simulation.bodies[ii].mat);
            self.meshes[1 + ii].update_instance_buffer(&self.device, &instance);
        }

        // self.queue.write_buffer(
        //     &self.uniforms.view.buffer,
        //     0,
        //     bytemuck::bytes_of(&self.uniforms.view.uniform),
        // );
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

        self.passes.render(
            &mut encoder,
            &view,
            &self.uniforms.shadow,
            &self.meshes,
            config,
        );

        self.queue.submit([encoder.finish()]);
        texture.present();
    }
}
