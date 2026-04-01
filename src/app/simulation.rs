use std::{cell::RefCell, rc::Rc};

use crate::Vec3;

#[derive(Debug)]
pub struct Simulation {
    pub state: State,
    pub bodies: Vec<Rc<RefCell<crate::app::body::Body>>>,
    pub camera: crate::app::camera::Camera,
    pub sun: Vec3,
}

impl Simulation {
    pub fn new() -> Self {
        Self {
            state: State::new(),
            bodies: vec![],
            camera: crate::app::camera::Camera::new(),
            sun: Vec3::new(-100.0, 0.0, 0.0),
        }
    }

    /*
    pub fn add_body(
        &mut self,
        mesh: Option<crate::mesh::Mesh>,
        instance: Option<super::gpu::InstanceInput>,
        mat: Option<Mat4>,
        color: Option<wgpu::Color>,
        color_mode: Option<u32>,
        entity: Option<crate::entity::Body>,
        body: Option<super::body::Body>,
    ) {
        self.bodies
            .push(Rc::new(RefCell::new(if let Some(body) = body {
                body
            } else {
                let instance = instance.unwrap_or({
                    let mut instance = super::gpu::InstanceInput::default();

                    if let Some(mat) = mat {
                        instance.mat = mat;
                    }

                    if let Some(color) = color {
                        instance.color = super::gpu::color_vec3(&color);
                    }

                    if let Some(color_mode) = color_mode {
                        instance.color_mode = color_mode;
                    }

                    instance
                });

                let mut body = crate::app::body::Body::new();
                body.mesh = mesh.map(|m| Rc::new(RefCell::new(m)));
                body.instance = instance;
                body.entity = entity;
                body
            })));
    }
    */

    pub fn update(&mut self) {
        if self.state.is_paused {
            return;
        }

        self.state.iteration += 1;
    }
}

#[derive(Clone, Debug)]
pub struct State {
    pub iteration: usize,
    pub is_paused: bool,
    pub pause_at: Option<usize>,
}

impl State {
    pub fn new() -> Self {
        Self {
            iteration: 0,
            is_paused: false,
            pause_at: None,
        }
    }
}
