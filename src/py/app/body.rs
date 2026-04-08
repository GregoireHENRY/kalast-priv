use std::{cell::RefCell, rc::Rc};

use pyo3::prelude::*;

#[pyclass(from_py_object, unsendable)]
#[derive(Clone)]
pub struct Body {
    pub inner: Rc<RefCell<crate::app::body::Body>>,
}

#[pymethods]
impl Body {
    /*
    #[new]
    #[pyo3(signature = (
        mesh=None,
        instance=None,
        entity=None,
    ))]
    pub fn new(
        mesh: Option<crate::py::mesh::Mesh>,
        instance: Option<super::gpu::InstanceInput>,
        entity: Option<crate::py::entity::Body>,
    ) -> Self {
        let mut body = crate::app::body::Body::new();

        if let Some(mesh) = mesh {
            body.mesh = Some(mesh.inner.clone());
        }

        if let Some(instance) = instance {
            body.instance = instance.inner.borrow().clone();
        };

        if let Some(entity) = entity {
            body.entity = Some(entity.inner.borrow().clone());
        }

        Self {
            inner: Rc::new(RefCell::new(body)),
        }
    }

    #[getter]
    fn instance(&self) -> super::gpu::InstanceInput {
        super::gpu::InstanceInput {
            inner: Rc::new(RefCell::new(self.inner.borrow().instance)),
        }
    }
    */
}
