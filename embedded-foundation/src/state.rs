
pub struct PlaceholderState {
    pub time: f64,
    pub resolution: [f32; 2],
    pub values: [f64; 8],
    pub shader_index: usize,
}

impl PlaceholderState {
    pub fn new() -> Self {
        Self { 
            time: 0.0, 
            resolution: [1920.0, 1080.0], 
            values: [0.5; 8], 
            shader_index: 0 
        }
    }

    pub fn update_time(&mut self, val: f64) {
        self.time = (self.time + val) % 1.0;
    }
}
