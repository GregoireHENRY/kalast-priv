use std::{cell::RefCell, rc::Rc};

use numpy::PyArrayMethods;
// use numpy::{PyArrayMethods, ToPyArray};
use pyo3::prelude::*;

use crate::{Float, Mat4};

#[pyclass(from_py_object, unsendable)]
#[derive(Clone)]
pub struct Body {
    // pub simulation: Rc<RefCell<crate::app::simulation::Simulation>>,
    pub inner: Rc<RefCell<crate::app::body::Body>>,
}

#[pymethods]
impl Body {
    #[new]
    #[pyo3(signature = (
        mesh=None,
        mat=None,
        entity=None,
    ))]
    pub fn new(
        mesh: Option<crate::py::mesh::Mesh>,
        mat: Option<Bound<'_, numpy::PyArray2<Float>>>,
        entity: Option<crate::py::entity::Body>,
    ) -> Self {
        let mut body = crate::app::body::Body::new();

        if let Some(mesh) = mesh {
            body.mesh = Some(mesh.inner.clone());
        }

        if let Some(mat) = mat {
            unsafe {
                body.mat = Mat4::from_cols_slice(mat.as_slice().unwrap()).transpose();
            }
        };

        if let Some(entity) = entity {
            body.entity = Some(entity.inner.borrow().clone());
        }

        Self {
            inner: Rc::new(RefCell::new(body)),
        }
    }
}
