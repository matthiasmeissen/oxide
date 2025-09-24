
mod modules;
use modules::{
    display::*,
    coordinator::*,
    midi::*,
};

use crossbeam_channel;
use triple_buffer::TripleBuffer;

use crate::modules::state::PlaceholderState;

// To run the simulator run this command in the terminal
// export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
// This will enable it for that session only


fn main() {
    println!("Embedded Foundation");
    let (sender, receiver) = crossbeam_channel::bounded(5);
    let (display_writer, display_reader) = TripleBuffer::new(&PlaceholderState::new()).split();

    start_coordinator_thread(receiver, display_writer);

    start_midi_thread(sender.clone());

    start_display(display_reader).expect("Error starting display.");
}
