use std::{cell::RefCell, rc::Rc};

use image::GenericImageView;
use ndarray::s;
use numpy::npyffi::types::npy_intp;
use numpy::{PyArray1, PyArray2, ToPyArray};
use pyo3::{prelude::*, types::PyList};

use memoffset::offset_of;
use numpy::PyArrayMethods;
use numpy::npyffi::objects::PyArrayObject;
use pyo3::ffi;
use std::ptr;

use crate::{Float, Vec3};

#[pyclass(unsendable)]
#[derive(Clone)]
pub struct Vertex {
    pub inner: Rc<RefCell<crate::mesh::Vertex>>,
}

#[pymethods]
impl Vertex {
    #[new]
    #[pyo3(signature = (
        pos=None,
        tex=None,
        normal=None,
        tangent=None,
        bitangent=None,
        color=None,
        color_mode=None,
    ))]
    pub fn new(
        pos: Option<[Float; 3]>,
        tex: Option<[Float; 2]>,
        normal: Option<[Float; 3]>,
        tangent: Option<[Float; 3]>,
        bitangent: Option<[Float; 3]>,
        color: Option<[Float; 3]>,
        color_mode: Option<u32>,
    ) -> Self {
        let mut vertex = crate::mesh::Vertex::default();

        if let Some(pos) = pos {
            vertex.pos = pos.into();
        }
        if let Some(tex) = tex {
            vertex.tex = tex.into();
        }
        if let Some(normal) = normal {
            vertex.normal = normal.into();
        }
        if let Some(tangent) = tangent {
            vertex.tangent = tangent.into();
        }
        if let Some(bitangent) = bitangent {
            vertex.bitangent = bitangent.into();
        }
        if let Some(color) = color {
            vertex.color = color.into();
        }
        if let Some(color_mode) = color_mode {
            vertex.color_mode = color_mode.into();
        }

        Self {
            inner: Rc::new(RefCell::new(vertex)),
        }
    }

    #[getter]
    fn pos<'py>(slf: pyo3::Bound<'py, Self>) -> pyo3::Bound<'py, numpy::PyArray1<Float>> {
        let inner = &slf.borrow().inner;
        let slice = &inner.borrow().pos;
        let arr = ndarray::ArrayView1::from(slice.as_ref());
        unsafe { numpy::PyArray1::borrow_from_array(&arr, slf.into_any()) }
    }

    #[setter]
    fn set_pos(&self, arr: [Float; 3]) {
        self.inner.borrow_mut().pos = arr.into();
    }

    #[getter]
    fn tex<'py>(slf: pyo3::Bound<'py, Self>) -> pyo3::Bound<'py, numpy::PyArray1<Float>> {
        let inner = &slf.borrow().inner;
        let slice = &inner.borrow().tex;
        let arr = ndarray::ArrayView1::from(slice.as_ref());
        unsafe { numpy::PyArray1::borrow_from_array(&arr, slf.into_any()) }
    }

    #[setter]
    fn set_tex(&self, arr: [Float; 2]) {
        self.inner.borrow_mut().tex = arr.into();
    }

    #[getter]
    fn normal<'py>(slf: pyo3::Bound<'py, Self>) -> pyo3::Bound<'py, numpy::PyArray1<Float>> {
        let inner = &slf.borrow().inner;
        let slice = &inner.borrow().normal;
        let arr = ndarray::ArrayView1::from(slice.as_ref());
        unsafe { numpy::PyArray1::borrow_from_array(&arr, slf.into_any()) }
    }

    #[setter]
    fn set_normal(&self, arr: [Float; 3]) {
        self.inner.borrow_mut().normal = arr.into();
    }

    #[getter]
    fn tangent<'py>(slf: pyo3::Bound<'py, Self>) -> pyo3::Bound<'py, numpy::PyArray1<Float>> {
        let inner = &slf.borrow().inner;
        let slice = &inner.borrow().tangent;
        let arr = ndarray::ArrayView1::from(slice.as_ref());
        unsafe { numpy::PyArray1::borrow_from_array(&arr, slf.into_any()) }
    }

    #[setter]
    fn set_tangent(&self, arr: [Float; 3]) {
        self.inner.borrow_mut().tangent = arr.into();
    }

    #[getter]
    fn bitangent<'py>(slf: pyo3::Bound<'py, Self>) -> pyo3::Bound<'py, numpy::PyArray1<Float>> {
        let inner = &slf.borrow().inner;
        let slice = &inner.borrow().bitangent;
        let arr = ndarray::ArrayView1::from(slice.as_ref());
        unsafe { numpy::PyArray1::borrow_from_array(&arr, slf.into_any()) }
    }

    #[setter]
    fn set_bitangent(&self, arr: [Float; 3]) {
        self.inner.borrow_mut().bitangent = arr.into();
    }

    #[getter]
    fn color<'py>(slf: pyo3::Bound<'py, Self>) -> pyo3::Bound<'py, numpy::PyArray1<Float>> {
        let inner = &slf.borrow().inner;
        let slice = &inner.borrow().color;
        let arr = ndarray::ArrayView1::from(slice.as_ref());
        unsafe { numpy::PyArray1::borrow_from_array(&arr, slf.into_any()) }
    }

    #[setter]
    fn set_color(&self, arr: [Float; 3]) {
        self.inner.borrow_mut().color = arr.into();
    }

    #[getter]
    fn color_mode(&self) -> u32 {
        self.inner.borrow().color_mode
    }

    #[setter]
    fn set_color_mode(&self, mode: u32) {
        self.inner.borrow_mut().color_mode = mode;
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.inner.borrow())
    }
}

impl std::fmt::Debug for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner.borrow())
    }
}

#[pyclass(unsendable)]
#[derive(Clone)]
pub struct Facet {
    pub inner: Rc<RefCell<crate::mesh::Facet>>,
}

#[pymethods]
impl Facet {
    #[new]
    #[pyo3(signature = (
        pos=None,
        normal=None,
        area=None,
    ))]
    pub fn new(pos: Option<[Float; 3]>, normal: Option<[Float; 3]>, area: Option<Float>) -> Self {
        let mut facet = crate::mesh::Facet::default();

        if let Some(pos) = pos {
            facet.pos = pos.into();
        }
        if let Some(normal) = normal {
            facet.normal = normal.into();
        }
        if let Some(area) = area {
            facet.area = area.into();
        }

        Self {
            inner: Rc::new(RefCell::new(facet)),
        }
    }

    #[getter]
    fn pos<'py>(slf: pyo3::Bound<'py, Self>) -> pyo3::Bound<'py, numpy::PyArray1<Float>> {
        let inner = &slf.borrow().inner;
        let slice = &inner.borrow().pos;
        let arr = ndarray::ArrayView1::from(slice.as_ref());
        unsafe { numpy::PyArray1::borrow_from_array(&arr, slf.into_any()) }
    }

    #[setter]
    fn set_pos(&self, arr: [Float; 3]) {
        self.inner.borrow_mut().pos = arr.into();
    }

    #[getter]
    fn normal<'py>(slf: pyo3::Bound<'py, Self>) -> pyo3::Bound<'py, numpy::PyArray1<Float>> {
        let inner = &slf.borrow().inner;
        let slice = &inner.borrow().normal;
        let arr = ndarray::ArrayView1::from(slice.as_ref());
        unsafe { numpy::PyArray1::borrow_from_array(&arr, slf.into_any()) }
    }

    #[setter]
    fn set_normal(&self, arr: [Float; 3]) {
        self.inner.borrow_mut().normal = arr.into();
    }

    #[getter]
    fn area(&self) -> Float {
        self.inner.borrow().area
    }

    #[setter]
    fn set_area(&self, area: Float) {
        self.inner.borrow_mut().area = area;
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.inner.borrow())
    }
}

impl std::fmt::Debug for Facet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner.borrow())
    }
}

#[pyclass(unsendable)]
#[derive(Clone)]
pub struct Material {
    pub inner: Rc<RefCell<crate::mesh::Material>>,
}

#[pymethods]
impl Material {
    pub fn diffuse_bytes(&self) -> PyResult<Vec<u8>> {
        Ok(self.inner.borrow().diffuse.clone().into_bytes())
    }

    pub fn diffuse_dimensions(&self) -> PyResult<(u32, u32)> {
        Ok(self.inner.borrow().diffuse.dimensions())
    }

    pub fn normal_bytes(&self) -> PyResult<Vec<u8>> {
        Ok(self.inner.borrow().normal.clone().into_bytes())
    }

    pub fn normal_dimensions(&self) -> PyResult<(u32, u32)> {
        Ok(self.inner.borrow().normal.dimensions())
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.inner.borrow())
    }
}

impl std::fmt::Debug for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner.borrow())
    }
}

#[pyclass(unsendable)]
pub struct Mesh {
    pub inner: Rc<RefCell<crate::mesh::Mesh>>,
}

#[pymethods]
impl Mesh {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(crate::mesh::Mesh::new())),
        }
    }

    pub fn add_vertex(&self, pos: [f64; 3], normal: [f64; 3]) {
        let mut mesh = self.inner.borrow_mut();
        mesh.vertices.push(crate::mesh::Vertex {
            pos: Vec3::from_array(pos),
            normal: Vec3::from_array(normal),
            ..crate::mesh::Vertex::default()
        });
    }

    fn __len__(&self) -> usize {
        self.inner.borrow().vertices.len()
    }

    fn __getitem__(&self, index: usize) -> PyResult<VertexView> {
        let mesh = self.inner.borrow();
        if index >= mesh.vertices.len() {
            return Err(pyo3::exceptions::PyIndexError::new_err("out of bounds"));
        }

        Ok(VertexView {
            mesh: self.inner.clone(),
            index,
        })
    }

    #[getter]
    fn positions<'py>(slf: Bound<'py, Self>) -> Bound<'py, PyArray2<f64>> {
        let py = slf.py();
        let mesh = slf.borrow();
        let mesh = mesh.inner.borrow();

        let n = mesh.vertices.len();

        let total_f64 = n * 18;

        let ptr = mesh.vertices.as_ptr() as *mut f64;

        let array_1d = unsafe {
            PyArray1::from_raw_parts(
                py,
                total_f64,
                ptr,
                Some(slf.into_any()), // keep mesh alive
            )
        };

        let array_2d = array_1d.reshape([n, 18]).unwrap();

        array_2d.slice(s![.., 0..3])
    }
}

#[pyclass(unsendable)]
pub struct VertexView {
    pub mesh: Rc<RefCell<crate::mesh::Mesh>>,
    pub index: usize,
}

#[pymethods]
impl VertexView {
    #[getter]
    fn pos(&self) -> [f64; 3] {
        let mesh = self.mesh.borrow();
        mesh.vertices[self.index].pos.to_array()
    }

    #[setter]
    fn set_pos(&self, arr: [f64; 3]) {
        let mut mesh = self.mesh.borrow_mut();
        mesh.vertices[self.index].pos = Vec3::from_array(arr);
    }

    #[getter]
    fn normal(&self) -> [f64; 3] {
        let mesh = self.mesh.borrow();
        mesh.vertices[self.index].normal.to_array()
    }

    #[setter]
    fn set_normal(&self, arr: [f64; 3]) {
        let mut mesh = self.mesh.borrow_mut();
        mesh.vertices[self.index].normal = Vec3::from_array(arr);
    }
}

/*
#[pyclass(unsendable)]
#[pyo3(get_all, set_all)]
pub struct Mesh {
    // Vec<Vertex>
    vertices: Py<PyList>,

    // Vec<u32>
    indices: Py<PyList>,

    // Vec<Facet>
    facets: Py<PyList>,

    material_id: Option<usize>,

    // Vec<Vertex>
    _vertices_before_flatten: Py<PyList>,
}

impl Mesh {
    fn from<'py>(py: Python<'py>, mesh: crate::mesh::Mesh) -> Self {
        let crate::mesh::Mesh {
            vertices,
            indices,
            facets,
            material_id,
            ..
        } = mesh;

        Self {
            vertices: PyList::new(
                py,
                vertices
                    .into_iter()
                    .map(|v| Vertex {
                        inner: Rc::new(RefCell::new(v)),
                    })
                    .collect::<Vec<Vertex>>(),
            )
            .unwrap()
            .into(),
            indices: PyList::new(py, indices.into_iter().map(|idx| idx).collect::<Vec<u32>>())
                .unwrap()
                .into(),
            facets: PyList::new(
                py,
                facets
                    .into_iter()
                    .map(|f| Facet {
                        inner: Rc::new(RefCell::new(f)),
                    })
                    .collect::<Vec<Facet>>(),
            )
            .unwrap()
            .into(),
            material_id,
            _vertices_before_flatten: PyList::empty(py).into(),
        }
    }
}

#[pymethods]
impl Mesh {
    #[new]
    #[pyo3(signature = (vertices = None, indices = None, facets = None, material_id = None))]
    fn new(
        py: Python<'_>,
        vertices: Option<Py<PyList>>,
        indices: Option<Py<PyList>>,
        facets: Option<Py<PyList>>,
        material_id: Option<usize>,
    ) -> Self {
        let mut mesh = Self::from(py, crate::mesh::Mesh::new());

        if let Some(vertices) = vertices {
            let _: Vec<Vertex> = vertices.bind(py).extract().unwrap();
            mesh.vertices = vertices;
        }

        if let Some(indices) = indices {
            let _: Vec<u32> = indices.bind(py).extract().unwrap();
            mesh.vertices = indices;
        }

        if let Some(facets) = facets {
            let _: Vec<Facet> = facets.bind(py).extract().unwrap();
            mesh.vertices = facets;
        }

        if let Some(material_id) = material_id {
            mesh.material_id = Some(material_id);
        }

        mesh
    }
}

#[pymethods]
impl Mesh {
    #[staticmethod]
    #[pyo3(signature = (path: "str", update_pos: "Callable[[numpy.array], numpy.array]") -> "None")]
    fn load<'py>(py: Python<'py>, path: &str, update_pos: Py<PyAny>) -> Self {
        let update_pos = |pos: Vec3| {
            update_pos
                .call1(py, (pos.to_array().to_pyarray(py),))
                .unwrap()
                .extract::<[Float; 3]>(py)
                .unwrap()
                .into()
        };

        Self::from(py, crate::mesh::Mesh::load(path, update_pos))
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self)
    }
}

impl std::fmt::Debug for Mesh {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Mesh(vertices={}, indices={}, facets={}, material_id={}",
            self.vertices,
            self.indices,
            self.facets,
            self.material_id
                .map_or("None".to_string(), |id: usize| id.to_string()),
        )
    }
}
*/
