use std::{cell::RefCell, rc::Rc};

use pyo3::prelude::*;

use crate::Float;

#[pyclass(unsendable)]
pub struct Config {
    pub app: Rc<RefCell<crate::app::App>>,
}

#[pymethods]
impl Config {
    #[getter]
    fn debug_app(&self) -> bool {
        self.app.borrow().config.debug_app
    }

    #[setter]
    fn set_debug_app(&mut self, v: bool) {
        self.app.borrow_mut().config.debug_app = v;
    }

    #[getter]
    fn debug_window(&self) -> bool {
        self.app.borrow().config.debug_window
    }

    #[setter]
    fn set_debug_window(&mut self, v: bool) {
        self.app.borrow_mut().config.debug_window = v;
    }

    #[getter]
    fn debug_window_mesh(&self) -> bool {
        self.app.borrow().config.debug_window_mesh
    }

    #[setter]
    fn set_debug_window_mesh(&mut self, v: bool) {
        self.app.borrow_mut().config.debug_window_mesh = v;
    }

    #[getter]
    fn debug_simulation(&self) -> bool {
        self.app.borrow().config.debug_simulation
    }

    #[setter]
    fn set_debug_simulation(&mut self, v: bool) {
        self.app.borrow_mut().config.debug_simulation = v;
    }

    #[getter]
    fn title(&self) -> String {
        self.app.borrow().config.title.clone()
    }

    #[setter]
    fn set_title(&mut self, v: &str) {
        self.app.borrow_mut().config.title = v.to_string();
    }

    #[getter]
    fn width(&self) -> u32 {
        self.app.borrow().config.width
    }

    #[setter]
    fn set_width(&mut self, v: u32) {
        self.app.borrow_mut().config.width = v;
    }

    #[getter]
    fn height(&self) -> u32 {
        self.app.borrow().config.height
    }

    #[setter]
    fn set_height(&mut self, v: u32) {
        self.app.borrow_mut().config.height = v;
    }

    #[getter]
    pub fn get_background(&self) -> [Float; 4] {
        let v = self.app.borrow().config.background;
        [v.r as Float, v.g as Float, v.b as Float, v.a as Float]
    }

    #[setter]
    pub fn set_background(&mut self, v: [Float; 4]) {
        let c = &mut self.app.borrow_mut().config.background;
        c.r = v[0] as f64;
        c.g = v[1] as f64;
        c.b = v[2] as f64;
        c.a = v[3] as f64;
    }

    #[getter]
    fn enable_back_face(&self) -> bool {
        self.app.borrow().config.enable_back_face
    }

    #[setter]
    fn set_enable_back_face(&mut self, v: bool) {
        self.app.borrow_mut().config.enable_back_face = v;
    }

    #[getter]
    fn sensitivity_move(&self) -> Float {
        self.app.borrow().config.sensitivity_move
    }

    #[setter]
    fn set_sensitivity_move(&mut self, v: Float) {
        self.app.borrow_mut().config.sensitivity_move = v;
    }

    #[getter]
    fn sensitivity_look(&self) -> Float {
        self.app.borrow().config.sensitivity_look
    }

    #[setter]
    fn set_sensitivity_look(&mut self, v: Float) {
        self.app.borrow_mut().config.sensitivity_look = v;
    }

    #[getter]
    fn sensitivity_rotate(&self) -> Float {
        self.app.borrow().config.sensitivity_rotate
    }

    #[setter]
    fn set_sensitivity_rotate(&mut self, v: Float) {
        self.app.borrow_mut().config.sensitivity_rotate = v;
    }

    #[getter]
    fn sensitivity_zoom(&self) -> Float {
        self.app.borrow().config.sensitivity_zoom
    }

    #[setter]
    fn set_sensitivity_zoom(&mut self, v: Float) {
        self.app.borrow_mut().config.sensitivity_zoom = v;
    }

    #[getter]
    fn shader_color_mode(&self) -> u32 {
        self.app.borrow().config.shader_color_mode
    }

    #[setter]
    fn set_shader_color_mode(&mut self, v: u32) {
        self.app.borrow_mut().config.shader_color_mode = v;
    }

    #[getter]
    fn shader_extra(&self) -> u32 {
        self.app.borrow().config.shader_extra
    }

    #[setter]
    fn set_shader_extra(&mut self, v: u32) {
        self.app.borrow_mut().config.shader_extra = v;
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.app.borrow().config)
    }
}
