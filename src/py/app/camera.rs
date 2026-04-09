use std::{cell::RefCell, rc::Rc};

use numpy::{PyArrayMethods, ToPyArray};
use pyo3::prelude::*;

use crate::Float;

#[pyclass(unsendable)]
pub struct Camera {
    pub simulation: Rc<RefCell<crate::app::simulation::Simulation>>,
}

#[pymethods]
impl Camera {
    #[getter]
    fn pos<'py>(slf: pyo3::Bound<'py, Self>) -> pyo3::Bound<'py, numpy::PyArray1<Float>> {
        let simulation = &slf.borrow().simulation;
        let v = &simulation.borrow().camera.pos;
        let arr = ndarray::ArrayView1::from(v.as_ref());
        unsafe { numpy::PyArray1::borrow_from_array(&arr, slf.into_any()) }
    }

    #[setter]
    fn set_pos(&self, v: [Float; 3]) {
        self.simulation.borrow_mut().camera.pos = v.into();
    }

    #[getter]
    fn dir<'py>(slf: pyo3::Bound<'py, Self>) -> pyo3::Bound<'py, numpy::PyArray1<Float>> {
        let simulation = &slf.borrow().simulation;
        let slice = &simulation.borrow().camera.dir;
        let arr = ndarray::ArrayView1::from(slice.as_ref());
        unsafe { numpy::PyArray1::borrow_from_array(&arr, slf.into_any()) }
    }

    #[setter]
    fn set_dir(&self, v: [Float; 3]) {
        self.simulation.borrow_mut().camera.dir = v.into();
    }

    #[getter]
    fn up<'py>(slf: pyo3::Bound<'py, Self>) -> pyo3::Bound<'py, numpy::PyArray1<Float>> {
        let simulation = &slf.borrow().simulation;
        let slice = &simulation.borrow().camera.up;
        let arr = ndarray::ArrayView1::from(slice.as_ref());
        unsafe { numpy::PyArray1::borrow_from_array(&arr, slf.into_any()) }
    }

    #[setter]
    fn set_up(&self, v: [Float; 3]) {
        self.simulation.borrow_mut().camera.up = v.into();
    }

    #[getter]
    fn anchor<'py>(slf: pyo3::Bound<'py, Self>) -> pyo3::Bound<'py, numpy::PyArray1<Float>> {
        let simulation = &slf.borrow().simulation;
        let slice = &simulation.borrow().camera.anchor;
        let arr = ndarray::ArrayView1::from(slice.as_ref());
        unsafe { numpy::PyArray1::borrow_from_array(&arr, slf.into_any()) }
    }

    #[setter]
    fn set_anchor(&self, v: [Float; 3]) {
        self.simulation.borrow_mut().camera.anchor = v.into();
    }

    #[getter]
    fn up_world<'py>(slf: pyo3::Bound<'py, Self>) -> pyo3::Bound<'py, numpy::PyArray1<Float>> {
        let simulation = &slf.borrow().simulation;
        let slice = &simulation.borrow().camera.up_world;
        let arr = ndarray::ArrayView1::from(slice.as_ref());
        unsafe { numpy::PyArray1::borrow_from_array(&arr, slf.into_any()) }
    }

    #[setter]
    fn set_up_world(&self, v: [Float; 3]) {
        self.simulation.borrow_mut().camera.up_world = v.into();
    }

    fn is_control_wasd(&self) -> bool {
        self.simulation.borrow().camera.control == crate::app::camera::Control::WASD
    }

    fn is_control_arcball(&self) -> bool {
        self.simulation.borrow().camera.control == crate::app::camera::Control::Arcball
    }

    fn is_control_none(&self) -> bool {
        self.simulation.borrow().camera.control == crate::app::camera::Control::None
    }

    fn set_control_wasd(&mut self) {
        self.simulation.borrow_mut().camera.control = crate::app::camera::Control::WASD;
    }

    fn set_control_arcball(&mut self) {
        self.simulation.borrow_mut().camera.control = crate::app::camera::Control::Arcball;
    }

    fn set_control_none(&mut self) {
        self.simulation.borrow_mut().camera.control = crate::app::camera::Control::None;
    }

    #[getter]
    fn projection(&self) -> Projection {
        Projection {
            simulation: self.simulation.clone(),
        }
    }

    fn target<'py>(&self, py: Python<'py>) -> pyo3::Bound<'py, numpy::PyArray1<Float>> {
        self.simulation
            .borrow()
            .camera
            .target()
            .as_ref()
            .to_pyarray(py)
    }

    fn right<'py>(&self, py: Python<'py>) -> pyo3::Bound<'py, numpy::PyArray1<Float>> {
        self.simulation
            .borrow()
            .camera
            .right()
            .as_ref()
            .to_pyarray(py)
    }

    fn lookto<'py>(&self, py: Python<'py>) -> pyo3::Bound<'py, numpy::PyArray2<Float>> {
        self.simulation
            .borrow()
            .camera
            .lookto()
            .unwrap()
            .as_ref()
            .to_pyarray(py)
            .reshape((4, 4))
            .unwrap()
    }

    fn view_proj<'py>(
        &self,
        py: Python<'py>,
        aspect: Float,
    ) -> pyo3::Bound<'py, numpy::PyArray2<Float>> {
        self.simulation
            .borrow()
            .camera
            .view_proj(aspect)
            .unwrap()
            .as_ref()
            .to_pyarray(py)
            .reshape((4, 4))
            .unwrap()
    }

    fn mat<'py>(&self, py: Python<'py>) -> pyo3::Bound<'py, numpy::PyArray2<Float>> {
        self.simulation
            .borrow()
            .camera
            .mat()
            .as_ref()
            .to_pyarray(py)
            .reshape((3, 3))
            .unwrap()
    }

    fn fix_up(&mut self) {
        self.simulation.borrow_mut().camera.fix_up()
    }

    fn look_anchor(&mut self) {
        self.simulation.borrow_mut().camera.look_anchor()
    }

    // fn set_target<'py>(&mut self, target: pyo3::Bound<'py, numpy::PyArray1<Float>>) {
    //     let v = unsafe { target.as_slice().unwrap() };
    //     self.simulation
    //         .borrow_mut()
    //         .camera
    //         .set_target(Vec3::from_slice(v));
    // }

    fn set_target<'py>(&mut self, target: [Float; 3]) {
        self.simulation
            .borrow_mut()
            .camera
            .set_target(target.into());
    }

    fn toggle_control(&mut self) {
        self.simulation.borrow_mut().camera.toggle_control()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.simulation.borrow().camera)
    }
}

#[pyclass(unsendable)]
pub struct Projection {
    pub simulation: Rc<RefCell<crate::app::simulation::Simulation>>,
}

#[pymethods]
impl Projection {
    #[getter]
    fn fovy(&self) -> Float {
        self.simulation.borrow().camera.projection.fovy
    }

    #[setter]
    fn set_fovy(&mut self, v: Float) {
        self.simulation.borrow_mut().camera.projection.fovy = v;
    }

    #[getter]
    fn znear(&self) -> Float {
        self.simulation.borrow().camera.projection.znear
    }

    #[setter]
    fn set_znear(&mut self, v: Float) {
        self.simulation.borrow_mut().camera.projection.znear = v;
    }

    #[getter]
    fn zfar(&self) -> Float {
        self.simulation.borrow().camera.projection.zfar
    }

    #[setter]
    fn set_zfar(&mut self, v: Float) {
        self.simulation.borrow_mut().camera.projection.zfar = v;
    }

    #[getter]
    fn side(&self) -> Float {
        self.simulation.borrow().camera.projection.side
    }

    #[setter]
    fn set_side(&mut self, v: Float) {
        self.simulation.borrow_mut().camera.projection.side = v;
    }

    fn is_orthographic(&self) -> bool {
        self.simulation.borrow().camera.projection.mode
            == crate::app::camera::ProjectionMode::Orthographic
    }

    fn is_perspective(&self) -> bool {
        self.simulation.borrow().camera.projection.mode
            == crate::app::camera::ProjectionMode::Perspective
    }

    fn set_orthographic(&mut self) {
        self.simulation.borrow_mut().camera.projection.mode =
            crate::app::camera::ProjectionMode::Orthographic;
    }

    fn set_perspective(&mut self) {
        self.simulation.borrow_mut().camera.projection.mode =
            crate::app::camera::ProjectionMode::Perspective;
    }

    fn mat<'py>(&self, py: Python<'py>, aspect: Float) -> pyo3::Bound<'py, numpy::PyArray2<Float>> {
        self.simulation
            .borrow()
            .camera
            .projection
            .mat(aspect)
            .as_ref()
            .to_pyarray(py)
            .reshape((4, 4))
            .unwrap()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.simulation.borrow().camera.projection)
    }
}
