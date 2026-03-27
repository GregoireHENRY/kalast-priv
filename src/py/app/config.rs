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
    fn set_debug_app(&mut self, debug_app: bool) {
        self.app.borrow_mut().config.debug_app = debug_app;
    }

    #[getter]
    fn debug_window(&self) -> bool {
        self.app.borrow().config.debug_window
    }

    #[setter]
    fn set_debug_window(&mut self, debug_window: bool) {
        self.app.borrow_mut().config.debug_window = debug_window;
    }

    #[getter]
    fn debug_window_mesh(&self) -> bool {
        self.app.borrow().config.debug_window_mesh
    }

    #[setter]
    fn set_debug_window_mesh(&mut self, debug_window_mesh: bool) {
        self.app.borrow_mut().config.debug_window_mesh = debug_window_mesh;
    }

    #[getter]
    fn debug_simulation(&self) -> bool {
        self.app.borrow().config.debug_simulation
    }

    #[setter]
    fn set_debug_simulation(&mut self, debug_simulation: bool) {
        self.app.borrow_mut().config.debug_simulation = debug_simulation;
    }

    #[getter]
    fn title(&self) -> String {
        self.app.borrow().config.title.clone()
    }

    #[setter]
    fn set_title(&mut self, title: &str) {
        self.app.borrow_mut().config.title = title.to_string();
    }

    #[getter]
    fn width(&self) -> u32 {
        self.app.borrow().config.width
    }

    #[setter]
    fn set_width(&mut self, width: u32) {
        self.app.borrow_mut().config.width = width;
    }

    #[getter]
    fn height(&self) -> u32 {
        self.app.borrow().config.height
    }

    #[setter]
    fn set_height(&mut self, height: u32) {
        self.app.borrow_mut().config.height = height;
    }

    #[getter]
    pub fn get_background(&self) -> [Float; 4] {
        let v = self.app.borrow().config.background;
        [v.r as Float, v.g as Float, v.b as Float, v.a as Float]
    }

    #[setter]
    pub fn set_background(&mut self, color: [Float; 4]) {
        let v = &mut self.app.borrow_mut().config.background;
        v.r = color[0] as f64;
        v.g = color[1] as f64;
        v.b = color[2] as f64;
        v.a = color[3] as f64;
    }

    #[getter]
    fn enable_back_face(&self) -> bool {
        self.app.borrow().config.enable_back_face
    }

    #[setter]
    fn set_enable_back_face(&mut self, enable_back_face: bool) {
        self.app.borrow_mut().config.enable_back_face = enable_back_face;
    }

    #[getter]
    fn sensitivity_move(&self) -> Float {
        self.app.borrow().config.sensitivity_move
    }

    #[setter]
    fn set_sensitivity_move(&mut self, sensitivity: Float) {
        self.app.borrow_mut().config.sensitivity_move = sensitivity;
    }

    #[getter]
    fn sensitivity_look(&self) -> Float {
        self.app.borrow().config.sensitivity_look
    }

    #[setter]
    fn set_sensitivity_look(&mut self, sensitivity: Float) {
        self.app.borrow_mut().config.sensitivity_look = sensitivity;
    }

    #[getter]
    fn sensitivity_rotate(&self) -> Float {
        self.app.borrow().config.sensitivity_rotate
    }

    #[setter]
    fn set_sensitivity_rotate(&mut self, sensitivity: Float) {
        self.app.borrow_mut().config.sensitivity_rotate = sensitivity;
    }

    #[getter]
    fn sensitivity_zoom(&self) -> Float {
        self.app.borrow().config.sensitivity_zoom
    }

    #[setter]
    fn set_sensitivity_zoom(&mut self, sensitivity: Float) {
        self.app.borrow_mut().config.sensitivity_zoom = sensitivity;
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.app.borrow().config)
    }
}
