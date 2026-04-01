pub mod body;
pub mod camera;
pub mod config;
pub mod simulation;
pub mod gpu;

use std::{cell::RefCell, rc::Rc};

use pyo3::prelude::*;

#[pyclass(unsendable)]
pub struct App {
    pub inner: Rc<RefCell<crate::app::App>>,
}

#[pymethods]
impl App {
    #[new]
    fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(crate::app::App::new())),
        }
    }

    #[getter]
    fn config(&self) -> config::Config {
        config::Config {
            app: self.inner.clone(),
        }
    }

    #[getter]
    fn simulation(&self) -> simulation::Simulation {
        simulation::Simulation {
            inner: self.inner.borrow().simulation.clone(),
        }
    }

    fn start(&mut self) {
        self.inner.borrow_mut().start();
    }

    #[setter]
    fn set_tick(&mut self, callback: Py<PyAny>) {
        Python::attach(|py| {
            let simulation = Py::new(
                py,
                simulation::Simulation {
                    inner: self.inner.borrow().simulation.clone(),
                },
            )
            .unwrap();

            self.inner.borrow_mut().tick = Some(crate::app::Tick::Python {
                callback,
                simulation,
            });
        });
    }
}
