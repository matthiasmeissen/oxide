#[derive(Clone, Copy, Debug, PartialEq)]
pub struct State {
    pub time: f64,
    pub resolution: [f32; 2],
    pub values: [f64; 8],
}

impl Default for State {
    fn default() -> Self {
        Self { 
            time: 0.0, 
            resolution: [0.0, 0.0], 
            values: [0.5; 8] 
        }
    }
}

pub enum Message {
    SetTime(f64),
    SetResolution(f32, f32),
    SetValue(usize, f64),
    MidiInput(MidiMessage),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MidiMessage {
    NoteOn { note: u8, velocity: u8 },
    NoteOff { note: u8 },
    ControlChange { controller: u8, value: u8 },
}