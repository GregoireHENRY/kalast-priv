use numpy::ndarray::ArrayView1;
use pyo3::prelude::*;

use crate::util;

#[pyfunction]
pub fn planck(t: f64, w: f64) -> f64 {
    // t: temperature (K)
    // w: wavelength (m)
    //
    // output spectral radiance (W/m3/sr)
    util::TWO_HC2 / (w.powi(5) * ((util::HC_PER_K / (t * w)).exp() - 1.0))
}

#[pyfunction]
pub fn planck_photon_count(t: f64, w: f64) -> f64 {
    // t: temperature (K)
    // w: wavelength (m)
    util::TWO_C / (w.powi(4) * ((util::HC_PER_K / (t * w)).exp() - 1.0))
}

#[pyfunction]
pub fn spectral_radiance(f: f64, e: f64, cose: f64, r: f64) -> f64 {
    // f: planck radiation (W/m3/sr)
    // e: spectral emissivity
    // cose: cosine of emission angle
    // r: roughness correction
    f * e * cose * r
}

#[pyfunction]
pub fn steradian(a: f64, d: f64) -> f64 {
    // a: area (m2)
    // d: distance (m)
    a / d.powi(2)
}

#[pyfunction]
pub fn irradiance(f: f64, sr: f64) -> f64 {
    // f: radiance (W/m2/sr) or spectral radiance (W/m3/sr)
    // sr: steradian
    f * sr
}

pub fn radiance(f: f64, r: ArrayView1<f64>, w: ArrayView1<f64>) -> f64 {
    // f: spectral radiance (W/m3/sr)
    // r: response function (filters, transparency, ...)
    // w: wavelength (m)
    let y = f * &r;
    crate::math::simpson_1_3(y.view(), w)
}

#[pyfunction]
pub fn reflectance(w: f64, a: f64, area: f64, cose: f64, d: f64, r: f64) -> f64 {
    // w: wavelength (m)
    // a: albedo
    // area reflecting
    // cose: cosine of emission angle
    // d: distance from reflecting to target
    // r: distance from sun to reflecting target
    planck(util::TEMP_SUN, w) * a * area * cose / (d * d * r * r)
}

// fn_f_sun = lambda x: planck(TEMP_SUN, x) * pi * RADIUS_SUN * RADIUS_SUN / (AU * AU)
// S, _err = scipy.integrate.quad(fn_f_sun, 1e-10, 1e-2)
// print(S)
// = solar constant

pub(crate) mod py {
    use numpy::PyReadonlyArray1;
    use pyo3::prelude::*;

    #[pyfunction]
    pub fn radiance(
        f: f64,
        r: PyReadonlyArray1<'_, f64>,
        w: PyReadonlyArray1<'_, f64>,
    ) -> PyResult<f64> {
        Ok(super::radiance(f, r.as_array(), w.as_array()))
    }
}
