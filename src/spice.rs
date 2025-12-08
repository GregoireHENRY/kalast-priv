use std::sync::Mutex;

use numpy::{PyArray1, PyArrayMethods, ndarray::ArrayView1};
use once_cell::sync::Lazy;
use pyo3::prelude::*;

use crate::{Float, UVec2, Vec3};

pub static BODIES: Lazy<Mutex<Vec<Body>>> = Lazy::new(|| {
    Mutex::new(vec![
        Body {
            entity: Entity {
                id: 399,
                name: "EARTH".to_string(),

                // IAU_EART HITRF93 EARTH_FIXED
                frame: "IAU_EARTH".to_string(),
                label: "".to_string(),
            },
            radii: Vec3::new(6378.1366, 6378.1366, 6356.751) * 1e3,
            orbit_period: 365.25 * 86400.0,
            spin_period: 0.0,
        },
        Body {
            entity: Entity {
                id: 301,
                name: "MOON".to_string(),
                frame: "IAU_MOON".to_string(),
                label: "".to_string(),
            },
            radii: Vec3::new(1738.1, 1738.1, 1736.0) * 1e3,
            orbit_period: 29.5 * 86400.0,
            spin_period: 29.5 * 86400.0,
        },
        Body {
            entity: Entity {
                id: 499,
                name: "MARS".to_string(),
                frame: "IAU_MARS".to_string(),
                label: "".to_string(),
            },
            radii: Vec3::new(3396.19, 3396.19, 3376.2) * 1e3,
            orbit_period: 687.0 * 86400.0,
            spin_period: 0.0,
        },
        Body {
            entity: Entity {
                id: 401,
                name: "PHOBOS".to_string(),
                frame: "IAU_PHOBOS".to_string(),
                label: "".to_string(),
            },
            radii: Vec3::new(13.0, 11.4, 9.1) * 1e3,
            orbit_period: 7.0 * 3600.0 + 39.0 * 60.0,
            spin_period: 7.0 * 3600.0 + 39.0 * 60.0,
        },
        Body {
            entity: Entity {
                id: 402,
                name: "DEIMOS".to_string(),
                frame: "IAU_DEIMOS".to_string(),
                label: "".to_string(),
            },
            radii: Vec3::new(7.8, 6.0, 5.1) * 1e3,
            orbit_period: 30.312 * 3600.0,
            spin_period: 30.312 * 3600.0,
        },
        Body {
            entity: Entity {
                id: 65803,
                name: "DIDYMOS".to_string(),
                frame: "DIDYMOS_FIXED".to_string(),
                label: "".to_string(),
            },
            radii: Vec3::new(409.5, 400.5, 302.5),
            orbit_period: 700.0 * 86400.0,
            spin_period: 2.26 * 3600.0,
        },
        Body {
            entity: Entity {
                id: 65803,
                name: "DIMORPHOS".to_string(),
                frame: "DIMORPHOS_FIXED".to_string(),
                label: "post_impact".to_string(),
            },
            radii: Vec3::new(88.5, 84.0, 57.0),
            orbit_period: 11.3676 * 3600.0,
            spin_period: 11.3676 * 3600.0,
        },
        Body {
            entity: Entity {
                id: 65803,
                name: "DIMORPHOS".to_string(),
                frame: "DIMORPHOS_FIXED".to_string(),
                label: "pre_impact".to_string(),
            },
            radii: Vec3::new(88.5, 84.0, 57.0),
            orbit_period: 11.921473 * 3600.0,
            spin_period: 11.921473 * 3600.0,
        },
    ])
});

pub static CAMERAS: Lazy<Mutex<Vec<Camera>>> = Lazy::new(|| {
    Mutex::new(vec![
        Camera {
            entity: Entity {
                id: -91200,
                name: "HERA_TIRI".to_string(),
                frame: "HERA_TIRI".to_string(),
                label: "".to_string(),
            },
            px: UVec2::new(1024, 768),
            fovy: 10.0,
        },
        Camera {
            entity: Entity {
                id: 0,
                name: "HERA_AFC-1".to_string(),
                frame: "HERA_AFC-1".to_string(),
                label: "".to_string(),
            },
            px: UVec2::new(1024, 1024),
            fovy: 5.47,
        },
    ])
});

pub static SPACECRAFTS: Lazy<Mutex<Vec<Spacecraft>>> = Lazy::new(|| {
    Mutex::new(vec![
        Spacecraft {
            entity: Entity {
                id: 0,
                name: "HERA_AFC-1".to_string(),
                frame: "HERA_AFC-1".to_string(),
                label: "".to_string(),
            },
            id_cameras: vec![0, 1],
        },
        Spacecraft {
            entity: Entity {
                id: -58,
                name: "HALCA".to_string(),
                frame: "".to_string(),
                label: "PHOBOS2".to_string(),
            },
            id_cameras: vec![],
        },
        Spacecraft {
            entity: Entity {
                id: 0,
                name: "MEX_SPACECRAFT".to_string(),
                frame: "MEX_SPACECRAFT".to_string(),
                label: "".to_string(),
            },
            id_cameras: vec![],
        },
        Spacecraft {
            entity: Entity {
                id: 0,
                name: "TGO_SPACECRAFT".to_string(),
                frame: "TGO_SPACECRAFT".to_string(),
                label: "".to_string(),
            },
            id_cameras: vec![],
        },
    ])
});

#[derive(Debug, Clone)]
pub enum EntityKind {
    Body(Body),
    Camera(Camera),
    Spacecraft(Spacecraft),
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub id: isize,
    pub name: String,
    pub frame: String,
    pub label: String,
}

impl Entity {
    pub const fn new() -> Self {
        Self {
            id: 0,
            name: String::new(),
            frame: String::new(),
            label: String::new(),
        }
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct Body {
    pub entity: Entity,
    pub radii: Vec3,
    #[pyo3(get, set)]
    pub orbit_period: Float,
    #[pyo3(get, set)]
    pub spin_period: Float,
}

#[pymethods]
impl Body {
    #[new]

    pub const fn new() -> Self {
        Self {
            entity: Entity::new(),
            radii: Vec3::ZERO,
            orbit_period: 0.0,
            spin_period: 0.0,
        }
    }

    #[getter]
    pub fn id(&self) -> isize {
        self.entity.id
    }

    #[getter]
    pub fn name(&self) -> &str {
        &self.entity.name
    }

    #[getter]
    pub fn frame(&self) -> &str {
        &self.entity.frame
    }

    #[setter]
    pub fn set_id(&mut self, id: isize) {
        self.entity.id = id;
    }

    #[setter]
    pub fn set_name(&mut self, name: &str) {
        self.entity.name = name.to_string();
    }

    #[setter]
    pub fn set_frame(&mut self, frame: &str) {
        self.entity.frame = frame.to_string();
    }

    // Getter numpy.ndarray read and write.
    #[getter]
    fn radii<'py>(slf: Bound<'py, Self>) -> Bound<'py, PyArray1<Float>> {
        let slice = &slf.borrow().radii;
        let slice2 = slice.as_ref();
        let arr = ArrayView1::from(slice2);
        unsafe { PyArray1::borrow_from_array(&arr, slf.into_any()) }
    }

    // Setter numpy.ndarray to allow shorthand operators.
    #[setter]
    pub fn set_radii<'py>(&mut self, radii: Bound<'py, PyArray1<Float>>) {
        let radii = unsafe { radii.as_slice().unwrap() };
        self.radii.x = radii[0];
        self.radii.y = radii[1];
        self.radii.z = radii[2];
    }

    pub fn radius(&self) -> Float {
        self.radii.element_sum() / 3.0
    }

    pub fn diameter(&self) -> Float {
        self.radius() * 2.0
    }

    pub fn flattening_radius(&self) -> Float {
        crate::math::flattening_radius(&self.radii)
    }

    pub fn __repr__(&self) -> String {
        format!(
            "Body(id={}, name={}, frame={}, label={}, radii={}, orbit_period={}, spin_period={})",
            self.entity.id,
            self.entity.name,
            self.entity.frame,
            self.entity.label,
            self.radii,
            self.orbit_period,
            self.spin_period
        )
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct Camera {
    pub entity: Entity,
    pub px: UVec2,
    #[pyo3(get, set)]
    pub fovy: Float, // in degree
}

#[pymethods]
impl Camera {
    #[new]

    pub const fn new() -> Self {
        Self {
            entity: Entity::new(),
            px: UVec2::ZERO,
            fovy: 0.0,
        }
    }

    #[getter]
    pub fn id(&self) -> isize {
        self.entity.id
    }

    #[getter]
    pub fn name(&self) -> &str {
        &self.entity.name
    }

    #[getter]
    pub fn frame(&self) -> &str {
        &self.entity.frame
    }

    #[setter]
    pub fn set_id(&mut self, id: isize) {
        self.entity.id = id;
    }

    #[setter]
    pub fn set_name(&mut self, name: &str) {
        self.entity.name = name.to_string();
    }

    #[setter]
    pub fn set_frame(&mut self, frame: &str) {
        self.entity.frame = frame.to_string();
    }

    // Getter numpy.ndarray read and write.
    #[getter]
    fn px<'py>(slf: Bound<'py, Self>) -> Bound<'py, PyArray1<usize>> {
        let slice = &slf.borrow().px;
        let slice2 = slice.as_ref();
        let arr = ArrayView1::from(slice2);
        unsafe { PyArray1::borrow_from_array(&arr, slf.into_any()) }
    }

    // Setter numpy.ndarray to allow shorthand operators.
    #[setter]
    pub fn set_px<'py>(&mut self, px: Bound<'py, PyArray1<usize>>) {
        let px = unsafe { px.as_slice().unwrap() };
        self.px.x = px[0];
        self.px.y = px[1];
    }

    pub fn npx(&self) -> usize {
        self.px.element_product()
    }

    pub fn __repr__(&self) -> String {
        format!(
            "Camera(id={}, name={}, frame={}, label={}, px={}, fovy={})",
            self.entity.id,
            self.entity.name,
            self.entity.frame,
            self.entity.label,
            self.px,
            self.fovy
        )
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct Spacecraft {
    pub entity: Entity,
    #[pyo3(get, set)]
    pub id_cameras: Vec<usize>,
}

#[pymethods]
impl Spacecraft {
    #[new]
    pub const fn new() -> Self {
        Self {
            entity: Entity::new(),
            id_cameras: vec![],
        }
    }

    #[getter]
    pub fn id(&self) -> isize {
        self.entity.id
    }

    #[getter]
    pub fn name(&self) -> &str {
        &self.entity.name
    }

    #[getter]
    pub fn frame(&self) -> &str {
        &self.entity.frame
    }

    #[setter]
    pub fn set_id(&mut self, id: isize) {
        self.entity.id = id;
    }

    #[setter]
    pub fn set_name(&mut self, name: &str) {
        self.entity.name = name.to_string();
    }

    #[setter]
    pub fn set_frame(&mut self, frame: &str) {
        self.entity.frame = frame.to_string();
    }

    pub fn __repr__(&self) -> String {
        let mut s = "".to_string();
        for (ii, id) in self.id_cameras.iter().enumerate() {
            if ii > 0 {
                s += ", ";
            }
            s += &format!("{}", id);
        }
        format!(
            "Spacecraft(id={}, name={}, frame={}, label={}, id_cameras=[{}])",
            self.entity.id, self.entity.name, self.entity.frame, self.entity.label, s
        )
    }
}
