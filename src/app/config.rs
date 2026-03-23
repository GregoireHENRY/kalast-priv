pub struct Config {
    pub width: u32,
    pub height: u32,
    
    pub background_color: wgpu::Color,

    pub debug_app: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            width: 1024,
            height: 768,
            
            background_color: wgpu::Color::BLACK,

            debug_app: false,
        }
    }
}
