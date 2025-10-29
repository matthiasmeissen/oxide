use crate::state::*;

use std::thread;
use crossbeam_channel::Receiver;
use triple_buffer::*;

const NUM_SHADERS: usize = 3;
const NUM_DSP: usize = 3;

struct Coordinator {
    app_state: State,
    ui_state: Screen
}

impl Coordinator {
    fn run (mut self,
        receiver: Receiver<Message>,
        mut window_writer: Input<State>,
        mut graphics_display_writer: Input<DisplayState>,
        mut oled_display_writer: Input<DisplayState>,
        mut audio_writer: Input<State>,
    ) {
        let mut last_published_app_state = self.app_state;
        let mut last_published_ui_state = self.ui_state;

        while let Ok(msg) = receiver.recv() {
            match msg {
                Message::SetTime(t) => self.app_state.time = t,
                Message::SetResolution(w, h) => self.app_state.resolution = [w, h],
                Message::SetValue(i, v) => self.app_state.values[i] = v,
                Message::SetShaderIndex(i) => self.app_state.shader_index = i,
                Message::SetFps(fps) => self.app_state.fps = fps,
                Message::SetDspType(dt) => self.app_state.dsp_type = dt,
                Message::UiInput(event) => self.handle_ui_input(event),
                Message::MidiInput(device, midi) => self.handle_midi_input(device, midi),
            }

            let app_state_change = self.app_state != last_published_app_state;
            let ui_state_change = self.ui_state != last_published_ui_state;

            if app_state_change || ui_state_change {
                let display_payload = DisplayState {
                    app: self.app_state,
                    ui: self.ui_state,
                };
                graphics_display_writer.write(display_payload);
                oled_display_writer.write(display_payload);
                last_published_ui_state = self.ui_state
            }

            if app_state_change {
                window_writer.write(self.app_state);
                audio_writer.write(self.app_state);
                last_published_app_state = self.app_state;
            }
        }
    }

    fn handle_ui_input(&mut self, event: InputEvent) {
        let current_screen = self.ui_state;

        let next_screen = match current_screen {
            Screen::Home => match event {
                InputEvent::Next => Screen::Shader,
                InputEvent::Prev => Screen::Audio,
                InputEvent::Enter => Screen::HomeSettings {selected_index: 0},
            }
            Screen::HomeSettings { mut selected_index} => match event {
                InputEvent::Next => {println!("Next"); Screen::Home},
                InputEvent::Prev => {println!("Prev"); Screen::Home},
                InputEvent::Enter => Screen::Home,
            }
            Screen::Shader => match event {
                InputEvent::Next => Screen::Audio,
                InputEvent::Prev => Screen::Home,
                InputEvent::Enter => Screen::ShaderSelect {selected_index: self.app_state.shader_index},
            }
            Screen::ShaderSelect { mut selected_index} => match event {
                InputEvent::Next => {
                    selected_index = (selected_index + 1) % NUM_SHADERS;
                    Screen::ShaderSelect { selected_index }
                },
                InputEvent::Prev => {
                    selected_index = (selected_index + NUM_SHADERS - 1) % NUM_SHADERS;
                    Screen::ShaderSelect { selected_index }
                },
                InputEvent::Enter => {
                    self.app_state.shader_index = selected_index;
                    Screen::Home
                },
            }
            Screen::Audio => match event {
                InputEvent::Next => Screen::Home,
                InputEvent::Prev => Screen::Shader,
                InputEvent::Enter => {
                    let current_dsp_index = match self.app_state.dsp_type {
                        DspType::SimpleSine => 0,
                        DspType::BasicFm => 1,
                        DspType::DrumEngine => 2,
                    };
                    Screen::AudioSelect { selected_index: current_dsp_index }
                },
            }
            Screen::AudioSelect { mut selected_index} => match event {
                InputEvent::Next => {
                    selected_index = (selected_index + 1) % NUM_DSP;
                    Screen::AudioSelect { selected_index }
                },
                InputEvent::Prev => {
                    selected_index = (selected_index + NUM_DSP - 1) % NUM_DSP;
                    Screen::AudioSelect { selected_index }
                },
                InputEvent::Enter => {
                    self.app_state.dsp_type = match selected_index {
                        0 => DspType::SimpleSine,
                        1 => DspType::BasicFm,
                        2 => DspType::DrumEngine,
                        _ => self.app_state.dsp_type,
                    };
                    Screen::Home
                },
            }
        };

        self.ui_state= next_screen;
    }

    fn handle_midi_input(&mut self, device: MidiDevice, midi: MidiMessage) {
        match midi {
            MidiMessage::ControlChange { controller, value } => {
                match device {
                    MidiDevice::LaunchControlXL => {
                        // From Novation
                        if controller == 77 { self.app_state.values[0] = normalize_midi(value); }
                        if controller == 78 { self.app_state.values[1] = normalize_midi(value); }
                        if controller == 79 { self.app_state.values[2] = normalize_midi(value); }
                        if controller == 80 { self.app_state.values[3] = normalize_midi(value); }

                        if controller == 106 { if value == 127 {self.handle_ui_input(InputEvent::Prev);} else {} }
                        if controller == 107 { if value == 127 {self.handle_ui_input(InputEvent::Next);} else {} }
                    },
                    MidiDevice::OPZ => {
                        // From OP-Z
                        if controller == 1 { self.app_state.values[0] = normalize_midi(value); }
                        if controller == 2 { self.app_state.values[1] = normalize_midi(value); }
                        if controller == 3 { self.app_state.values[2] = normalize_midi(value); }
                        if controller == 4 { self.app_state.values[3] = normalize_midi(value); }
                    }
                    MidiDevice::Deluge => {
                        if controller == 0 { self.app_state.values[0] = normalize_midi(value); }
                        if controller == 1 { self.app_state.values[1] = normalize_midi(value); }
                        if controller == 2 { self.app_state.values[2] = normalize_midi(value); }
                        if controller == 3 { self.app_state.values[3] = normalize_midi(value); }
                    }
                    _ => ()
                }
                //println!("{:?}", self.app_state);
            }
            MidiMessage::NoteOn { note, .. } => {
                match device {
                    MidiDevice::LaunchControlXL => {
                        // From Novation
                        if note == 73 { self.app_state.values[4] = 1.0 }
                        if note == 74 { self.app_state.values[5] = 1.0 }
                        if note == 75 { self.app_state.values[6] = 1.0 }
                        if note == 76 { self.app_state.values[7] = 1.0 }
                        
                        if note == 105 { self.handle_ui_input(InputEvent::Enter); }

                        if note == 41 { self.app_state.shader_index = 0 }
                        if note == 42 { self.app_state.shader_index = 1 }
                        if note == 43 { self.app_state.shader_index = 2 }

                        if note == 57 { self.app_state.dsp_type = DspType::SimpleSine }
                        if note == 58 { self.app_state.dsp_type = DspType::DrumEngine }
                        if note == 59 { self.app_state.dsp_type = DspType::BasicFm }
                    },
                    MidiDevice::OPZ => {
                        // From OP-Z
                        if note == 53 { self.app_state.values[4] = 1.0 }
                        if note == 54 { self.app_state.shader_index += 1 }
                    },
                    MidiDevice::Deluge => {
                        if note == 60 { self.app_state.values[4] = 1.0 }
                        if note == 62 { self.app_state.values[5] = 1.0 }
                        if note == 64 { self.app_state.values[6] = 1.0 }
                        if note == 65 { self.app_state.values[7] = 1.0 }
                    }
                    _ => ()
                }

            }
            MidiMessage::NoteOff { note } => {
                match device {
                    MidiDevice::LaunchControlXL => {
                        // From Novation
                        if note == 73 { self.app_state.values[4] = 0.0 }
                        if note == 74 { self.app_state.values[5] = 0.0 }
                        if note == 75 { self.app_state.values[6] = 0.0 }
                        if note == 76 { self.app_state.values[7] = 0.0 }
                    },
                    MidiDevice::OPZ => {
                        // From OP-Z
                        if note == 53 { self.app_state.values[4] = 0.0 }
                    },
                    MidiDevice::Deluge => {
                        if note == 60 { self.app_state.values[4] = 0.0 }
                        if note == 62 { self.app_state.values[5] = 0.0 }
                        if note == 64 { self.app_state.values[6] = 0.0 }
                        if note == 65 { self.app_state.values[7] = 0.0 }
                    }
                    _ => ()
                }
            }
        }
    }
}

pub fn start_coordinator_thread(
    receiver: Receiver<Message>,
    window_writer: Input<State>,
    graphics_display_writer: Input<DisplayState>,
    oled_display_writer: Input<DisplayState>,
    audio_writer: Input<State>,
) {
    thread::spawn(move || {
        let coordinator = Coordinator {
            app_state: State::default(),
            ui_state: Screen::default(),
        };

        coordinator.run(receiver, window_writer, graphics_display_writer, oled_display_writer, audio_writer);
    });
}

fn normalize_midi(value: u8) -> f32 {
    value as f32 / 128.0
}

struct MidiConfig {
    launch_control_xl: MidiDeviceMapping,
    opz: MidiDeviceMapping,
    deluge: MidiDeviceMapping,
}

impl MidiConfig {
    fn new() -> Self {
        Self { 
            launch_control_xl: MidiDeviceMapping {
                v0_cv: Some(77),
                v1_cv: Some(78),
                v2_cv: Some(79),
                v3_cv: Some(80),
                
                v4_gate: Some(73),
                v5_gate: Some(74),
                v6_gate: Some(75),
                v7_gate: Some(76),

                prev_ui: Some(106),
                next_ui: Some(107),
                enter_ui: Some(105),
            }, 
            opz: MidiDeviceMapping {
                v0_cv: Some(1),
                v1_cv: Some(2),
                v2_cv: Some(3),
                v3_cv: Some(4),
                
                v4_gate: Some(53),
                v5_gate: Some(54),
                v6_gate: Some(55),
                v7_gate: Some(56),

                prev_ui: None,
                next_ui: None,
                enter_ui: None,
            }, 
            deluge: MidiDeviceMapping {
                v0_cv: Some(0),
                v1_cv: Some(1),
                v2_cv: Some(2),
                v3_cv: Some(3),
                
                v4_gate: Some(60),
                v5_gate: Some(62),
                v6_gate: Some(64),
                v7_gate: Some(65),

                prev_ui: None,
                next_ui: None,
                enter_ui: None,
            }, 
        }
    }
}
