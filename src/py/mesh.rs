use std::{cell::RefCell, rc::Rc};

use image::GenericImageView;
use pyo3::prelude::*;

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

fn vertex_matrix<'a>(mesh: &'a crate::mesh::Mesh) -> ndarray::ArrayView2<'a, f64> {
    let slice: &[f64] = bytemuck::cast_slice(&mesh.vertices);

    ndarray::ArrayView1::from(slice)
        .into_shape_with_order((mesh.vertices.len(), crate::mesh::VERTEX_STRIDE))
        .unwrap()
}

fn vertex_matrix_array<'a>(
    slf: Bound<'_, Mesh>,
    start: usize,
    size: usize,
) -> Bound<'_, numpy::PyArray2<f64>> {
    let mesh = slf.borrow();
    let mesh = mesh.inner.borrow();
    let arr = vertex_matrix(&mesh);
    let arr = arr.slice(ndarray::s![.., start..start + size]);
    unsafe { numpy::PyArray2::borrow_from_array(&arr, slf.into_any()) }
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

    #[getter]
    fn vertices(&self) -> VerticesView {
        VerticesView {
            mesh: self.inner.clone(),
        }
    }

    #[getter]
    fn indices(slf: Bound<'_, Self>) -> Bound<'_, numpy::PyArray1<u32>> {
        let inner = &slf.borrow().inner;
        let slice = &inner.borrow().indices;
        let arr = ndarray::ArrayView1::from(slice);
        unsafe { numpy::PyArray1::borrow_from_array(&arr, slf.into_any()) }
    }

    #[getter]
    fn facets(&self) -> FacetsView {
        FacetsView {
            mesh: self.inner.clone(),
        }
    }

    #[getter]
    fn positions(slf: Bound<'_, Self>) -> Bound<'_, numpy::PyArray2<f64>> {
        vertex_matrix_array(slf, crate::mesh::POS_OFFSET, 3)
    }

    #[getter]
    fn texs(slf: Bound<'_, Self>) -> Bound<'_, numpy::PyArray2<f64>> {
        vertex_matrix_array(slf, crate::mesh::TEX_OFFSET, 2)
    }

    #[getter]
    fn normals(slf: Bound<'_, Self>) -> Bound<'_, numpy::PyArray2<f64>> {
        vertex_matrix_array(slf, crate::mesh::NORMAL_OFFSET, 3)
    }

    #[getter]
    fn tangents(slf: Bound<'_, Self>) -> Bound<'_, numpy::PyArray2<f64>> {
        vertex_matrix_array(slf, crate::mesh::TANGENT_OFFSET, 3)
    }

    #[getter]
    fn bitangents(slf: Bound<'_, Self>) -> Bound<'_, numpy::PyArray2<f64>> {
        vertex_matrix_array(slf, crate::mesh::BITANGENT_OFFSET, 3)
    }

    #[getter]
    fn colors(slf: Bound<'_, Self>) -> Bound<'_, numpy::PyArray2<f64>> {
        vertex_matrix_array(slf, crate::mesh::COLOR_OFFSET, 3)
    }

    #[getter]
    fn color_modes(slf: Bound<'_, Self>) -> Bound<'_, numpy::PyArray1<u32>> {
        let start = crate::mesh::COLOR_MODE_OFFSET * 2;
        let size = 1;

        let mesh = slf.borrow();
        let mesh = mesh.inner.borrow();

        let slice: &[u32] = bytemuck::cast_slice(&mesh.vertices);
        let arr = ndarray::ArrayView1::from(slice)
            .into_shape_with_order((mesh.vertices.len(), crate::mesh::VERTEX_STRIDE * 2))
            .unwrap();

        let arr = arr.slice(ndarray::s![.., start..start + size]);
        let arr = arr.flatten();
        unsafe { numpy::PyArray1::borrow_from_array(&arr, slf.into_any()) }
    }

    pub fn __repr__(&self) -> String {
        format!("{:?}", self.inner.borrow())
    }
}

crate::impl_mesh_view!(VerticesView, VertexView, vertices);
crate::impl_mesh_view!(FacetsView, FacetView, facets);

crate::impl_mesh_field_vec!(VertexView, vertices, pos);
crate::impl_mesh_field_vec!(VertexView, vertices, tex);
crate::impl_mesh_field_vec!(VertexView, vertices, normal);
crate::impl_mesh_field_vec!(VertexView, vertices, tangent);
crate::impl_mesh_field_vec!(VertexView, vertices, bitangent);
crate::impl_mesh_field_vec!(VertexView, vertices, color);
crate::impl_mesh_field_scalar!(VertexView, vertices, color_mode, u32);

crate::impl_mesh_field_vec!(FacetView, facets, pos);
crate::impl_mesh_field_vec!(FacetView, facets, normal);