#[macro_export]
macro_rules! impl_py_attrs_vec {
    ($ty:ident, $( $field:ident : $len:expr ),+ $(,)?) => {
        paste::paste!{
            #[pymethods]
            impl $ty {
                $(
                    #[getter]
                    fn $field<'py>(slf: pyo3::Bound<'py, Self>) -> pyo3::Bound<'py, numpy::PyArray1<Float>> {
                        let inner = &slf.borrow().inner;
                        let slice = &inner.borrow().$field;
                        let arr = ndarray::ArrayView1::from(slice.as_ref());
                        unsafe { numpy::PyArray1::borrow_from_array(&arr, slf.into_any()) }
                    }

                    #[setter]
                    fn [<set_ $field>](&self, arr: [Float; $len]) {
                        self.inner.borrow_mut().$field = arr.into();
                    }
                )+
            }
        }
    };
}

#[macro_export]
macro_rules! impl_mesh_view {
    ($collection_name:ident, $element_name:ident, $field:ident) => {
        #[pyclass(unsendable)]
        pub struct $collection_name {
            mesh: Rc<RefCell<crate::mesh::Mesh>>,
        }

        #[pymethods]
        impl $collection_name {
            fn __len__(&self) -> usize {
                self.mesh.borrow().$field.len()
            }

            fn __getitem__(&self, mut index: isize) -> PyResult<$element_name> {
                let mesh = self.mesh.borrow();
                let n = mesh.$field.len();

                if index < 0 {
                    index += n as isize;
                }
                
                let index = index as usize;

                if index >= n {
                    return Err(pyo3::exceptions::PyIndexError::new_err("out of bounds"));
                }

                Ok($element_name {
                    mesh: self.mesh.clone(),
                    index,
                })
            }

            pub fn __repr__(&self) -> String {
                format!("{:?}", self.mesh.borrow().$field)
            }
        }

        #[pyclass(unsendable)]
        pub struct $element_name {
            pub mesh: Rc<RefCell<crate::mesh::Mesh>>,
            pub index: usize,
        }

        #[pymethods]
        impl $element_name {
            pub fn __repr__(&self) -> String {
                let mesh = self.mesh.borrow();
                format!("{:?}", mesh.$field[self.index])
            }
        }
    };
}

#[macro_export]
macro_rules! impl_mesh_field_vec {
    ($collection_view:ident, $collection:ident, $field:ident) => {
        #[pymethods]
        impl $collection_view {
            #[getter]
            fn $field<'py>(slf: Bound<'py, Self>) -> Bound<'py, numpy::PyArray1<f64>> {
                let this = slf.borrow();
                let mesh = this.mesh.borrow();
                let element = &mesh.$collection[this.index];
                let arr = ndarray::ArrayView1::from(element.$field.as_ref());
                unsafe { numpy::PyArray1::borrow_from_array(&arr, slf.into_any()) }
            }
        }
    };
}

#[macro_export]
macro_rules! impl_mesh_field_scalar {
    ($collection_view:ident, $collection:ident, $field:ident, $ty:ty) => {
        paste::paste! {
            #[pymethods]
            impl $collection_view {
                #[getter]
                fn $field(&self) -> $ty {
                    self.mesh.borrow().$collection[self.index].$field
                }

                #[setter]
                fn [<set_ $field>](&mut self, $field: $ty) {
                    self.mesh.borrow_mut().$collection[self.index].$field = $field;
                }
            }
        }
    };
}
