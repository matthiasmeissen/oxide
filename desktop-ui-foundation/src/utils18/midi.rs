use crate::utils18::state::*;

use std::thread;
use std::time::Duration;
use crossbeam_channel::Sender;

pub fn start_midi_thread(midi_sender: Sender<Message>) {
    thread::spawn(move || {
        let mut val1 = 0.0;
        let mut val2 = 0.0;
        loop {
            // Use try_send to avoid blocking if channel is full
            midi_sender.try_send(Message::SetValue(0, val1)).ok();
            midi_sender.try_send(Message::SetValue(1, val2)).ok();
            println!("MIDI update value1: {val1} and value2 {val2}");
            val1 += 0.1;
            val2 = val1 * 2.0;
            thread::sleep(Duration::from_millis(200));
        }
    });
}