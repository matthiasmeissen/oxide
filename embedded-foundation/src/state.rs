
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

    pub fn update_values(&mut self) {
        self.values[0] = (self.time * 0.2) % 1.0;
        self.values[1] = (self.time * 1.4) % 1.0;
        self.values[2] = (self.time * 0.4) % 1.0;
        self.values[3] = (self.time * 1.8) % 1.0;
    }
}
