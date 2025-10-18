use crate::state::*;

use std::thread;
use crossbeam_channel::Receiver;
use triple_buffer::*;

const NUM_SHADERS: usize = 3;

struct Coordinator {
    app_state: State,
    ui_state: Screen
}

impl Coordinator {
    fn run (mut self,
        receiver: Receiver<Message>,
        mut window_writer: Input<State>,
        mut display_writer: Input<DisplayState>,
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
                Message::UiInput(event) => {println!("Test: {:?}", event); self.handle_ui_input(event)},
                Message::MidiInput(device, midi) => self.handle_midi_input(device, midi),
            }

            let app_state_change = self.app_state != last_published_app_state;
            let ui_state_change = self.ui_state != last_published_ui_state;

            if app_state_change || ui_state_change {
                let display_payload = DisplayState {
                    app: self.app_state,
                    ui: self.ui_state,
                };
                display_writer.write(display_payload);
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
                InputEvent::Enter => Screen::ShaderSelect {selected_index: 0},
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
                InputEvent::Enter => Screen::Shader,
            }
            Screen::Audio => match event {
                InputEvent::Next => Screen::Home,
                InputEvent::Prev => Screen::Shader,
                InputEvent::Enter => Screen::AudioSelect {selected_index: 0},
            }
            Screen::AudioSelect { mut selected_index} => match event {
                InputEvent::Next => {println!("Next"); Screen::Audio},
                InputEvent::Prev => {println!("Prev"); Screen::Audio},
                InputEvent::Enter => Screen::Audio,
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

                        if note == 41 { self.app_state.shader_index = 0 }
                        if note == 42 { self.app_state.shader_index = 1 }
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
    display_writer: Input<DisplayState>,
    audio_writer: Input<State>,
) {
    thread::spawn(move || {
        let coordinator = Coordinator {
            app_state: State::default(),
            ui_state: Screen::default(),
        };

        coordinator.run(receiver, window_writer, display_writer, audio_writer);
    });
}

fn normalize_midi(value: u8) -> f32 {
    value as f32 / 128.0
}
