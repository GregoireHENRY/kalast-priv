// Diffuse solar radiation
//
// Args:
//     view factor between the surface of the two bodies
//     radiation of the Sun on the surface of the other body
//     albedo of the surface of the other body
//
// Out:
//     heat flux (W/m2)
//
// The diffuse solar radiation contribution from all $N$ facets $i$ the other body onto the
// facet $j$ of the body is defined as,
//
// .. math::
// W_{i}=\sum_{\substack{j \\ j\neq i}}^{N}V_{ij}\frac{S_\odot A\cos\varsigma_j\left(t\right)}{r_H^2\left(t\right)}
//
// where $V_{ij}$ is the view factor describing the fraction of energy emitted from one facet
// $i$ towards the facet $j$, $S_\odot$ is [Solar Constant][SOLAR_CONSTANT], $A$ the albedo,
// $\varsigma_j$ the illumination angle of the facet $j$, and $r_H$ the heliocentric distance
// in [AU][ASTRONOMICAL_UNIT].
//
//
// Direct thermal heating
//
// Args:
//     view factor between the surface of the two bodies
//     temperature and emissivity of the surface of the other body
//
// Out:
//     heat flux (W/m2)
//
// Expression:
//     The direct thermal heating contribution from all $N$ facets $i$ of the other body onto the
//     facet $j$ of the body is defined as,
//
//     $$u_{j}=\sum_{i\cancel{=}j}^{N}V_{ij}\epsilon\sigma T_{i}^4$$
//
//     where $V_{ij}$ is the view factor describing the fraction of energy emitted from one facet
//     $i$ towards the facet $j$, $\epsilon$ the emissivity, $\sigma$ the
//     [Stefan-Boltzmann constant][STEFAN_BOLTZMANN], and $T_i$ the temperature of the facet $i$.
//
// units:
// - radiance: W/m2/sr
// - spectral radiance: W/m3/sr
// - irradiance (=flux density): W/m2
// - spectral irradiance: W/m3
//   W/m2/um = W/m3 * 1e-6
//
// Jansky: 1 W/m2/Hz = 1e26 Jy
// 1) convert spectral irradiance from W/m3 to W/m2/Hz
//    with: W/m3 * lamda^2 / speed_light = W/m2/Hz
// 2) Then can apply: W/m2/Hz * JANSKY
//
//
// kirchhoff_law:
//     Emissivity and albedo (directional-hemispherical reflectivity) are simply related.
//     Required to obtain thermal equilibrium and essential to derive Planck spectrum.
//     a = 1 - e

use anyhow::{Result, anyhow};
use numpy::ndarray::{Array1, ArrayView1, s};
use pyo3::prelude::*;

use crate::util;

#[pyfunction]
pub fn stability(d: f64, dt: f64, dx2: f64) -> f64 {
    // Stability coefficient for conduction_1d, lower than 0.5 is converging.
    // Also called Fourier mesh number.
    //
    // d: diffusivity (...)
    // dt: time step (s)
    // dx2: depth step squared (m2)
    d * dt / dx2
}

#[pyfunction]
pub fn stability_maxdt(d: f64, dx2: f64, s: f64) -> f64 {
    // Find largest dt for conduction_1d to be stable considering depth step and diffusivity.
    // s is usually 0.5
    //
    // d: diffusivity (...)
    // dx2: depth step squared (m2)
    // s: stability coef
    s * dx2 / d
}

#[pyfunction]
pub fn conduction(t: f64, f: f64, k: f64, dx: f64) -> f64 {
    // Update temperature from a flux over a distance.
    // Adiabatic is f=0.
    //
    // t: temperature (K)
    // f: heat flux (W/m2)
    // k: conductivity (...)
    // dx: distance (m)
    t + dx * f / k
}

#[pyfunction]
pub fn effective_temperature(dau: f64, r: f64, a: f64, e: f64) -> f64 {
    // dau: distance of Sun is AU
    // r: ratio between areas receiving and emitting
    // a: albedo
    // e: emissivity
    (util::SOLAR_CONSTANT * r * (1.0 - a) / (e * util::STEFAN_BOLTZMANN * dau.powi(2))).powf(0.25)
}

#[pyfunction]
pub fn radiation_sun(dau: f64, cosi: f64, a: f64) -> f64 {
    // dau: distance of Sun is AU
    // cosi: cosine of incidence angle of local surface
    // a: albedo
    util::SOLAR_CONSTANT * (1.0 - a) * cosi / dau.powi(2)
}

#[pyfunction]
pub fn radiation_sun_reflected(viewf: f64, a: f64, cosi: f64, dau: f64) -> f64 {
    // viewf: view-factor of local surface
    // a: albedo
    // cosi: cosine of incidence angle of local surface
    // dau: distance of Sun is AU
    viewf * util::SOLAR_CONSTANT * a * cosi / dau.powi(2)
}

/// care with albedos
#[pyfunction]
pub fn radiation_sun_reflected_reuse(viewf: f64, f: f64, a: f64) -> f64 {
    // viewf: view-factor of local surface
    // f: radiation from sun from another surface
    // a: albedo
    viewf * f * a / (1.0 - a)
}

#[pyfunction]
pub fn radiation_emitted(viewf: f64, t: f64, e: f64) -> f64 {
    // viewf: view-factor of local surface
    // t: temperature (K)
    // e: emissivity
    viewf * util::STEFAN_BOLTZMANN * e * t.powi(4)
}

#[pyfunction]
pub fn newton_method_fn(
    t: f64,
    f: f64,
    set3: f64,
    k: f64,
    subt1: f64,
    subt2: f64,
    twodx: f64,
) -> f64 {
    f - set3 * t + k * (-3.0 * t + 4.0 * subt1 - subt2) / twodx
}

#[pyfunction]
pub fn newton_method_dfn(set3: f64, k: f64, twodx: f64) -> f64 {
    -4.0 * set3 - 3.0 * k / twodx
}

pub fn newton_method(
    mut t: f64,
    f: f64,
    se: f64,
    k: f64,
    subt1: f64,
    subt2: f64,
    twodx: f64,
) -> Result<f64> {
    for _ in 0..util::NEWTON_METHOD_MAX_ITERATION {
        let set3 = se * t.powi(3);
        let fn_ = newton_method_fn(t, f, set3, k, subt1, subt2, twodx);
        let dfn = newton_method_dfn(set3, k, twodx);
        let delta = -fn_ / dfn;
        t += delta;
        if delta.abs() < util::NEWTON_METHOD_THRESHOLD {
            return Ok(t);
        }
    }
    Err(anyhow!("Newton method reached maximum iteration"))
}

pub fn conduction_1d(
    t: ArrayView1<'_, f64>,
    d: ArrayView1<'_, f64>,
    dtpdx2: ArrayView1<'_, f64>,
) -> Array1<f64> {
    let t_mid = t.slice(s![1..-1]);
    &t_mid + &d.slice(s![1..-1]) * &dtpdx2 * (&t.slice(s![..-2]) - 2.0 * &t_mid + &t.slice(s![2..]))
}

pub(crate) mod py {
    use numpy::{PyArray1, PyReadonlyArray1, ToPyArray};
    use pyo3::prelude::*;

    #[pyfunction]
    pub fn newton_method(
        t: f64,
        f: f64,
        se: f64,
        k: f64,
        subt1: f64,
        subt2: f64,
        twodx: f64,
    ) -> PyResult<f64> {
        Ok(super::newton_method(t, f, se, k, subt1, subt2, twodx).unwrap())
    }

    #[pyfunction]
    pub fn conduction_1d<'py>(
        py: Python<'py>,
        t: PyReadonlyArray1<'py, f64>,
        d: PyReadonlyArray1<'py, f64>,
        dtpdx2: PyReadonlyArray1<'_, f64>,
    ) -> Bound<'py, PyArray1<f64>> {
        super::conduction_1d(t.as_array(), d.as_array(), dtpdx2.as_array()).to_pyarray(py)
    }
}
