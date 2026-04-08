use crate::{Mat4, Vec3};

#[derive(Debug)]
pub struct Simulation {
    pub state: State,
    pub bodies: Vec<crate::app::body::Body>,
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

    pub fn load_mesh<P>(&mut self, path: P, mat: Mat4, flatten: bool)
    where
        P: AsRef<std::path::Path>,
    {
        let mut mesh = crate::mesh::Mesh::load(path, |x| x);

        if flatten {
            mesh.flatten();
        }

        self.bodies.push(super::body::Body {
            mesh: Some(mesh),
            mat,
            ..Default::default()
        });
    }

    pub fn add_mesh(&mut self, mesh: crate::mesh::Mesh, mat: Mat4) {
        self.bodies.push(super::body::Body {
            mesh: Some(mesh),
            mat,
            ..Default::default()
        });
    }

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
    
    // return pause state after toggle
    pub fn toggle_pause(&mut self) -> bool {
        self.is_paused = !self.is_paused;
        self.is_paused
    }
}
