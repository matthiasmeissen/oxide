
mod modules;
use modules::{coordinator::*, state::*, i2c::*, graphics::*};

use crossbeam_channel;
use triple_buffer::TripleBuffer;
use std::{thread, time::Duration};

fn main() {
    let (sender, receiver) = crossbeam_channel::bounded(5);
    let (window_writer, window_reader) = TripleBuffer::new(&State::default()).split();
    let (display_writer, display_reader) = TripleBuffer::new(&State::default()).split();

    start_coordinator_thread(receiver, window_writer, display_writer);

    start_i2c_thread(sender.clone());

    start_graphics_thread(sender.clone(), window_reader);

    thread::sleep(Duration::from_secs(1));
}
