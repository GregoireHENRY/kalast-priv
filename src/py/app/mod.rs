pub mod body;
pub mod camera;
pub mod config;
pub mod gpu;
pub mod simulation;

use std::{cell::RefCell, rc::Rc};

use pyo3::prelude::*;

use crate::{Mat4, Vec3};

#[pyclass(unsendable)]
pub struct App {
    pub inner: Rc<RefCell<crate::app::App>>,
}

#[pymethods]
impl App {
    #[new]
    fn new() -> Self {
        let m = Mat4::from_axis_angle(Vec3::Z, 0.01);
        println!("{}", m);
        
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
    fn get_simulation(&mut self) -> simulation::Simulation {
        simulation::Simulation {
            inner: self.inner.borrow_mut().simulation.clone(),
        }
    }

    fn start(&mut self) {
        self.inner.borrow_mut().start();
    }

    #[setter]
    fn set_tick(&mut self, callback: Py<PyAny>) {
        self.inner.borrow_mut().tick = Some(crate::app::Tick::Python {
            callback,
            simulation: self.get_simulation(),
        });
    }
}
