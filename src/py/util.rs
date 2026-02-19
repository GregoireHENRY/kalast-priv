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
