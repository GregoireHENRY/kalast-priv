use std::{cell::RefCell, rc::Rc};

use ndarray::Array1;
use numpy::{PyArrayMethods, ToPyArray, ndarray};
use pyo3::prelude::*;

use crate::{
    Float,
    //Mat4,
    py::tpm::properties::Properties,
    routines::setup::{
        // Body as RsBody, Interior as RsInterior,
        ProgressDebug as RsProgressDebug,
        Time as RsTime,
    },
};

#[pyclass(from_py_object, unsendable)]
#[derive(Clone)]
pub struct ProgressDebug {
    inner: Rc<RefCell<RsProgressDebug>>,
}

#[pymethods]
impl ProgressDebug {
    #[new]
    #[pyo3(signature = (frequency="10", digits_full=0, digits_decimal=3))]
    fn new(frequency: &str, digits_full: usize, digits_decimal: usize) -> Self {
        Self {
            inner: Rc::new(RefCell::new(RsProgressDebug {
                frequency: frequency.to_string(),
                digits_full,
                digits_decimal,
            })),
        }
    }

    #[getter]
    fn frequency(&self) -> String {
        self.inner.borrow().frequency.clone()
    }

    #[setter]
    fn set_frequency(&self, s: &str) {
        self.inner.borrow_mut().frequency = s.to_string();
    }

    #[getter]
    fn digits_full(&self) -> usize {
        self.inner.borrow().digits_full
    }

    #[setter]
    fn set_digits_full(&self, v: usize) {
        self.inner.borrow_mut().digits_full = v;
    }

    #[getter]
    fn digits_decimal(&self) -> usize {
        self.inner.borrow().digits_decimal
    }

    #[setter]
    fn set_digits_decimal(&self, v: usize) {
        self.inner.borrow_mut().digits_decimal = v;
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.inner.borrow())
    }
}

#[pyclass(from_py_object, unsendable)]
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

#[pyclass(from_py_object, unsendable)]
#[derive(Clone)]
pub struct Body {
    // pub inner: Rc<RefCell<RsBody>>,
    // pub surface: Rc<RefCell<RsSurface>>,
    // pub interior: Rc<RefCell<RsInterior>>,
}

#[pymethods]
impl Body {
    #[new]
    #[pyo3(signature = ())]
    // #[pyo3(signature = (mesh=None, state=[[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]], spin_period=0.0, spin_axis=[0.0, 0.0, 1.0], orbit_period=0.0, orbit_axis=[0.0, 0.0, 1.0]))]
    pub fn new(// mesh: Option<crate::py::mesh::Mesh>,
        // state: [[Float; 4]; 4],
        // spin_period: Float,
        // spin_axis: [Float; 3],
        // orbit_period: Float,
        // orbit_axis: [Float; 3],
    ) -> Self {
        Self {
            // inner: Rc::new(RefCell::new(RsBody {
            //     mesh: mesh.unwrap_or(crate::py::mesh::Mesh::empty()),
            //     interior: RsInterior::Column(vec![]),
            //     state: Mat4::from_cols_array_2d(&state),
            //     spin_period,
            //     spin_axis: spin_axis.into(),
            //     orbit_period,
            //     orbit_axis: orbit_axis.into(),
            // })),
        }
    }

    /*
    #[getter]
    fn mesh<'py>(&'py self, py: Python<'py>) -> PyResult<Py<crate::mesh::Mesh>> {
        let mesh = crate::mesh::Mesh {
            inner: Rc::clone(&self.mesh),
        };
        Py::new(py, mesh)
    }

    #[setter]
    fn set_mesh(&mut self, mesh: PyRef<crate::mesh::Mesh>) {
        self.inner.borrow_mut().mesh = Rc::clone(&mesh);
    }
    */

    /*
    // Getter that allows numpy.ndarray read and write.
    #[getter]
    fn state<'py>(slf: Bound<'py, Self>) -> Bound<'py, numpy::PyArray2<Float>> {
        let inner = &slf.borrow().inner;
        let matref = &inner.borrow().state;
        let arr = ndarray::ArrayView::from(matref.as_ref());
        let arr2d = arr.into_shape_with_order((4, 4)).unwrap();
        unsafe { numpy::PyArray2::borrow_from_array(&arr2d, slf.into_any()) }
    }

    // Setter that allows shorthand operators.
    #[setter]
    fn set_state(&self, state: [[Float; 4]; 4]) {
        self.inner.borrow_mut().state = Mat4::from_cols_array_2d(&state);
    }

    #[getter]
    fn spin_period(&self) -> Float {
        self.inner.borrow().spin_period
    }

    #[setter]
    fn set_spin_period(&self, period: Float) {
        self.inner.borrow_mut().spin_period = period;
    }

    // Getter that allows numpy.ndarray read and write.
    #[getter]
    fn spin_axis<'py>(slf: Bound<'py, Self>) -> Bound<'py, numpy::PyArray1<Float>> {
        let inner = &slf.borrow().inner;
        let slice = &inner.borrow().spin_axis;
        let arr = ndarray::ArrayView1::from(slice.as_ref());
        unsafe { numpy::PyArray1::borrow_from_array(&arr, slf.into_any()) }
    }

    // Setter that allows shorthand operators.
    #[setter]
    fn set_spin_axis(&self, axis: [Float; 3]) {
        self.inner.borrow_mut().spin_axis = axis.into();
    }

    #[getter]
    fn orbit_period(&self) -> Float {
        self.inner.borrow().orbit_period
    }

    #[setter]
    fn set_orbit_period(&self, period: Float) {
        self.inner.borrow_mut().orbit_period = period;
    }

    // Getter that allows numpy.ndarray read and write.
    #[getter]
    fn orbit_axis<'py>(slf: Bound<'py, Self>) -> Bound<'py, numpy::PyArray1<Float>> {
        let inner = &slf.borrow().inner;
        let slice = &inner.borrow().orbit_axis;
        let arr = ndarray::ArrayView1::from(slice.as_ref());
        unsafe { numpy::PyArray1::borrow_from_array(&arr, slf.into_any()) }
    }

    // Setter that allows shorthand operators.
    #[setter]
    fn set_orbit_axis(&self, axis: [Float; 3]) {
        self.inner.borrow_mut().orbit_axis = axis.into();
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.inner.borrow())
    }
    */
}

/*
impl std::fmt::Debug for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner.borrow())
    }
}
*/

#[pyclass(unsendable)]
pub struct BodyDataMap {
    temperatures: Vec<Array1<Float>>,
    thermal_properties_all: usize,
    thermal_properties_map: Vec<(usize, usize)>,
    // TODO: ADD RECORD
}

#[pymethods]
impl BodyDataMap {
    #[new]
    #[pyo3(signature = (temperatures, thermal_properties_all, thermal_properties_map))]
    fn new(
        temperatures: Vec<numpy::PyReadonlyArray1<Float>>,
        thermal_properties_all: usize,
        thermal_properties_map: Vec<(usize, usize)>,
    ) -> Self {
        Self {
            temperatures: temperatures
                .iter()
                .map(|a| a.to_owned_array())
                .collect::<Vec<_>>(),
            thermal_properties_all,
            thermal_properties_map,
        }
    }

    #[getter]
    fn temperatures<'py>(&'py self, py: Python<'py>) -> Vec<Bound<'py, numpy::PyArray1<Float>>> {
        self.temperatures
            .iter()
            .map(|arr| arr.to_pyarray(py))
            .collect::<Vec<_>>()
    }

    #[setter]
    fn set_temperatures(&mut self, v: Vec<numpy::PyReadonlyArray1<Float>>) -> PyResult<()> {
        self.temperatures = v.iter().map(|arr| arr.to_owned_array()).collect::<Vec<_>>();
        Ok(())
    }

    #[getter]
    fn thermal_properties_all(&self) -> usize {
        self.thermal_properties_all
    }

    #[setter]
    fn set_thermal_properties_all(&mut self, v: usize) {
        self.thermal_properties_all = v;
    }

    #[getter]
    fn thermal_properties_map(&self) -> Vec<(usize, usize)> {
        self.thermal_properties_map.clone()
    }

    #[setter]
    fn set_thermal_properties_map(&mut self, v: Vec<(usize, usize)>) {
        self.thermal_properties_map = v;
    }

    pub fn __repr__(&self) -> String {
        format!(
            "BodyDataMap(temperatures={:?}, thermal_properties_all={}, thermal_properties_map={:?})",
            &self.temperatures, self.thermal_properties_all, self.thermal_properties_map,
        )
    }
}

#[pyclass(unsendable)]
pub struct Setup {
    pub sun_position: RefCell<[Float; 3]>,

    #[pyo3(get, set)]
    pub thermal_properties: Vec<Properties>,

    // Vec<Body>
    // pub bodies: Py<PyList>,
    // bodies: PyList::new(py, Vec::<Body>::new()).unwrap().into(),
    // self.bodies.bind(py).iter().join(", "),
    #[pyo3(get, set)]
    pub bodies: Vec<Body>,

    #[pyo3(get, set)]
    pub bodies_data_map: Vec<Py<BodyDataMap>>,

    pub progress_debug: Rc<RefCell<RsProgressDebug>>,

    pub time: Rc<RefCell<RsTime>>,
}

#[pymethods]
impl Setup {
    #[new]
    fn new() -> Self {
        Self {
            sun_position: RefCell::new([0.0; 3]),
            thermal_properties: vec![],
            bodies: vec![],
            bodies_data_map: vec![],
            progress_debug: Rc::new(RefCell::new(RsProgressDebug::new())),
            time: Rc::new(RefCell::new(RsTime::new())),
        }
    }

    #[getter]
    fn sun_position<'py>(&'py self, py: Python<'py>) -> Bound<'py, numpy::PyArray1<Float>> {
        numpy::PyArray1::from_slice(py, self.sun_position.borrow().as_slice())
    }

    #[setter]
    fn set_sun_position(&self, v: numpy::PyReadonlyArray1<Float>) -> PyResult<()> {
        self.sun_position
            .borrow_mut()
            .copy_from_slice(v.as_slice().unwrap());
        Ok(())
    }

    #[getter]
    fn progress_debug<'py>(&'py self, py: Python<'py>) -> PyResult<Py<ProgressDebug>> {
        let progress_debug = ProgressDebug {
            inner: Rc::clone(&self.progress_debug),
        };
        Py::new(py, progress_debug)
    }

    #[setter]
    fn set_progress_debug(&mut self, progress_debug: PyRef<ProgressDebug>) {
        self.progress_debug = Rc::clone(&progress_debug.inner);
    }

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
            // "Setup(sun_position={:?}, thermal_properties={:?}, bodies={:?}, bodies_data_map={:?}, progress_debug={:?}, time={:?})",
            "Setup(sun_position={:?}, thermal_properties={:?}, bodies=???, bodies_data_map={:?}, progress_debug={:?}, time={:?})",
            self.sun_position.borrow(),
            self.thermal_properties,
            // self.bodies,
            self.bodies_data_map,
            self.progress_debug.borrow(),
            self.time.borrow()
        )
    }
}
