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
        // color=None,
        // color_mode=None,
    ))]
    pub fn new(
        mat: Option<Bound<'_, numpy::PyArray2<Float>>>,
        // color: Option<Bound<'_, numpy::PyArray1<Float>>>,
        // color_mode: Option<u32>,
    ) -> Self {
        let mut instance = crate::app::gpu::InstanceInput::default();

        if let Some(mat) = mat {
            unsafe {
                instance.mat = Mat4::from_cols_slice(mat.as_slice().unwrap())
                    .transpose()
                    .to_cols_array_2d();
            }
        }

        /*
        if let Some(color) = color {
            unsafe {
                instance.color = Vec3::from_slice(color.as_slice().unwrap());
            }
        }

        if let Some(color_mode) = color_mode {
            instance.color_mode = color_mode;
        }
        */

        Self {
            inner: Rc::new(RefCell::new(instance)),
        }
    }
}
