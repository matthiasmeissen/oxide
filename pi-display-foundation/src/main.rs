
mod modules;
use modules::{coordinator::*, midi::*, state::*};

#[cfg(target_os = "linux")]
use modules::display::*;

#[cfg(not(target_os = "linux"))]
use modules::display_simulator::*;

use crossbeam_channel;
use triple_buffer::TripleBuffer;

// To run the simulator run this command in the terminal
// export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
// This will enable it for that session only

fn main() {
    let (sender, receiver) = crossbeam_channel::bounded(5);
    let (display_writer, display_reader) = TripleBuffer::new(&State::default()).split();

    start_coordinator_thread(receiver, display_writer);

    start_midi_thread(sender.clone());

    #[cfg(target_os = "linux")]
    start_display(display_reader);

    #[cfg(not(target_os = "linux"))]
    start_display_simulator(display_reader);
}
