pub const HOUR: f64 = 3600.0;
pub const DAY: f64 = 24.0 * HOUR;

/// degrees per radian
pub const DPR: f64 = 180.0 / std::f64::consts::PI;
/// radians per degree
pub const RPD: f64 = 1.0 / DPR;

pub const AU: f64 = 1.495978707e11;

// integrated solar flux at 1 AU (W/m2)
pub const SOLAR_CONSTANT: f64 = 1369.0;

pub const STEFAN_BOLTZMANN: f64 = 5.670374419e-8;

pub const PLANK_CONSTANT: f64 = 6.62607015e-34;
pub const SPEED_LIGHT: f64 = 299792458.0;
pub const BOLTZMANN_CONSTANT: f64 = 1.380649e-23;
pub const TWO_C: f64 = 2.0 * SPEED_LIGHT;
pub const HC: f64 = PLANK_CONSTANT * SPEED_LIGHT;
pub const HC2: f64 = HC * SPEED_LIGHT;
pub const HC_PER_K: f64 = HC / BOLTZMANN_CONSTANT;
pub const TWO_HC2: f64 = 2.0 * HC2;
pub const TEMP_SUN: f64 = 5778.0;
pub const RADIUS_SUN: f64 = 696340e3;
pub const JANSKY: f64 = 1e26;

/// Zero-point flux in the Johnson V-band (W/m3) (see Bessell+ 1998)
pub const BAND_V0: f64 = 3.631e-2;

pub const MASS_SUN: f64 = 1.989e30;
pub const GRAVITATIONAL_CONSTANT: f64 = 6.6743e-11;

pub const NEWTON_METHOD_MAX_ITERATION: usize = 1000;
pub const NEWTON_METHOD_THRESHOLD: f64 = 0.1;

pub const SPICE_PICTUR_1: &str = "YYYY-MM-DD HR:MN ::RND";
pub const SPICE_PICTUR_2: &str = "YYYY-MM-DD ::RND";
pub const SPICE_PICTUR_3: &str = "YYYYMMDDTHRMNSC ::RND";

// incident spectral solar flux at 1 AU at 545 nm
pub const SFLUX_545: f64 = 1896.0;

pub fn bool_to_on_off(b: bool) -> String {
    if b { "ON" } else { "OFF" }.to_string()
}
