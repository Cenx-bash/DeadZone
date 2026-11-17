#[derive(Clone)]
pub struct Settings {
    pub master_volume: f32,
    pub sensitivity: f32,
    pub fov: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            master_volume: 0.7,
            sensitivity: 0.0035,
            fov: 75.0,
        }
    }
}
