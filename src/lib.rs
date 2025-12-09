pub mod astro;
pub mod gpu;
pub mod math;
pub mod mesh;
pub mod python;
pub mod spice;
pub mod tpm;
pub mod util;

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

// #[pymodule]
// #[pyo3(name = "_rs")]
// pub mod python {
//     use pyo3::prelude::*;
//
//     #[pymodule]
//     pub mod util {
//
//         #[pymodule_export]
//         pub const HOUR: f64 = crate::util::HOUR;
//
//         #[pymodule_export]
//         pub const DAY: f64 = crate::util::DAY;
//
//         #[pymodule_export]
//         pub const DPR: f64 = crate::util::DPR;
//
//         #[pymodule_export]
//         pub const RPD: f64 = crate::util::RPD;
//
//         #[pymodule_export]
//         pub const AU: f64 = crate::util::AU;
//     }
//
//     #[pymodule]
//     pub mod math {
//         #[pymodule_export]
//         pub use crate::math::py::{cosine_angle_vectors, cosine_incidence};
//     }
//
//     #[pymodule]
//     pub mod spice {
//         #[pymodule_export]
//         pub use crate::spice::{Body, Camera, Spacecraft};
//
//         // static BODIES: Vec<Body> = crate::spice::BODIES.lock().unwrap().to_vec();
//     }
//
//     #[pymodule]
//     pub mod mesh {
//         #[pymodule_export]
//         pub use crate::mesh::{
//             Facet, Mesh, Vertex,
//             py::{area_facet, normal_facet},
//             view_factor_scalar, view_factor_scalar_with_area,
//         };
//     }
//
//     #[pymodule]
//     pub mod astro {}
//
//     #[pymodule]
//     pub mod tpm {
//         use pyo3::prelude::*;
//
//         #[pymodule]
//         pub mod core {
//             #[pymodule_export]
//             pub use crate::tpm::core::{
//                 py::{conduction_1d, newton_method},
//                 stability, stability_maxdt,
//             };
//         }
//
//         #[pymodule]
//         pub mod properties {
//             #[pymodule_export]
//             pub use crate::tpm::properties::{conductivity, diffusivity};
//         }
//
//         #[pymodule]
//         pub mod emit {
//             #[pymodule_export]
//             pub use crate::tpm::emit::{planck, planck_photon_count, py::radiance};
//         }
//     }
//
//     #[pymodule]
//     pub mod gpu {}
// }
