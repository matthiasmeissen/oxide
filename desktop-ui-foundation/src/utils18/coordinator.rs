use crate::utils18::state::*;

use std::thread;
use crossbeam_channel::Receiver;
use triple_buffer::Input;

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
                Message::SetResolution(w, h) => current_state.resolution = [w, h],
                Message::SetValue(i, v) => current_state.values[i] = v,
            }

            if current_state != last_published_state {
                window_writer.write(current_state);
                audio_writer.write(current_state);
                last_published_state = current_state;
                println!("// -> Cooridnator published new state.");
            }
        }
    });
}