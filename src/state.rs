use std::default;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct State {
    pub time: f64,
    pub resolution: [f32; 2],
    pub values: [f32; 8],
    pub shader_index: usize,
    pub fps: f32,
    pub dsp_type: DspType,
}

impl Default for State {
    fn default() -> Self {
        Self { 
            time: 0.0, 
            resolution: [0.0, 0.0], 
            values: [0.0; 8],
            shader_index: 0,
            fps: 0.0,
            dsp_type: DspType::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Screen {
    Home,
    HomeSettings { selected_index: usize },
    Shader,
    ShaderSelect { selected_index: usize },
    Audio,
    AudioSelect { selected_index: usize },
}

impl Default for Screen {
    fn default() -> Self {
        Screen::Home
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DisplayState {
    pub app: State,
    pub ui: Screen,
}

impl Default for DisplayState {
    fn default() -> Self {
        Self {
            app: State::default(),
            ui: Screen::default(),
        }
    }
}

pub enum Message {
    SetTime(f64),
    SetResolution(f32, f32),
    SetValue(usize, f32),
    MidiInput(MidiDevice, MidiMessage),
    SetShaderIndex(usize),
    SetFps(f32),
    SetDspType(DspType),
    UiInput(InputEvent),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputEvent {
    Next,
    Prev,
    Enter,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MidiMessage {
    NoteOn { note: u8, velocity: u8 },
    NoteOff { note: u8 },
    ControlChange { controller: u8, value: u8 },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MidiDevice {
    Undefined,
    OPZ,
    LaunchControlXL,
    Deluge,
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
