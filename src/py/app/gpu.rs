use std::{cell::RefCell, rc::Rc};

// use glam::Vec3;
use numpy::PyArrayMethods;
use pyo3::prelude::*;

use crate::{Float, Mat4};

#[pyclass(from_py_object, unsendable)]
#[derive(Clone)]
pub struct InstanceInput {
    pub inner: Rc<RefCell<crate::app::gpu::InstanceInput>>,
}

#[pymethods]
impl InstanceInput {
    #[new]
    #[pyo3(signature = (
        mat=None,
    ))]
    pub fn new(mat: Option<Bound<'_, numpy::PyArray2<Float>>>) -> Self {
        let mut instance = crate::app::gpu::InstanceInput::default();

        if let Some(mat) = mat {
            unsafe {
                instance.mat = Mat4::from_cols_slice(mat.as_slice().unwrap()).transpose();
            }

            instance.compute_normal();
        }

        Self {
            inner: Rc::new(RefCell::new(instance)),
        }
    }

    #[getter]
    fn mat<'py>(slf: pyo3::Bound<'py, Self>) -> Bound<'py, numpy::PyArray2<Float>> {
        let inner = &slf.borrow().inner;
        let mat = &inner.borrow().mat;
        let arr = ndarray::ArrayView1::from(mat.as_ref())
            .into_shape_with_order((4, 4))
            .unwrap();
        unsafe { numpy::PyArray2::borrow_from_array(&arr, slf.into_any()) }
    }

    fn set_mat(&self, arr: [[Float; 4]; 4]) {
        self.inner.borrow_mut().mat = Mat4::from_cols_array_2d(&arr);
        println!("{:?}", arr);
        println!("{}", self.inner.borrow().mat);
    }

    fn compute_normal(&mut self) {
        self.inner.borrow_mut().compute_normal();
    }
}
