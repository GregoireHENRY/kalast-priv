use std::vec;

use ndarray::Array1;
use pyo3::{IntoPyObjectExt, prelude::*};

use crate::{Float, Mat4, Vec3};

#[derive(Clone)]
pub struct ProgressDebug {
    pub frequency: String,
    pub digits_full: usize,
    pub digits_decimal: usize,
}

impl ProgressDebug {
    pub fn new() -> Self {
        Self {
            frequency: "10".to_string(),
            digits_full: 3,
            digits_decimal: 0,
        }
    }
}

impl std::fmt::Debug for ProgressDebug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ProgressDebug(frequency={}, digits_full={}, digits_decimal={})",
            self.frequency, self.digits_full, self.digits_decimal,
        )
    }
}

#[derive(Clone)]
pub struct Time {
    pub dt: Float,
    pub duration_total: Float,
    pub duration_record: Float,
}

impl Time {
    pub fn new() -> Self {
        Self {
            dt: 0.0,
            duration_total: 0.0,
            duration_record: 0.0,
        }
    }
}

impl std::fmt::Debug for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Time(dt={}, duration_total={}, duration_record={})",
            self.dt, self.duration_total, self.duration_record,
        )
    }
}

#[derive(Clone, Debug)]
pub enum FacetSelection {
    Some(Vec<usize>),
    All,
}

#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct Record {
    #[pyo3(get, set)]
    pub temperature_surface: bool,

    #[pyo3(get, set)]
    pub flux_surface: bool,

    pub surface_facets: FacetSelection,

    #[pyo3(get, set)]
    pub temperature_interior: bool,

    #[pyo3(get, set)]
    pub interior_time_indices: Vec<usize>,
}

#[pymethods]
impl Record {
    #[new]
    pub fn new() -> Self {
        Self {
            temperature_surface: false,
            flux_surface: false,
            surface_facets: FacetSelection::Some(vec![]),
            temperature_interior: false,
            interior_time_indices: vec![],
        }
    }

    #[getter]
    pub fn get_surface_facets<'py>(&self, py: Python<'py>) -> PyResult<Py<PyAny>> {
        match &self.surface_facets {
            FacetSelection::All => Ok("ALL".into_py_any(py).unwrap()),
            FacetSelection::Some(facets) => Ok(facets.into_py_any(py).unwrap()),
        }
    }

    #[setter]
    pub fn set_surface_facets(&mut self, facets: Vec<usize>) {
        self.surface_facets = FacetSelection::Some(facets);
    }

    pub fn set_surface_facets_all(&mut self) {
        self.surface_facets = FacetSelection::All;
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self)
    }
}

impl std::fmt::Debug for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Record(temperature_surface={}, flux_surface={}, surface_facets={:?}, temperature_interior={}, interior_time_indices={:?})",
            self.temperature_surface,
            self.flux_surface,
            self.surface_facets,
            self.temperature_interior,
            self.interior_time_indices
        )
    }
}

#[pyclass(from_py_object)]
#[pyo3(get_all, set_all)]
#[derive(Clone)]
pub struct SkinDepthParams {
    pub diffusivity: Float,
    pub period: Float,
}

#[pymethods]
impl SkinDepthParams {
    #[new]
    pub fn new() -> Self {
        Self {
            diffusivity: 0.0,
            period: 0.0,
        }
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self)
    }
}

impl std::fmt::Debug for SkinDepthParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SkinDepthParams(diffusivity={}, period={})",
            self.diffusivity, self.period,
        )
    }
}

#[derive(Debug, Clone)]
pub enum DepthOption {
    X(Float),
    SkinDepth1(usize),
    SkinDepth2pi,
}

impl DepthOption {
    pub fn value(&self, params: Option<SkinDepthParams>) -> Float {
        match self {
            Self::X(x) => *x,
            Self::SkinDepth1(n) => {
                let SkinDepthParams {
                    diffusivity,
                    period,
                } = params.unwrap();
                crate::tpm::properties::skin_depth_1(diffusivity, period) * (*n as Float)
            }
            Self::SkinDepth2pi => {
                let SkinDepthParams {
                    diffusivity,
                    period,
                } = params.unwrap();
                crate::tpm::properties::skin_depth_2pi(diffusivity, period)
            }
        }
    }
}

#[pyclass(from_py_object)]
#[derive(Clone)]
pub struct SetupColumn {
    pub depth_max: DepthOption,

    #[pyo3(get, set)]
    pub dx: Float,
}

#[pymethods]
impl SetupColumn {
    #[new]
    pub fn new() -> Self {
        Self {
            depth_max: DepthOption::SkinDepth2pi,
            dx: 0.0,
        }
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self)
    }
}

impl std::fmt::Debug for SetupColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SkinDepthParams(depth_max={:?}, dx={})",
            self.depth_max, self.dx,
        )
    }
}

impl SetupColumn {
    pub fn make_column(&self, params: Option<SkinDepthParams>) -> Array1<Float> {
        let depth_max = self.depth_max.value(params);
        Array1::range(0.0, depth_max + self.dx, self.dx)
    }
}

#[derive(Debug, Clone)]
pub enum Interior {
    Column(Vec<Float>),
    SetupColumn(SetupColumn),
}

#[derive(Clone)]
pub struct Body {
    pub mesh: crate::mesh::Mesh,
    pub interior: Interior,
    pub state: Mat4,
    pub spin_period: Float,
    pub spin_axis: Vec3,
    pub orbit_period: Float,
    pub orbit_axis: Vec3,
}

impl Body {
    pub fn new() -> Self {
        Self {
            mesh: crate::mesh::Mesh::new(),
            interior: Interior::Column(vec![]),
            state: Mat4::IDENTITY,
            spin_period: 0.0,
            spin_axis: Vec3::Z,
            orbit_period: 0.0,
            orbit_axis: Vec3::Z,
        }
    }
}

impl std::fmt::Debug for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Body(mesh={:?}, interior={:?}, state={}, spin_period={}, spin_axis={}, orbit_period={}, orbit_axis={})",
            self.mesh,
            self.interior,
            &self.state,
            self.spin_period,
            &self.spin_axis,
            self.orbit_period,
            &self.orbit_axis,
        )
    }
}

#[derive(Clone)]
pub struct BodyDataMap {
    pub temperatures: Vec<Array1<Float>>,

    // thermal properties for the whole body
    pub thermal_properties_all: usize,

    // can map facet index (defined in surface of Body) to index of thermal properties (defined in Setup)
    pub thermal_properties_map: Vec<(usize, usize)>,

    pub record: Record,
}

impl BodyDataMap {
    pub fn new() -> Self {
        Self {
            temperatures: vec![],
            thermal_properties_all: 0,
            thermal_properties_map: vec![],
            record: Record::new(),
        }
    }
}

impl std::fmt::Debug for BodyDataMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BodyDataMap(temperatures={:?}, thermal_properties_all={}, thermal_properties_map={:?}, record={:?})",
            &self.temperatures,
            self.thermal_properties_all,
            self.thermal_properties_map,
            self.record,
        )
    }
}

#[derive(Clone)]
pub struct Setup {
    pub sun_position: Vec3,
    pub thermal_properties: Vec<crate::tpm::properties::Properties>,
    pub bodies: Vec<Body>,
    pub bodies_data_map: Vec<BodyDataMap>,
    pub progress_debug: ProgressDebug,
    pub time: Time,
}

impl Setup {
    pub fn new() -> Self {
        Self {
            sun_position: Vec3::ZERO,
            thermal_properties: vec![],
            bodies: vec![],
            bodies_data_map: vec![],
            progress_debug: ProgressDebug::new(),
            time: Time::new(),
        }
    }

    pub fn prepare(&mut self) {}
}

impl std::fmt::Debug for Setup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Record(sun_position={}, thermal_properties={:?}, bodies={:?}, bodies_data_map={:?}, progress_debug={:?}, time={:?})",
            &self.sun_position,
            self.thermal_properties,
            self.bodies,
            self.bodies_data_map,
            self.progress_debug,
            self.time,
        )
    }
}
