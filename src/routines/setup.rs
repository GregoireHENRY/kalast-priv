// use numpy::{
//     PyArray1, PyArrayMethods,
//     ndarray::{Array1, ArrayView1},
// };

use numpy::ndarray::Array1;
use pyo3::{IntoPyObjectExt, prelude::*};

use crate::{Float, Mat4, Vec3};

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

#[pyclass]
#[pyo3(get_all, set_all)]
#[derive(Clone)]
pub struct ProgressDebug {
    pub frequency: String,
    pub digits_full: usize,
    pub digits_decimal: usize,
}

#[pymethods]
impl ProgressDebug {
    #[new]
    pub fn new() -> Self {
        Self {
            frequency: "10".to_string(),
            digits_full: 3,
            digits_decimal: 0,
        }
    }

    pub fn __repr__(&self) -> String {
        format!(
            "ProgressDebug(frequency={}, digits_full={}, digits_decimal={})",
            self.frequency, self.digits_full, self.digits_decimal,
        )
    }
}

impl std::fmt::Debug for ProgressDebug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.__repr__())
    }
}

#[derive(Clone, Debug)]
pub enum FacetSelection {
    Some(Vec<usize>),
    All,
}

// impl std::fmt::Debug for FacetSelection {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_fmt(format_args!("FacetSelection({})", "ALL"))
//     }
// }

#[pyclass]
#[derive(Clone)]
pub struct Record {
    #[pyo3(get, set)]
    pub temperature_surface: bool,

    #[pyo3(get, set)]
    pub flux_surface: bool,

    // write getter / setter
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
        format!(
            "Record(temperature_surface={}, flux_surface={}, surface_facets={:?}, temperature_interior={}, interior_time_indices={:?})",
            self.temperature_surface,
            self.flux_surface,
            self.surface_facets,
            self.temperature_interior,
            self.interior_time_indices
        )
    }
}

impl std::fmt::Debug for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.__repr__())
    }
}

#[derive(Debug, Clone)]
pub enum Surface {
    Mesh(crate::mesh::Mesh),
    Facets(Vec<crate::mesh::Facet>),
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct SkinDepthParams {
    pub diffusivity: Float,
    pub period: Float,
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

#[pyclass]
#[derive(Debug, Clone)]
pub struct SetupColumn {
    pub depth_max: DepthOption,
    pub dx: Float,
}

impl SetupColumn {
    pub fn make_column(&self, params: Option<SkinDepthParams>) -> Array1<Float> {
        let depth_max = self.depth_max.value(params);
        Array1::range(0.0, depth_max + self.dx, self.dx)
    }
}

#[derive(Debug, Clone)]
pub enum Interior {
    None(),
    Column(Array1<Float>),
    SetupColumn(SetupColumn),
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct Body {
    pub surface: Surface,
    pub interior: Interior,
    pub state: Mat4,
    pub spin_period: Float,
    pub spin_axis: Vec3,
    pub orbit_period: Float,
    pub orbit_axis: Vec3,
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct BodyDataMap {
    pub temperatures: Vec<Array1<Float>>,

    // thermal properties for the whole body
    pub thermal_properties_all: usize,

    // can map facet index (defined in surface of Body) to index of thermal properties (defined in Setup)
    pub thermal_properties_map: Vec<(usize, usize)>,

    pub record: Record,
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

    // -> Bound<'py, &mut Time>
    // #[getter]
    // fn time<'py>(slf: Bound<'py, Self>) -> Bound<'py, PyAny> {
    //     // Ok(&mut slf.borrow_mut().time)
    //     slf.borrow_mut().time.into_bound_py_any(slf.py()).unwrap()
    // }
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

pub mod py {
    use std::{cell::RefCell, rc::Rc};

    use pyo3::prelude::*;

    use super::Time as RsTime;
    use crate::{Float, Vec3};

    #[pyclass(unsendable)]
    #[derive(Clone)]
    pub struct Time {
        inner: Rc<RefCell<RsTime>>,
    }

    #[pymethods]
    impl Time {
        #[new]
        #[pyo3(signature = (dt=0.0, duration_total=0.0, duration_record=0.0))]
        fn new(dt: Float, duration_total: Float, duration_record: Float) -> Self {
            Self {
                inner: Rc::new(RefCell::new(RsTime {
                    dt,
                    duration_total,
                    duration_record,
                })),
            }
        }

        #[getter]
        fn dt(&self) -> Float {
            self.inner.borrow().dt
        }

        #[setter]
        fn set_dt(&self, v: Float) {
            self.inner.borrow_mut().dt = v;
        }

        #[getter]
        fn duration_total(&self) -> Float {
            self.inner.borrow().duration_total
        }

        #[setter]
        fn set_duration_total(&self, v: Float) {
            self.inner.borrow_mut().duration_total = v;
        }

        #[getter]
        fn duration_record(&self) -> Float {
            self.inner.borrow().duration_record
        }

        #[setter]
        fn set_duration_record(&self, v: Float) {
            self.inner.borrow_mut().duration_record = v;
        }

        pub fn __repr__(&self) -> String {
            format!("{:?}", self.inner.borrow())
        }
    }

    #[pyclass(unsendable)]
    pub struct Setup {
        pub sun_position: RefCell<Vec3>,
        pub time: Rc<RefCell<RsTime>>,
    }

    #[pymethods]
    impl Setup {
        #[new]
        fn new() -> Self {
            Self {
                sun_position: RefCell::new(Vec3::ZERO),
                time: Rc::new(RefCell::new(RsTime::new())),
            }
        }

        // Getter (and setter) numpy.ndarray, no clone.
        // #[getter]
        // fn sun_position<'py>(slf: Bound<'py, Self>) -> Bound<'py, PyArray1<Float>> {
        //     let slice = &slf.borrow().sun_position;
        //     let slice2 = slice.as_ref();
        //     let arr = ArrayView1::from(slice2);
        //     unsafe { PyArray1::borrow_from_array(&arr, slf.into_any()) }
        // }

        // Setter numpy.ndarray for shorthand operators.
        // #[setter]
        // pub fn set_sun_position<'py>(&mut self, sun_position: Bound<'py, PyArray1<Float>>) {
        //     let sun_position = unsafe { sun_position.as_slice().unwrap() };
        //     self.sun_position.x = sun_position[0];
        //     self.sun_position.y = sun_position[1];
        //     self.sun_position.z = sun_position[2];
        // }

        #[getter]
        fn time<'py>(&'py self, py: Python<'py>) -> PyResult<Py<Time>> {
            let time = Time {
                inner: Rc::clone(&self.time),
            };
            Py::new(py, time)
        }

        #[setter]
        fn set_time(&mut self, time: PyRef<Time>) {
            self.time = Rc::clone(&time.inner);
        }

        pub fn __repr__(&self) -> String {
            format!(
                "Setup(sun_position={}, time={:?})",
                self.sun_position.borrow(),
                self.time.borrow()
            )
        }
    }
}
