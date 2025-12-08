use confique::Config as Confique;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{Float, Vec3};

#[pyclass]
#[derive(Confique, Clone, Deserialize, Serialize)]
pub struct Config {
    #[pyo3(get, set)]
    pub debug_state_creation: bool,

    #[pyo3(get, set)]
    pub debug_state_rendering: bool,

    #[pyo3(get, set)]
    pub debug_state_closure: bool,

    #[pyo3(get, set)]
    pub debug_event_device: bool,

    #[pyo3(get, set)]
    pub debug_event_window_except_redraw: bool,

    #[pyo3(get, set)]
    pub width: u32,

    #[pyo3(get, set)]
    pub height: u32,

    pub background: wgpu::Color,

    #[pyo3(get, set)]
    pub depthpass_enable: bool,

    #[pyo3(get, set)]
    pub render_light: bool,

    #[pyo3(get, set)]
    pub hdr_enable: bool,

    #[pyo3(get, set)]
    pub font: String,

    #[pyo3(get, set)]
    pub texts: Vec<ConfigText>,

    #[pyo3(get, set)]
    pub show_text_info: bool,

    #[pyo3(get, set)]
    pub fps_time_refresh: Float,

    #[pyo3(get, set)]
    pub enable_back_face: bool,

    #[pyo3(get, set)]
    pub wireframe: bool,

    pub camera_pos: Vec3,

    pub camera_dir: Vec3,

    pub camera_up: Vec3,

    pub camera_anchor: Vec3,

    pub camera_type: crate::gpu::scene::CameraType,

    pub up_world: Vec3,

    // #[pyo3(get, set)]
    // pub camera_yaw: f32, // deg

    // #[pyo3(get, set)]
    // pub camera_pitch: f32, // deg
    #[pyo3(get, set)]
    pub camera_fovy: Float, // deg

    #[pyo3(get, set)]
    pub camera_znear: Float,

    #[pyo3(get, set)]
    pub camera_zfar: Float,

    #[pyo3(get, set)]
    pub camera_projection: crate::gpu::scene::ProjectionType,

    #[pyo3(get, set)]
    pub camera_speed: Float,

    #[pyo3(get, set)]
    pub camera_sensitivity: Float,

    pub light_pos: Vec3,

    pub light_color: Vec3,

    #[pyo3(get, set)]
    pub start_paused: bool,

    #[pyo3(get, set)]
    pub global_test: u32,

    #[pyo3(get, set)]
    pub ambient_strength: Float,

    #[pyo3(get, set)]
    pub diffuse_enable: bool,

    #[pyo3(get, set)]
    pub specular_enable: bool,

    #[pyo3(get, set)]
    pub models: Vec<ConfigModel>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            debug_state_creation: false,
            debug_state_rendering: false,
            debug_state_closure: false,
            debug_event_device: false,
            debug_event_window_except_redraw: false,
            width: 1024,
            height: 768,
            depthpass_enable: false,
            render_light: false,
            hdr_enable: false,
            background: wgpu::Color::BLACK,
            font: "res/DejaVuSans.ttf".to_string(),
            texts: vec![ConfigText {
                text: "kalast".to_string(),
                ..Default::default()
            }],
            show_text_info: true,
            fps_time_refresh: 0.05,
            enable_back_face: false,
            wireframe: false,
            camera_pos: Vec3::new(10.0, 0.0, 0.0),
            // camera_yaw: -90.0,
            // camera_pitch: -20.0,
            camera_dir: Vec3::new(-1.0, 0.0, 0.0),
            camera_up: Vec3::new(0.0, 0.0, 1.0),
            camera_anchor: Vec3::new(0.0, 0.0, 0.0),
            camera_type: crate::gpu::scene::CameraType::Arcball,
            up_world: Vec3::new(0.0, 0.0, 1.0),
            camera_fovy: 45.0,
            camera_znear: 0.1,
            camera_zfar: 100.0,
            camera_projection: crate::gpu::scene::ProjectionType::Perspective,
            camera_speed: 4.0,
            camera_sensitivity: 0.4,
            light_pos: Vec3::new(2.0, 2.0, 0.0),
            light_color: Vec3::new(1.0, 1.0, 1.0),
            start_paused: false,
            global_test: 0,
            ambient_strength: 0.1,
            diffuse_enable: true,
            specular_enable: false,
            models: vec![],
        }
    }
}

impl Config {
    pub fn load(path: &str) -> Self {
        Self::from_file(path).unwrap()
    }
}

#[pymethods]
impl Config {
    #[new]
    #[pyo3(signature = (path=None))]
    pub fn py_load(path: Option<&str>) -> Self {
        if let Some(path) = path {
            Self::from_file(path).unwrap()
        } else {
            Self::default()
        }
    }

    #[getter]
    pub fn get_background(&self) -> PyResult<[Float; 4]> {
        Ok([
            self.background.r as Float,
            self.background.g as Float,
            self.background.b as Float,
            self.background.a as Float,
        ])
    }

    #[setter]
    pub fn set_background(&mut self, color: [Float; 4]) -> PyResult<()> {
        self.background.r = color[0] as f64;
        self.background.g = color[1] as f64;
        self.background.b = color[2] as f64;
        self.background.a = color[3] as f64;
        Ok(())
    }

    #[getter]
    pub fn get_camera_pos(&self) -> PyResult<[Float; 3]> {
        Ok(self.camera_pos.into())
    }

    #[setter]
    pub fn set_camera_pos(&mut self, pos: [Float; 3]) -> PyResult<()> {
        self.camera_pos.x = pos[0];
        self.camera_pos.y = pos[1];
        self.camera_pos.z = pos[2];
        Ok(())
    }

    #[getter]
    pub fn get_camera_dir(&self) -> PyResult<[Float; 3]> {
        Ok(self.camera_dir.into())
    }

    #[setter]
    pub fn set_camera_dir(&mut self, dir: [Float; 3]) -> PyResult<()> {
        self.camera_dir.x = dir[0];
        self.camera_dir.y = dir[1];
        self.camera_dir.z = dir[2];
        Ok(())
    }

    #[getter]
    pub fn get_camera_up(&self) -> PyResult<[Float; 3]> {
        Ok(self.camera_up.into())
    }

    #[setter]
    pub fn set_camera_up(&mut self, up: [Float; 3]) -> PyResult<()> {
        self.camera_up.x = up[0];
        self.camera_up.y = up[1];
        self.camera_up.z = up[2];
        Ok(())
    }

    #[getter]
    pub fn get_camera_anchor(&self) -> PyResult<[Float; 3]> {
        Ok(self.camera_anchor.into())
    }

    #[setter]
    pub fn set_camera_anchor(&mut self, anchor: [Float; 3]) -> PyResult<()> {
        self.camera_anchor.x = anchor[0];
        self.camera_anchor.y = anchor[1];
        self.camera_anchor.z = anchor[2];
        Ok(())
    }

    #[getter]
    pub fn get_up_world(&self) -> PyResult<[Float; 3]> {
        Ok(self.up_world.into())
    }

    #[setter]
    pub fn set_up_world(&mut self, up: [Float; 3]) -> PyResult<()> {
        self.up_world.x = up[0];
        self.up_world.y = up[1];
        self.up_world.z = up[2];
        Ok(())
    }

    #[getter]
    pub fn get_light_pos(&self) -> PyResult<[Float; 3]> {
        Ok(self.light_pos.into())
    }

    #[setter]
    pub fn set_light_pos(&mut self, pos: [Float; 3]) -> PyResult<()> {
        self.light_pos.x = pos[0];
        self.light_pos.y = pos[1];
        self.light_pos.z = pos[2];
        Ok(())
    }

    #[getter]
    pub fn get_light_color(&self) -> PyResult<[Float; 3]> {
        Ok(self.light_color.into())
    }

    #[setter]
    pub fn set_light_color(&mut self, color: [Float; 3]) -> PyResult<()> {
        self.light_color.x = color[0];
        self.light_color.y = color[1];
        self.light_color.z = color[2];
        Ok(())
    }
}

#[pyclass]
#[derive(Confique, Clone, Deserialize, Serialize)]
pub struct ConfigText {
    #[pyo3(get, set)]
    pub text: String,

    pub color: glam::Vec4,

    pub pos: glam::Vec2,

    pub ha: String,
}

impl Default for ConfigText {
    fn default() -> Self {
        Self {
            text: "hello".to_string(),
            color: glam::Vec4::new(1.0, 1.0, 1.0, 1.0),
            pos: glam::Vec2::new(0.0, 0.0),
            ha: "left".to_string(),
        }
    }
}

#[pymethods]
impl ConfigText {
    #[new]
    #[pyo3(signature = (text: "str", color: "list[float]" = [1.0; 4], pos: "list[float]" = [0.0; 2], ha: "str" = "left") -> "None")]
    pub fn py_new(text: &str, color: [f32; 4], pos: [f32; 2], ha: &str) -> Self {
        Self {
            text: text.to_string(),
            color: color.into(),
            pos: pos.into(),
            ha: ha.to_string(),
        }
    }
}

#[pyclass]
#[derive(Confique, Clone, Deserialize, Serialize)]
pub struct ConfigModel {
    #[pyo3(get, set)]
    pub path: String,

    #[pyo3(get, set)]
    pub flat: bool,

    pub pos_factor: Vec3,

    #[pyo3(get, set)]
    pub color_mode: Option<u32>,

    pub color: Vec3,
}

impl Default for ConfigModel {
    fn default() -> Self {
        Self {
            path: "".to_string(),
            flat: false,
            pos_factor: Vec3::ONE,
            color_mode: None,
            color: Vec3::new(0.5, 0.5, 0.5),
        }
    }
}

#[pymethods]
impl ConfigModel {
    // use pyo3::types::PyTuple;
    // #[pyo3(signature = (*args))]
    // args: &Bound<'_, PyTuple>
    // println!("{:?}", args);

    #[new]
    #[pyo3(signature = (path: "str", flat: "bool" = false, pos_factor: "list[float] " = [1.0; 3], color_mode: "int | None" = None, color: "list[float]" = [0.5; 3]) -> "None")]
    pub fn py_new(
        path: &str,
        flat: bool,
        pos_factor: [Float; 3],
        color_mode: Option<u32>,
        color: [Float; 3],
    ) -> Self {
        Self {
            path: path.into(),
            flat,
            pos_factor: pos_factor.into(),
            color_mode,
            color: color.into(),
        }
    }

    #[getter]
    pub fn get_pos_factor(&self) -> PyResult<[Float; 3]> {
        Ok([self.pos_factor.x, self.pos_factor.y, self.pos_factor.z])
    }

    #[setter]
    pub fn set_pos_factor(&mut self, factor: [Float; 3]) -> PyResult<()> {
        self.pos_factor.x = factor[0];
        self.pos_factor.y = factor[1];
        self.pos_factor.z = factor[2];
        Ok(())
    }

    #[getter]
    pub fn get_color(&self) -> PyResult<[Float; 3]> {
        Ok([self.color.x, self.color.y, self.color.z])
    }

    #[setter]
    pub fn set_color(&mut self, color: [Float; 3]) -> PyResult<()> {
        self.color.x = color[0];
        self.color.y = color[1];
        self.color.z = color[2];
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UniformGlobal {
    pub test: u32,
    pub _padding: u32,
    pub ambient_strength: Float,
    pub diffuse_enable: u32,
    pub specular_enable: u32,
}

impl super::buffer::UniformBindTrait for UniformGlobal {}
