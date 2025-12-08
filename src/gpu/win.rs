use numpy::PyArrayMethods;
use pyo3::{IntoPyObjectExt, prelude::*};
use wgpu::util::DeviceExt;
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, KeyEvent, MouseButton, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
    platform::run_on_demand::EventLoopExtRunOnDemand,
    window::{Window, WindowId},
};

use crate::{
    Float, PI, Vec3,
    gpu::{buffer::UniformBindTrait, light::DrawLight, model::DrawModel},
};

#[pyclass]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct StateStep {
    #[pyo3(get, set)]
    pub iteration: usize,

    #[pyo3(get, set)]
    pub dt: std::time::Duration,

    #[pyo3(get, set)]
    pub models_state: Vec<super::scene::ModelState>,

    pub light_pos: Vec3,
}

#[pymethods]
impl StateStep {
    // Getter numpy.ndarray read and write.
    #[getter]
    fn light_pos<'py>(slf: Bound<'py, Self>) -> Bound<'py, numpy::PyArray1<Float>> {
        let slice = &slf.borrow().light_pos;
        let slice2 = slice.as_ref();
        let arr = numpy::ndarray::ArrayView1::from(slice2);
        unsafe { numpy::PyArray1::borrow_from_array(&arr, slf.into_any()) }
    }

    // Setter numpy.ndarray to allow shorthand operators.
    #[setter]
    pub fn set_light_pos<'py>(&mut self, light_pos: Bound<'py, numpy::PyArray1<Float>>) {
        let light_pos = unsafe { light_pos.as_slice().unwrap() };
        self.light_pos.x = light_pos[0];
        self.light_pos.y = light_pos[1];
        self.light_pos.z = light_pos[2];
    }

    // Getter numpy.ndarray read and write.
    // #[getter]
    // fn models_state<'py>(slf: Bound<'py, Self>) -> &'py [super::scene::ModelState] {
    //     let arr = slf.borrow().models_state.as_slice();
    //     pyo3::types::PyList::new(slf.py(), arr)
    // }

    // #[getter]
    // fn models_state<'py>(slf: Bound<'py, Self>) -> Bound<'_, pyo3::types::PyList> {
    // fn models_state<'py>(slf: Bound<'py, Self>) -> Bound<'_, PyAny> {
    //     let arr = &slf.borrow().models_state;
    //     let slice = arr.as_slice();
    //     slice.into_bound_py_any(slf.py()).unwrap()
    // }

    // #[getter]
    // fn get_models_state<'py>(&self, py: Python<'py>) -> Vec<&'py super::scene::ModelState> {
    //     self.models_state.iter().map(|model| model).collect()
    // }

    // not working, still cloning state
    pub fn get_model_state<'py>(slf: Bound<'py, Self>, ii: usize) -> PyResult<Bound<'py, PyAny>> {
        slf.borrow_mut().models_state[ii].into_bound_py_any(slf.py())
    }

    // need to set it manually
    pub fn set_model_state(&mut self, ii: usize, model_state: super::scene::ModelState) {
        self.models_state[ii] = model_state;
    }
}

pub struct State<'win> {
    pub window: std::sync::Arc<Window>,
    pub surface: wgpu::Surface<'win>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config_surf: wgpu::SurfaceConfiguration,
    pub globals: super::buffer::UniformBind<super::config::UniformGlobal>,
    pub texts: Vec<super::text::Text>,
    // pub camera: super::scene::CameraFpsWGPU,
    pub camera: super::scene::Camera,
    pub projection: super::scene::Projection,
    pub camera_uniform: super::buffer::UniformBind<super::scene::CameraUniform>,
    pub controller: super::scene::Controller,
    pub light: super::buffer::UniformBind<super::light::LightUniform>,
    pub depthpass: DepthPass,
    pub cube: super::model::Model,
    pub models: Vec<super::model::Model>,
    pub pipeline_hdr: super::hdr::HdrPipeline,
    pub environment_bind_group: wgpu::BindGroup,
    pub pipeline: wgpu::RenderPipeline,
    pub pipeline_light: wgpu::RenderPipeline,
    pub pipeline_sky: wgpu::RenderPipeline,
    pub debug_material: super::model::Material,
    pub is_paused: bool,
    pub config: super::config::Config,
    pub is_surface_configured: bool,
    pub is_mouse_pressed: bool,
    pub iteration: usize,
    pub fps_counter: super::FpsCounter,
}

impl State<'_> {
    pub async fn new(window: std::sync::Arc<Window>, config: super::config::Config) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let mut features_wgpu = wgpu::FeaturesWGPU::empty();
        features_wgpu.insert(wgpu::FeaturesWGPU::POLYGON_MODE_LINE);

        let mut features_webgpu = wgpu::FeaturesWebGPU::empty();
        features_webgpu.insert(wgpu::FeaturesWebGPU::DEPTH32FLOAT_STENCIL8);

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

        if config.debug_state_creation {
            println!("device created OK");
        }

        let caps = surface.get_capabilities(&adapter);
        let format = caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(caps.formats[0]);
        let config_surf = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: format,
            width: size.width,
            height: size.height,
            present_mode: caps.present_modes[0],
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![format.add_srgb_suffix()],
            desired_maximum_frame_latency: 2,
        };

        let globals = super::config::UniformGlobal {
            test: config.global_test,
            ambient_strength: config.ambient_strength,
            diffuse_enable: config.diffuse_enable as u32,
            specular_enable: config.specular_enable as u32,
            ..Default::default()
        }
        .register(&device, 0);

        let text_user = super::text::Text::new(&config.font, &config.texts, &device, &config_surf);

        let config_texts_info = vec![super::config::ConfigText {
            text: "kalast".to_string(),
            ..Default::default()
        }];
        let text_info =
            super::text::Text::new(&config.font, &config_texts_info, &device, &config_surf);
        let texts = vec![text_info, text_user];

        // let camera = super::scene::CameraFpsWGPU::new(
        //     config.camera_pos,
        //     config.camera_yaw * std::f32::consts::PI / 180.0,
        //     config.camera_pitch * std::f32::consts::PI / 180.0,
        // );

        let camera = super::scene::Camera {
            pos: config.camera_pos,
            dir: config.camera_dir,
            up: config.camera_up,
            anchor: config.camera_anchor,
            typ: config.camera_type,
            up_world: config.up_world,
        };
        let projection = super::scene::Projection::new(
            config_surf.width,
            config_surf.height,
            config.camera_fovy * PI / 180.0,
            config.camera_znear,
            config.camera_zfar,
            config.camera_projection,
        );
        let camera_uniform =
            super::scene::CameraUniform::new(&camera, &projection).register(&device, 0);
        let controller =
            super::scene::Controller::new(config.camera_speed, config.camera_sensitivity);

        let light = super::light::LightUniform {
            position: config.light_pos.into(),
            _padding: 0,
            color: config.light_color.into(),
            _padding2: 0,
        }
        .register(&device, 0);

        if config.debug_state_creation {
            println!("camera/light created OK");
        }

        let texture_bind_group_layout = super::texture::create_layout(
            &device,
            true,
            wgpu::TextureViewDimension::D2,
            wgpu::SamplerBindingType::Filtering,
            2,
        );

        if config.debug_state_creation {
            println!("texture created OK");
        }

        let depthpass = DepthPass::new(&device, &config_surf, &config);

        if config.debug_state_creation {
            println!("depthpass created OK");
        }

        let config_cube = super::config::ConfigModel {
            path: "res/cube.obj".to_string(),
            pos_factor: Vec3::new(0.1, 0.1, 0.1),
            ..Default::default()
        };
        let mut cube = super::model::Model::load(
            &config_cube,
            &device,
            &queue,
            &texture_bind_group_layout,
            super::scene::ModelState::new(),
        );
        cube.meshes[0]
            .inner
            .update_colors(1, [1.0, 1.0, 1.0].into());

        let models = config
            .models
            .iter()
            .map(|config_model| {
                super::model::Model::load(
                    config_model,
                    &device,
                    &queue,
                    &texture_bind_group_layout,
                    super::scene::ModelState::new(),
                )
            })
            .collect();

        if config.debug_state_creation {
            println!("models/instance created OK");
        }

        let pipeline_hdr = super::hdr::HdrPipeline::new(
            &device,
            &config_surf,
            config.enable_back_face,
            config.wireframe,
        );

        if config.debug_state_creation {
            println!("pipeline hdr created OK");
        }

        let hdr_loader = super::hdr::HdrLoader::new(&device);
        let sky_bytes = std::fs::read("res/pure-sky.hdr").unwrap();
        let sky_texture = hdr_loader
            .from_equirectangular_bytes(&device, &queue, &sky_bytes, 1080)
            .unwrap();
        let environment_layout = super::texture::create_layout(
            &device,
            false,
            wgpu::TextureViewDimension::Cube,
            wgpu::SamplerBindingType::NonFiltering,
            1,
        );
        let environment_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &environment_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&sky_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sky_texture.sampler),
                },
            ],
        });

        if config.debug_state_creation {
            println!("environment sky created OK");
        }

        let format_render = if config.hdr_enable {
            pipeline_hdr.format
        } else {
            format
        };

        let pipeline = {
            let shader = wgpu::include_wgsl!("../../shaders/shader.wgsl");

            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[
                    &globals.layout,
                    &camera_uniform.layout,
                    &light.layout,
                    &texture_bind_group_layout,
                ],
                ..Default::default()
            });
            super::render::create_render_pipeline(
                &device,
                &layout,
                format_render,
                //Some(super::texture::Texture::DEPTH_AND_STENCIL_FORMAT),
                Some(super::texture::Texture::DEPTH_FORMAT),
                &[
                    super::model::Vertex::descriptor(),
                    super::scene::ModelState::descriptor(),
                ],
                shader,
                wgpu::PrimitiveTopology::TriangleList,
                config.enable_back_face,
                config.wireframe,
            )
        };

        if config.debug_state_creation {
            println!("pipeline main created OK");
        }

        let pipeline_light = {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&camera_uniform.layout, &light.layout],
                push_constant_ranges: &[],
            });
            let shader = wgpu::include_wgsl!("../../shaders/light.wgsl");
            super::render::create_render_pipeline(
                &device,
                &layout,
                format_render,
                Some(super::texture::Texture::DEPTH_FORMAT),
                &[crate::mesh::Vertex::descriptor()],
                shader,
                wgpu::PrimitiveTopology::TriangleList,
                config.enable_back_face,
                config.wireframe,
            )
        };

        if config.debug_state_creation {
            println!("pipeline light created OK");
        }

        let pipeline_sky = {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Sky Pipeline Layout"),
                bind_group_layouts: &[&camera_uniform.layout, &environment_layout],
                push_constant_ranges: &[],
            });
            let shader = wgpu::include_wgsl!("../../shaders/sky.wgsl");
            super::render::create_render_pipeline(
                &device,
                &layout,
                format_render,
                Some(super::texture::Texture::DEPTH_FORMAT),
                &[],
                shader,
                wgpu::PrimitiveTopology::TriangleList,
                config.enable_back_face,
                config.wireframe,
            )
        };

        if config.debug_state_creation {
            println!("pipeline sky created OK");
        }

        let debug_material = {
            let diffuse_bytes = include_bytes!("../../res/cobble-diffuse.png");
            let normal_bytes = include_bytes!("../../res/cobble-normal.png");

            let diffuse_texture =
                super::texture::Texture::from_bytes(diffuse_bytes, &device, &queue, false).unwrap();
            let normal_texture =
                super::texture::Texture::from_bytes(normal_bytes, &device, &queue, true).unwrap();

            super::model::Material::new(
                diffuse_texture,
                normal_texture,
                &device,
                &texture_bind_group_layout,
            )
        };

        if config.start_paused {
            println!("Paused at start")
        }

        if config.debug_state_creation {
            println!("state created OK");
        }

        let state = State {
            window,
            surface,
            device,
            queue,
            config_surf,
            depthpass,
            texts,
            camera,
            projection,
            camera_uniform,
            controller,
            cube,
            models,
            light,
            globals,
            pipeline_hdr,
            environment_bind_group,
            pipeline,
            pipeline_light,
            pipeline_sky,
            debug_material,
            is_paused: config.start_paused,
            is_surface_configured: false,
            is_mouse_pressed: false,
            config,
            iteration: 0,
            fps_counter: super::FpsCounter::new(),
        };

        state
    }

    pub fn get_window(&self) -> &Window {
        &self.window
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.config_surf.width = width;
        self.config_surf.height = height;
        self.projection.resize(width, height);
        self.pipeline_hdr.resize(&self.device, width, height);
        self.surface.configure(&self.device, &self.config_surf);
        self.depthpass.resize(&self.device, width, height);

        for text in &mut self.texts {
            text.resize(width, height, &self.queue);
        }

        self.is_surface_configured = true;
    }

    pub fn state_step(&self, dt: std::time::Duration) -> StateStep {
        StateStep {
            iteration: self.iteration,
            dt,
            models_state: self.models.iter().map(|model| model.state).collect(),
            light_pos: self.light.uniform.position.into(),
        }
    }

    // pub fn update<F>(&mut self, dt: std::time::Duration, closure_update_state: F)
    pub fn update(&mut self, dt: std::time::Duration)
    // where
    //     F: Fn(StateStep) -> Option<StateStep>,
    {
        self.camera.update_with_controller(&mut self.controller, dt);
        self.camera_uniform
            .uniform
            .update_view_proj(&self.camera, &self.projection);
        self.queue.write_buffer(
            &self.camera_uniform.buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform.uniform]),
        );

        let fps = self.fps_counter.update(dt, self.config.fps_time_refresh);
        self.texts[0].sections = super::text::info(self.iteration, fps);

        // Updates before are not frozen when state is paused, all updates after this are frozen.
        if self.is_paused && self.iteration > 0 {
            return;
        }

        // self.update_state_closure(closure_update_state, self.state_step(dt));
        self.iteration += 1;
    }

    pub fn update_state_closure<F>(&mut self, closure_update_state: F, state_step: StateStep)
    where
        F: Fn(StateStep) -> Option<StateStep>,
    {
        if self.config.debug_state_closure {
            println!("{:?}", state_step);
        }

        if let Some(state) = (closure_update_state)(state_step) {
            for iib in 0..self.models.len() {
                self.models[iib].state.p = state.models_state[iib].p;
                self.models[iib].state.m = state.models_state[iib].m;
                self.models[iib].update_buffer(&self.device);
            }

            self.light.uniform.position = state.light_pos.into();
            self.light.write(&self.queue);
        }
    }

    pub fn render(&mut self) {
        let texture = self
            .surface
            .get_current_texture()
            .expect("failed to acquire next swapchain texture");

        let view = texture.texture.create_view(&wgpu::TextureViewDescriptor {
            format: Some(self.config_surf.format.add_srgb_suffix()),
            ..Default::default()
        });

        if self.config.debug_state_rendering {
            println!("state render: texture and view acquired OK")
        }

        let mut encoder = self.device.create_command_encoder(&Default::default());

        if self.config.debug_state_rendering {
            println!("state render: encoder created OK")
        }

        for (iit, text) in self.texts.iter_mut().enumerate() {
            if iit == 0 && !self.config.show_text_info {
                continue;
            }
            text.queue(&self.device, &self.queue);
        }

        {
            let view_render = if self.config.hdr_enable {
                self.pipeline_hdr.view()
            } else {
                &view
            };

            let stencil_ops = None;
            // let stencil_ops = Some(wgpu::Operations {
            //     load: wgpu::LoadOp::Clear(0),
            //     store: wgpu::StoreOp::Store,
            // });

            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: view_render,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.config.background),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depthpass.texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops,
                }),
                ..Default::default()
            });

            if self.config.debug_state_rendering {
                println!("state render: encoder render pass began OK")
            }

            rpass.set_vertex_buffer(1, self.models[0].state_buffer.slice(..));

            if self.config.render_light {
                rpass.set_pipeline(&self.pipeline_light);
                rpass.draw_light_model(
                    &self.cube,
                    &self.camera_uniform.bind_group,
                    &self.light.bind_group,
                );
            }

            rpass.set_pipeline(&self.pipeline);

            /*
            rpass.draw_model_instanced(
                &self.model,
                0..self.instances.len() as u32,
                &self.camera_bind_group,
                &self.light_bind_group,
            );
            */

            for model in &self.models {
                rpass.draw_model_instanced_with_material(
                    model,
                    &self.debug_material,
                    0..1 as u32,
                    &self.globals.bind_group,
                    &self.camera_uniform.bind_group,
                    &self.light.bind_group,
                );
            }

            if self.config.debug_state_rendering {
                println!("state render: all models rendered OK")
            }

            for (iit, text) in self.texts.iter_mut().enumerate() {
                if iit == 0 && !self.config.show_text_info {
                    continue;
                }
                text.draw(&mut rpass);
            }

            // If you wanted to call any drawing commands, they would go here.
            // here
        }

        if self.config.debug_state_rendering {
            println!("state render: all OK")
        }

        // or here dunno

        if self.config.hdr_enable {
            self.pipeline_hdr.process(&mut encoder, &view);
        }

        if self.config.depthpass_enable {
            self.depthpass.render(&view, &mut encoder);
        }

        if self.config.debug_state_rendering {
            println!("state render: ready to submit..")
        }

        self.queue.submit([encoder.finish()]);

        if self.config.debug_state_rendering {
            println!("state render: encoder submitted to queue OK")
        }

        self.window.pre_present_notify();
        texture.present();

        if self.config.debug_state_rendering {
            println!("state render: texture getting presented OK")
        }
    }

    pub fn handle_mouse_moved_clear_color(&mut self, x: f64, y: f64) {
        self.config.background.r = x / self.config.width as f64;
        self.config.background.g = y / self.config.height as f64;
    }

    pub fn handle_mouse_button(&mut self, button: MouseButton, is_pressed: bool) {
        match button {
            MouseButton::Left => {
                self.is_mouse_pressed = is_pressed;
            }
            _ => {}
        }
    }

    pub fn handle_key(&mut self, key: KeyCode, is_pressed: bool) {
        if !self.controller.handle_key(key, is_pressed) {
            match (key, is_pressed) {
                (KeyCode::Escape, true) => {
                    println!("key escape is pressed");
                }
                (KeyCode::KeyP, true) => {
                    self.is_paused = !self.is_paused;
                    println!(
                        "pause toggled: {}",
                        crate::util::bool_to_on_off(self.is_paused)
                    );
                }
                (KeyCode::KeyT, true) => {
                    // switch camera type
                    self.camera.toggle_type();
                    println!("Changed camera type, now is {:?}", self.camera.typ);

                    match self.camera.typ {
                        super::scene::CameraType::Arcball => {
                            self.window.set_cursor_visible(true);

                            let mid = (self.config_surf.width / 2, self.config_surf.height / 2);
                            self.window
                                .set_cursor_position(winit::dpi::PhysicalPosition::new(
                                    mid.0, mid.1,
                                ))
                                .unwrap();
                        }
                        super::scene::CameraType::WASD => {
                            self.window.set_cursor_visible(false);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

// #[pymethods]
// impl State {
//     pub fn update_closure<'py>(slf: Bound<'py, Self>) {}
// }

pub struct DepthPass {
    pub texture: super::texture::Texture,
    pub layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_depth_indices: u32,
    pub pipeline: wgpu::RenderPipeline,
}

impl DepthPass {
    pub fn new(
        device: &wgpu::Device,
        config_surf: &wgpu::SurfaceConfiguration,
        config: &super::config::Config,
    ) -> Self {
        let texture = super::texture::Texture::create_depth_texture_non_comparison_sampler(
            device,
            config_surf.width,
            config_surf.height,
        );

        if config.debug_state_creation {
            println!("depthpass: texture created OK");
        }

        let layout = super::texture::create_layout(
            device,
            false,
            wgpu::TextureViewDimension::D2,
            wgpu::SamplerBindingType::NonFiltering,
            1,
        );

        if config.debug_state_creation {
            println!("depthpass: layout created OK");
        }

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
            label: None,
        });

        if config.debug_state_creation {
            println!("depthpass: bind group created OK");
        }

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&super::buffer::DEPTH_VERTICES_QUARTER_TOP_RIGHT),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(super::buffer::DEPTH_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        if config.debug_state_creation {
            println!("depthpass: buffers created OK");
        }

        let pipeline = {
            let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&layout],
                push_constant_ranges: &[],
            });
            let shader = wgpu::include_wgsl!("../../shaders/view_depth.wgsl");
            super::render::create_render_pipeline(
                &device,
                &layout,
                config_surf.format,
                None,
                &[crate::mesh::Vertex::descriptor()],
                shader,
                wgpu::PrimitiveTopology::TriangleList,
                config.enable_back_face,
                config.wireframe,
            )
        };

        if config.debug_state_creation {
            println!("depthpass: pipeline created OK");
        }

        Self {
            texture,
            layout,
            bind_group,
            vertex_buffer,
            index_buffer,
            num_depth_indices: super::buffer::DEPTH_INDICES.len() as u32,
            pipeline,
        }
    }

    pub fn resize(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        self.texture = super::texture::Texture::create_depth_texture_non_comparison_sampler(
            device, width, height,
        );
        self.bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.texture.sampler),
                },
            ],
            label: None,
        });
    }

    pub fn render(&self, view: &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_depth_indices, 0, 0..1);
    }
}

// #[pyclass]
// #[pyclass(unsendable)]
// pub struct App<F, F2> {
// pub struct App<F> {
pub struct App<'win> {
    pub state: Option<State<'win>>,
    pub last_time: std::time::Instant,
    pub resumed: usize,

    // pub closure_update_state: fn(&mut State),
    // pub closure_update_state: Box<dyn FnMut(PyRefMut<State>)>,
    // pub closure_update_state_full: F2,

    // App is create in App::new() with a given Config.
    // At creation of State, the config is transfered from App to State.
    // For that, we store config in private and use self._config.take() for transfer.
    // Config can be accessed by App with App::config() by reading value in State.
    _config: Option<super::config::Config>,
}

// impl<F, F2> App<F, F2> {
// impl<F> App<F> {
impl App<'_> {
    pub fn new(
        config: super::config::Config,
        // closure_update_state: Box<dyn FnMut(PyRefMut<State>)>,
        // closure_update_state_full: F2,
    ) -> Self {
        Self {
            state: None,
            last_time: std::time::Instant::now(),
            resumed: 0,
            // closure_update_state,
            // closure_update_state_full,
            _config: Some(config),
        }
    }

    pub fn config(&self) -> &super::config::Config {
        self.state
            .as_ref()
            .map(|state| &state.config)
            .or(self._config.as_ref())
            .as_ref()
            .expect("No config found on neither State or App")
    }

    pub fn run_blocked(&mut self, ev: winit::event_loop::EventLoop<()>) {
        if self.resumed > 0 {
            panic!("App has already been run once before and cannot be ran on blocked now.")
        }

        ev.run_app(self).unwrap();
    }

    pub fn run_once(&mut self, ev: &mut winit::event_loop::EventLoop<()>) {
        ev.run_app_on_demand(self).unwrap();
    }
}

// impl<F, F2> ApplicationHandler for App<F, F2>
// impl<F> ApplicationHandler for App<F>
impl ApplicationHandler for App<'_>
// where
// F: Fn(StateStep) -> Option<StateStep>,
// F2: Fn(&mut State),
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.resumed += 1;
        if let Some(config) = self._config.take() {
            println!("resumed config taken");
            let attrs = winit::window::WindowAttributes::default()
                .with_inner_size(winit::dpi::PhysicalSize::new(config.width, config.height));

            let window = std::sync::Arc::new(event_loop.create_window(attrs).unwrap());
            self.state = Some(pollster::block_on(State::new(window.clone(), config)));
            window.request_redraw();
        } else {
            println!("resumed");
            self.state.as_ref().unwrap().window.request_redraw();
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        let state = self.state.as_mut().unwrap();
        if state.config.debug_event_device {
            println!("device event: {:?}", event);
        }
        match event {
            DeviceEvent::MouseMotion { delta: (dx, dy) } => {
                if state.camera.typ == super::scene::CameraType::WASD {
                    state.controller.mouse_motion(dx as Float, dy as Float);
                }
                // else if state.is_mouse_pressed {
                // state.controller.handle_mouse(dx, dy);
                //}
            }

            DeviceEvent::MouseWheel { delta } => {
                let (dx, dy) = match delta {
                    winit::event::MouseScrollDelta::LineDelta(dx, dy) => {
                        (dx as Float * 100.0, dy as Float * 100.0)
                    }
                    winit::event::MouseScrollDelta::PixelDelta(winit::dpi::PhysicalPosition {
                        x,
                        y,
                    }) => (x as Float, y as Float),
                };

                if state.camera.typ == super::scene::CameraType::Arcball {
                    state.controller.mouse_motion(-dx, -dy);
                }
            }
            _ => {}
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _id: WindowId,
        event: WindowEvent,
    ) {
        let state = self.state.as_mut().unwrap();

        if state.config.debug_event_window_except_redraw && event != WindowEvent::RedrawRequested {
            println!("win event: {:?}", event);
        }

        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // We can't render unless the surface is configured
                // maybe this should be put in state.render(), don't know
                if !state.is_surface_configured {
                    return;
                }

                let now = std::time::Instant::now();
                let dt = now - self.last_time;
                self.last_time = now;
                // state.update(dt, &self.closure_update_state);
                println!(
                    "going to update (iteration={},resumed={})",
                    state.iteration, self.resumed
                );
                state.update(dt);
                println!(
                    "update done (iteration={},resumed={})",
                    state.iteration, self.resumed
                );
                state.render();

                state.get_window().request_redraw();
                event_loop.exit();

                // if !self.has_been_shown_once {
                //     println!("exit first run loop on demand");
                //     self.has_been_shown_once = true;
                //     event_loop.exit();
                // } else {
                //     state.get_window().request_redraw();
                // }
            }
            WindowEvent::Resized(size) => {
                state.resize(size.width, size.height);
            }

            // not sure if useful
            // WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer }

            // window event cursor moved gives absolute current position, instead of delta moved, so not really useful
            //   for controller event or so, maybe useful in future
            // WindowEvent::CursorMoved { position, .. } => {}

            // does not give more info than device mouse wheel, and cursor delta moved already managed by device event
            // so we manage that there instead of here, I keep it commented here in case
            // WindowEvent::MouseWheel { delta, .. } => {
            //     state.handle_mouse_scroll(&delta);
            // }

            // pinch for zoom in arcball is only available as window event, no device event
            WindowEvent::PinchGesture { delta, .. } => {
                if state.camera.typ == super::scene::CameraType::Arcball {
                    state.controller.zoom(delta as Float);
                }
            }

            //
            WindowEvent::MouseInput {
                state: mouse_state,
                button,
                ..
            } => state.handle_mouse_button(button, mouse_state.is_pressed()),

            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => state.handle_key(code, key_state.is_pressed()),
            _ => (),
        };

        if state.camera.typ == super::scene::CameraType::WASD {
            let mid = (state.config_surf.width / 2, state.config_surf.height / 2);
            state
                .window
                .set_cursor_position(winit::dpi::PhysicalPosition::new(mid.0, mid.1))
                .unwrap();
        }
    }
}

/*
// pub fn run<F, F2>(
pub fn run<F>(
    config: super::config::Config,
    closure_update_state: F,
    // closure_update_state_full: F2,
) where
    F: Fn(StateStep) -> Option<StateStep>,
    // F2: Fn(&mut State),
{
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    // let mut app = App::new(config, closure_update_state, closure_update_state_full);
    let mut app = App::new(config, closure_update_state);
    event_loop.run_app(&mut app).unwrap();
}

#[pyfunction]
// #[pyo3(name = "run", signature = (config, closure_update_state, closure_update_state_full))]
#[pyo3(name = "run", signature = (config, closure_update_state))]
pub fn py_run<'py>(
    py: Python<'py>,
    config: super::config::Config,
    closure_update_state: Py<PyAny>,
    // closure_update_state_full: Py<PyAny>,
) {
    let closure_update_state = |state: StateStep| -> Option<StateStep> {
        closure_update_state
            .call1(py, (state,))
            .unwrap()
            .extract::<Option<StateStep>>(py)
            .unwrap()
    };

    // let closure_update_state_full = {
    //     |state: &mut State| {
    //         let a = state.closure_update_state_full.call1(py, (state,)).unwrap();
    //         ()
    //     }
    // };

    // run(config, closure_update_state, closure_update_state_full);
    run(config, closure_update_state);
}
*/

/*
// #[pyclass]
#[pyclass(unsendable)]
pub struct PyApp {
    // pub ev: std::sync::Arc<std::sync::Mutex<EventLoop<()>>>,
    pub app: Py<App>,
}

#[pymethods]
impl PyApp {
    // pub fn run<'py>(&mut self, py: Python<'py>) {
    //     let mut app_pyref: PyRefMut<App> = self.app.try_borrow_mut(py).unwrap();
    //     let app_ref: &mut App = &mut *app_pyref;

    //     let mut lock = self.ev.lock();
    //     let lock_unwrapped = lock.as_mut().unwrap();
    //     let ev: &mut EventLoop<()> = lock_unwrapped.borrow_mut();
    //     // ev.create_window(window_attributes);
    //     ev.run_app_on_demand(app_ref).unwrap();
    // }

    pub fn run<'py>(&mut self, py: Python<'py>) {
        let ev = EventLoop::new().unwrap();
        ev.set_control_flow(ControlFlow::Poll);

        let mut app_pyref: PyRefMut<App> = self.app.try_borrow_mut(py).unwrap();
        let app_ref: &mut App = &mut *app_pyref;
        ev.run_app(app_ref).unwrap();
    }
}
*/

/*
#[pyfunction]
#[pyo3(name = "create", signature = (config, closure))]
pub fn py_create<'py>(py: Python<'py>, config: super::config::Config, closure: Py<PyAny>) -> PyApp {
    // fn wrap(c: fn(&mut State), state: &mut State) {
    // fn wrap(c: fn()) {
    //     c()
    // }

    PyApp {
        // ev: std::sync::Arc::new(std::sync::Mutex::new(ev)),
        app: Py::new(py, {
            App::new(
                config,
                Box::new(|state: PyRefMut<State>| {
                    // let closure = |state: &mut State| {
                    closure.call1(py, (state,)).unwrap();
                    // .extract::<_>(py)
                    // .unwrap();
                    // .extract::<Option<StateStep>>(py)
                    // .unwrap()
                }),
            )
        })
        .unwrap(),
    }
}
*/
