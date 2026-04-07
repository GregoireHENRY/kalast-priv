use std::{cell::RefCell, rc::Rc};

use pyo3::prelude::*;

use crate::Float;

#[pyclass(unsendable)]
pub struct Simulation {
    pub inner: Rc<RefCell<crate::app::simulation::Simulation>>,
}

#[pymethods]
impl Simulation {
    #[getter]
    fn state(&self) -> State {
        State {
            simulation: self.inner.clone(),
        }
    }

    #[getter]
    fn camera(&self) -> super::camera::Camera {
        super::camera::Camera {
            simulation: self.inner.clone(),
        }
    }

    #[getter]
    fn sun<'py>(slf: pyo3::Bound<'py, Self>) -> pyo3::Bound<'py, numpy::PyArray1<Float>> {
        let inner = &slf.borrow().inner;
        let slice = &inner.borrow().sun;
        let arr = ndarray::ArrayView1::from(slice.as_ref());
        unsafe { numpy::PyArray1::borrow_from_array(&arr, slf.into_any()) }
    }

    #[setter]
    fn set_sun(&self, arr: [Float; 3]) {
        self.inner.borrow_mut().sun = arr.into();
    }

    #[pyo3(signature = (
        mesh=None,
        instance=None,
        mat=None,
        // color=None,
        // color_mode=None,
        entity=None,
        body=None
    ))]
    fn add_body(
        &mut self,
        mesh: Option<crate::py::mesh::Mesh>,
        instance: Option<super::gpu::InstanceInput>,
        mat: Option<Bound<'_, numpy::PyArray2<Float>>>,
        // color: Option<Bound<'_, numpy::PyArray1<Float>>>,
        // color_mode: Option<u32>,
        entity: Option<crate::py::entity::Body>,
        body: Option<super::body::Body>,
    ) {
        self.inner
            .borrow_mut()
            .bodies
            .push(if let Some(body) = body {
                body.inner.clone()
            } else {
                let instance = instance.unwrap_or(super::gpu::InstanceInput::new(
                    mat,
                    // color, color_mode
                ));
                super::body::Body::new(mesh, Some(instance), entity)
                    .inner
                    .clone()
            });
    }

    fn get_matrix_model<'py>(
        slf: pyo3::Bound<'py, Self>,
        index: usize,
    ) -> Bound<'py, numpy::PyArray2<Float>> {
        let inner = &slf.borrow().inner;
        let body = &inner.borrow().bodies[index];
        let slice = body.borrow().instance.mat;
        let arr = ndarray::ArrayView2::from(slice.as_ref());
        unsafe { numpy::PyArray2::borrow_from_array(&arr, slf.into_any()) }
    }

    fn update(&mut self) {
        self.inner.borrow_mut().update();
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.inner.borrow())
    }
}

#[pyclass(unsendable)]
pub struct State {
    pub simulation: Rc<RefCell<crate::app::simulation::Simulation>>,
}

#[pymethods]
impl State {
    #[getter]
    fn iteration(&self) -> usize {
        self.simulation.borrow().state.iteration
    }

    #[setter]
    fn set_iteration(&mut self, iteration: usize) {
        self.simulation.borrow_mut().state.iteration = iteration;
    }

    #[getter]
    fn is_paused(&self) -> bool {
        self.simulation.borrow().state.is_paused
    }

    #[setter]
    fn set_is_paused(&mut self, is_paused: bool) {
        self.simulation.borrow_mut().state.is_paused = is_paused;
    }

    #[getter]
    fn pause_at(&self) -> Option<usize> {
        self.simulation.borrow().state.pause_at
    }

    #[setter]
    fn set_pause_at(&mut self, pause_at: Option<usize>) {
        self.simulation.borrow_mut().state.pause_at = pause_at;
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.simulation.borrow().state)
    }
}
