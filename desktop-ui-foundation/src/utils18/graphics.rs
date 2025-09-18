use crate::utils18::state::*;

use std::thread;
use std::time::Duration;
use crossbeam_channel::Sender;
use triple_buffer::Output;

pub fn start_graphics_thread(window_sender: Sender<Message>, mut window_reader: Output<State>) {
    thread::spawn(move || {
        let mut time = 0.0;
        loop {
            // Send Time Value
            window_sender.try_send(Message::SetTime(time)).ok();
            println!("Window update time: {:.2}", time);

            // Read latest state, this is non blocking
            let state = window_reader.read();
            println!("Window read: {:?}", *state);

            time += 0.1;
            thread::sleep(Duration::from_millis(100));
        }
    });
}