use crate::utils18::state::*;

use std::thread;
use std::time::Duration;
use triple_buffer::Output;

pub fn start_audio_thread(mut audio_reader: Output<State>) {
    thread::spawn(move || {
        loop {
            let state = audio_reader.read();
            let freq = state.values[0];
            let amp = state.values[1];
            println!("Audio set freq {:.2} and amp {:.2}", freq, amp);
            thread::sleep(Duration::from_millis(20));
        }
    });
}