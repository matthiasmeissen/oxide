#[derive(Clone, Copy, Debug, PartialEq)]
pub struct State {
    pub time: f64,
    pub resolution: [f64; 2],
    pub values: [f64; 4],
}

impl Default for State {
    fn default() -> Self {
        Self { 
            time: 0.0, 
            resolution: [480.0, 320.0], 
            values: [0.5; 4] 
        }
    }
}

pub enum Message {
    SetTime(f64),
    SetResolution(f64, f64),
    SetValue(usize, f64),
}