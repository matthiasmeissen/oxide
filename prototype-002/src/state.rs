#[derive(Clone, Copy, Debug, PartialEq)]
pub struct State {
    pub time: f64,
    pub resolution: [f32; 2],
    pub values: [f32; 8],
    pub shader_index: usize,
    pub fps: f32,
    pub screen_index: usize,
    pub dsp_type: DspType,
}

impl Default for State {
    fn default() -> Self {
        Self { 
            time: 0.0, 
            resolution: [0.0, 0.0], 
            values: [0.5; 8],
            shader_index: 0,
            fps: 0.0,
            screen_index: 0,
            dsp_type: DspType::default(),
        }
    }
}

pub enum Message {
    SetTime(f64),
    SetResolution(f32, f32),
    SetValue(usize, f32),
    MidiInput(MidiMessage),
    SetShaderIndex(usize),
    SetFps(f32),
    SetScreenIndex(usize),
    SetDspType(DspType),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MidiMessage {
    NoteOn { note: u8, velocity: u8 },
    NoteOff { note: u8 },
    ControlChange { controller: u8, value: u8 },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DspType {
    SimpleSine,
    BasicFm,
    DrumEngine,
}

impl Default for DspType {
    fn default() -> Self {
        DspType::BasicFm
    }
}