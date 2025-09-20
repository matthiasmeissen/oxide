use crate::utils::state::*;

use std::thread;
use crossbeam_channel::Receiver;
use triple_buffer::Input;

const DEBUG: bool = false;

pub fn start_coordinator_thread(
    receiver: Receiver<Message>, 
    mut window_writer: Input<State>, 
    mut audio_writer: Input<State>) 
    {
    thread::spawn(move || {
        let mut current_state = State::default();
        let mut last_published_state = current_state;

        while let Ok(update) = receiver.recv() {
            match update {
                Message::SetTime(t) => current_state.time = t,
                Message::SetResolution(w, h) => {
                    println!("{:?}", current_state);
                    current_state.resolution = [w, h]
                },
                Message::SetValue(i, v) => {
                    println!("{:?}", current_state);
                    current_state.values[i] = v
                },
                Message::MidiInput(midi) => match midi {
                    MidiMessage::ControlChange { controller, value } => {
                        // From Novation
                        if controller == 77 { current_state.values[0] = normalize_midi(value); }
                        if controller == 78 { current_state.values[1] = normalize_midi(value); }
                        if controller == 79 { current_state.values[2] = normalize_midi(value); }
                        if controller == 80 { current_state.values[3] = normalize_midi(value); }

                        // From OP-Z
                        if controller == 1 { current_state.values[0] = normalize_midi(value); }
                        if controller == 2 { current_state.values[1] = normalize_midi(value); }
                        if controller == 3 { current_state.values[2] = normalize_midi(value); }
                        if controller == 4 { current_state.values[3] = normalize_midi(value); }

                        println!("{:?}", current_state);
                    }
                    MidiMessage::NoteOn { note, velocity } => {
                        // From OP-Z
                        if note == 53 { current_state.values[4] = 1.0 }
                    }
                    MidiMessage::NoteOff { note } => {
                        // From OP-Z
                        if note == 53 { current_state.values[4] = 0.0 }
                    }
                    _ => ()
                },
            }

            if current_state != last_published_state {
                window_writer.write(current_state);
                audio_writer.write(current_state);
                last_published_state = current_state;
                if DEBUG {println!("// -> Cooridnator published new state. {:#?}", current_state)} else {};
            }
        }
    });
}

fn normalize_midi(value: u8) -> f64 {
    value as f64 / 128.0
}
