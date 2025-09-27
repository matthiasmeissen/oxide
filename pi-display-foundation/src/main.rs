
mod modules;
use modules::{coordinator::*, midi::*, state::*, graphics::*};

#[cfg(target_os = "linux")]
use modules::display::*;

#[cfg(not(target_os = "linux"))]
use modules::display_simulator::*;

use crossbeam_channel;
use triple_buffer::TripleBuffer;
use std::{thread, time::Duration};

// To run the simulator run this command in the terminal
// export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
// This will enable it for that session only

fn main() {
    let (sender, receiver) = crossbeam_channel::bounded(5);
    let (window_writer, window_reader) = TripleBuffer::new(&State::default()).split();
    let (display_writer, display_reader) = TripleBuffer::new(&State::default()).split();

    start_coordinator_thread(receiver, window_writer, display_writer);

    start_midi_thread(sender.clone());

    #[cfg(target_os = "linux")]
    start_display(display_reader);
    
    #[cfg(not(target_os = "linux"))]
    start_display_simulator(display_reader);

    //start_graphics_thread(sender.clone(), window_reader);

    thread::sleep(Duration::from_secs(1));
}
