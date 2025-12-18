pub mod astro;
pub mod gpu;
pub mod math;
pub mod mesh;
pub mod py;
pub mod spice;
pub mod tpm;
pub mod util;
pub mod routines;

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
