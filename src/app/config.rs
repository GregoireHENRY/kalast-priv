use crate::Float;

#[derive(Clone, Debug)]
pub struct Config {
    pub debug_app: bool,
    pub debug_window: bool,
    pub debug_window_mesh: bool,
    pub debug_simulation: bool,

    pub title: String,
    pub width: u32,
    pub height: u32,

    pub background: wgpu::Color,
    pub enable_back_face: bool,

    pub sensitivity_move: Float,
    pub sensitivity_look: Float,
    pub sensitivity_rotate: Float,
    pub sensitivity_zoom: Float,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            debug_app: false,
            debug_window: false,
            debug_window_mesh: false,
            debug_simulation: false,

            title: "kalast".to_string(),
            width: 1024,
            height: 768,

            background: wgpu::Color::BLACK,
            enable_back_face: false,
            
            sensitivity_move: 1.0,
            sensitivity_look: 1.0,
            sensitivity_rotate: 1.0,
            sensitivity_zoom: 1.0,
        }
    }
}
