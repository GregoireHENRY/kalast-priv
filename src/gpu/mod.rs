pub mod buffer;
pub mod compute;
pub mod config;
pub mod hdr;
pub mod light;
pub mod model;
pub mod render;
pub mod scene;
pub mod text;
pub mod text_msd;
pub mod texture;
pub mod win;

use crate::Float;
use pyo3::prelude::*;

#[pyclass]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct FpsCounter {
    pub time: std::time::Duration,
    pub frames: usize,
    __fps: usize,
}

impl FpsCounter {
    pub fn new() -> Self {
        Self {
            time: std::time::Duration::from_secs_f64(0.0),
            frames: 0,
            __fps: 0,
        }
    }

    pub fn update(&mut self, time: std::time::Duration, refresh: Float) -> usize {
        self.frames += 1;
        self.time += time;

        let t = self.time.as_secs_f64() as Float;
        if t >= refresh {
            self.__fps = (self.frames as Float / t) as usize;
            self.time = std::time::Duration::from_secs_f64(0.0);
            self.frames = 0;
        }

        self.__fps
    }
}
