use crate::Vec3;

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
