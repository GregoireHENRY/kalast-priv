pub mod astro;
pub mod gpu;
pub mod math;
pub mod mesh;
pub mod spice;
pub mod tpm;
pub mod util;

use std::sync::Mutex;

use once_cell::sync::Lazy;
use pyo3::prelude::*;

pub type UVec2 = glam::USizeVec2;

#[cfg(feature = "calc_f32")]
pub type Float = f32;
#[cfg(not(feature = "calc_f32"))]
pub type Float = f64;

#[cfg(feature = "calc_f32")]
pub const PI: Float = std::f32::consts::PI;
#[cfg(not(feature = "calc_f32"))]
pub const PI: Float = std::f64::consts::PI;

#[cfg(feature = "calc_f32")]
pub type Vec2 = glam::Vec2;
#[cfg(not(feature = "calc_f32"))]
pub type Vec2 = glam::DVec2;

#[cfg(feature = "calc_f32")]
pub type Vec3 = glam::Vec3;
#[cfg(not(feature = "calc_f32"))]
pub type Vec3 = glam::DVec3;

#[cfg(feature = "calc_f32")]
pub type Vec4 = glam::Vec4;
#[cfg(not(feature = "calc_f32"))]
pub type Vec4 = glam::DVec4;

#[cfg(feature = "calc_f32")]
pub type Mat3 = glam::Mat3;
#[cfg(not(feature = "calc_f32"))]
pub type Mat3 = glam::DMat3;

#[cfg(feature = "calc_f32")]
pub type Mat4 = glam::Mat4;
#[cfg(not(feature = "calc_f32"))]
pub type Mat4 = glam::DMat4;

#[cfg(feature = "calc_f32")]
pub type Quat = glam::Quat;
#[cfg(not(feature = "calc_f32"))]
pub type Quat = glam::DQuat;

#[macro_export]
macro_rules! pyadd_c {
    ($m:expr, $p:ident $(::$c:tt)+) => {
        // 1) receive `pyadd_c!(crate::tpm::core::NEWTON_METHOD_MAX_ITERATION);`
        // split $p and send to 2)
        pyadd_c!($m, $p::, $($c),*);
    };

    ($m:expr, $($p:ident::)+, $head:ident, $($tail:ident),+) => {
        // 2) receive $p + a split version of it.
        // The first time it isolate the head `tpm` and call 2) again.
        // The second time it isolate the new head `core` and only one tail so it will call 3).
        pyadd_c!($m, $($p::)* $head::, $($tail),*);
    };

    ($m:expr, $($p:ident::)+ , $c:ident) => {
        // 3) this is called when $method is NEWTON_METHOD_MAX_ITERATION and $module is the full
        // path before.
        $m.add(stringify!($c), $($p::)+ $c)?;
    };
}

#[macro_export]
macro_rules! pyadd_f {
    ($m:expr, $f:path) => {
        $m.add_function(wrap_pyfunction!($f, &$m)?)?;
    };
}

pub fn pyadd_c_lazy<'py, T>(m: &Bound<'py, PyModule>, name: &str, lazy_data: &Lazy<Mutex<T>>)
where
    T: Clone + std::fmt::Debug + IntoPyObject<'py>,
{
    let locked = lazy_data.lock().unwrap();
    m.add(name, locked.clone()).unwrap();
}

/// This module is implemented in Rust.
#[pymodule]
#[pyo3(name = "_rs")]
fn kalast(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    let util = PyModule::new(m.py(), "util")?;
    pyadd_c!(util, crate::util::HOUR);
    pyadd_c!(util, crate::util::DAY);
    pyadd_c!(util, crate::util::DPR);
    pyadd_c!(util, crate::util::RPD);
    pyadd_c!(util, crate::util::AU);
    pyadd_c!(util, crate::util::SOLAR_CONSTANT);
    pyadd_c!(util, crate::util::STEFAN_BOLTZMANN);
    pyadd_c!(util, crate::util::PLANK_CONSTANT);
    pyadd_c!(util, crate::util::SPEED_LIGHT);
    pyadd_c!(util, crate::util::BOLTZMANN_CONSTANT);
    pyadd_c!(util, crate::util::TWO_C);
    pyadd_c!(util, crate::util::HC);
    pyadd_c!(util, crate::util::HC2);
    pyadd_c!(util, crate::util::HC_PER_K);
    pyadd_c!(util, crate::util::TWO_HC2);
    pyadd_c!(util, crate::util::TEMP_SUN);
    pyadd_c!(util, crate::util::RADIUS_SUN);
    pyadd_c!(util, crate::util::JANSKY);
    pyadd_c!(util, crate::util::BAND_V0);
    pyadd_c!(util, crate::util::GRAVITATIONAL_CONSTANT);
    pyadd_c!(util, crate::util::MASS_SUN);
    pyadd_c!(util, crate::util::NEWTON_METHOD_MAX_ITERATION);
    pyadd_c!(util, crate::util::NEWTON_METHOD_THRESHOLD);
    m.add_submodule(&util)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("kalast._rs.util", util)?;

    let math = PyModule::new(m.py(), "math")?;
    pyadd_f!(math, crate::math::py::cosine_angle_vectors);
    pyadd_f!(math, crate::math::py::cosine_incidence);
    pyadd_f!(math, crate::math::py::flattening_radius);
    pyadd_f!(math, crate::math::py::trapez);
    pyadd_f!(math, crate::math::py::simpson_1_3);
    pyadd_f!(math, crate::math::py::simpson_3_8);
    pyadd_f!(math, crate::math::py::boole);
    m.add_submodule(&math)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("kalast._rs.math", math)?;

    let spice = PyModule::new(m.py(), "spice")?;
    spice.add("BODIES", crate::spice::BODIES.lock().unwrap().clone())?;
    spice.add("CAMERAS", crate::spice::CAMERAS.lock().unwrap().clone())?;
    spice.add("SPACECRAFTS", crate::spice::SPACECRAFTS.lock().unwrap().clone())?;
    spice.add_class::<crate::spice::Body>()?;
    spice.add_class::<crate::spice::Camera>()?;
    spice.add_class::<crate::spice::Spacecraft>()?;
    m.add_submodule(&spice)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("kalast._rs.spice", spice)?;

    let mesh = PyModule::new(m.py(), "mesh")?;
    pyadd_f!(mesh, crate::mesh::py::load_image);
    pyadd_f!(mesh, crate::mesh::py::normal_facet);
    pyadd_f!(mesh, crate::mesh::py::area_facet);
    pyadd_f!(mesh, crate::mesh::py::is_point_in_or_on);
    pyadd_f!(mesh, crate::mesh::py::is_point_in_or_on_triangle);
    pyadd_f!(mesh, crate::mesh::py::is_facing_plane);
    pyadd_f!(mesh, crate::mesh::py::is_not_parallel_to_plane);
    pyadd_f!(mesh, crate::mesh::py::intersect_plane);
    pyadd_f!(mesh, crate::mesh::py::intersect_triangle);
    pyadd_f!(mesh, crate::mesh::py::intersect_triangle_moller_trumblore);
    pyadd_f!(mesh, crate::mesh::py::intersect_mesh);
    pyadd_f!(mesh, crate::mesh::view_factor_scalar_with_area);
    pyadd_f!(mesh, crate::mesh::view_factor_scalar);
    pyadd_f!(mesh, crate::mesh::py::view_factor_facets);
    pyadd_f!(mesh, crate::mesh::largest_slope_angle_sphere);
    pyadd_f!(mesh, crate::mesh::curvature_radius);
    pyadd_f!(mesh, crate::mesh::curvature_diameter_from_radius);
    pyadd_f!(mesh, crate::mesh::curvature_diameter_sphere);
    pyadd_f!(mesh, crate::mesh::z_in_crater);
    pyadd_f!(mesh, crate::mesh::rms_slope);
    pyadd_f!(mesh, crate::mesh::rms_slope_hemisphere);
    pyadd_f!(mesh, crate::mesh::distribution_slope_angles);
    pyadd_f!(mesh, crate::mesh::py::rms_slope_terrain);
    mesh.add_class::<crate::mesh::Vertex>()?;
    mesh.add_class::<crate::mesh::Facet>()?;
    mesh.add_class::<crate::mesh::Material>()?;
    mesh.add_class::<crate::mesh::Mesh>()?;
    mesh.add_class::<crate::mesh::Model>()?;
    m.add_submodule(&mesh)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("kalast._rs.mesh", mesh)?;

    let astro = PyModule::new(m.py(), "astro")?;
    // nothing yet
    m.add_submodule(&astro)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("kalast._rs.astro", astro)?;

    let tpm = PyModule::new(m.py(), "tpm")?;
    m.add_submodule(&tpm)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("kalast._rs.tpm", &tpm)?;

    let core = PyModule::new(tpm.py(), "core")?;
    pyadd_f!(core, crate::tpm::core::stability);
    pyadd_f!(core, crate::tpm::core::stability_maxdt);
    pyadd_f!(core, crate::tpm::core::conduction);
    pyadd_f!(core, crate::tpm::core::effective_temperature);
    pyadd_f!(core, crate::tpm::core::radiation_sun);
    pyadd_f!(core, crate::tpm::core::radiation_sun_reflected);
    pyadd_f!(core, crate::tpm::core::radiation_sun_reflected_reuse);
    pyadd_f!(core, crate::tpm::core::radiation_emitted);
    pyadd_f!(core, crate::tpm::core::newton_method_fn);
    pyadd_f!(core, crate::tpm::core::newton_method_dfn);
    pyadd_f!(core, crate::tpm::core::py::newton_method);
    pyadd_f!(core, crate::tpm::core::py::conduction_1d);
    tpm.add_submodule(&core)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("kalast._rs.tpm.core", core)?;

    let properties = PyModule::new(tpm.py(), "properties")?;
    pyadd_c!(properties, crate::tpm::properties::DIDYMOS);
    pyadd_c!(properties, crate::tpm::properties::DIMORPHOS);
    pyadd_c!(properties, crate::tpm::properties::MOON);
    pyadd_c!(properties, crate::tpm::properties::PHOBOS);
    pyadd_c!(properties, crate::tpm::properties::DEIMOS);
    pyadd_f!(properties, crate::tpm::properties::conductivity);
    pyadd_f!(properties, crate::tpm::properties::diffusivity);
    pyadd_f!(properties, crate::tpm::properties::thermal_inertia);
    pyadd_f!(properties, crate::tpm::properties::skin_depth_1);
    pyadd_f!(properties, crate::tpm::properties::skin_depth_2pi);
    properties.add_class::<crate::tpm::properties::Properties>()?;
    tpm.add_submodule(&properties)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("kalast._rs.tpm.properties", properties)?;

    let emit = PyModule::new(tpm.py(), "emit")?;
    pyadd_f!(emit, crate::tpm::emit::planck);
    pyadd_f!(emit, crate::tpm::emit::planck_photon_count);
    pyadd_f!(emit, crate::tpm::emit::spectral_radiance);
    pyadd_f!(emit, crate::tpm::emit::steradian);
    pyadd_f!(emit, crate::tpm::emit::irradiance);
    pyadd_f!(emit, crate::tpm::emit::reflectance);
    pyadd_f!(emit, crate::tpm::emit::py::radiance);
    tpm.add_submodule(&emit)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("kalast._rs.tpm.emit", emit)?;

    let gpu = PyModule::new(m.py(), "gpu")?;
    m.add_submodule(&gpu)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("kalast._rs.gpu", &gpu)?;

    let compute = PyModule::new(gpu.py(), "compute")?;
    pyadd_f!(compute, crate::gpu::compute::run);
    gpu.add_submodule(&compute)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("kalast._rs.gpu.compute", compute)?;

    let win = PyModule::new(gpu.py(), "win")?;
    // pyadd_f!(win, crate::gpu::win::py_run);
    // pyadd_f!(win, crate::gpu::win::py_create);
    // win.add_class::<crate::gpu::win::PyApp>()?;
    // win.add_class::<crate::gpu::win::State>()?;
    win.add_class::<crate::gpu::win::StateStep>()?;
    gpu.add_submodule(&win)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("kalast._rs.gpu.win", win)?;

    let scene = PyModule::new(gpu.py(), "scene")?;
    scene.add_class::<crate::gpu::scene::ModelState>()?;
    gpu.add_submodule(&scene)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("kalast._rs.gpu.scene", scene)?;

    let config = PyModule::new(gpu.py(), "config")?;
    config.add_class::<crate::gpu::config::Config>()?;
    config.add_class::<crate::gpu::config::ConfigModel>()?;
    config.add_class::<crate::gpu::config::ConfigText>()?;
    gpu.add_submodule(&config)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item("kalast._rs.gpu.config", config)?;

    Ok(())
}

// need to adds functions: view factor, ray interception, shadowing
// need to plug viewer
