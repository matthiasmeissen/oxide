
mod modules;
#[cfg(target_os = "linux")]
use modules::display::*;

#[cfg(not(target_os = "linux"))]
use modules::display_simulator::*;

use modules::{coordinator::*, midi::*};

use crossbeam_channel;
use triple_buffer::TripleBuffer;

use crate::modules::state::PlaceholderState;

fn main() {
    let (sender, receiver) = crossbeam_channel::bounded(5);
    let (display_writer, display_reader) = TripleBuffer::new(&PlaceholderState::new()).split();

    start_coordinator_thread(receiver, display_writer);

    start_midi_thread(sender.clone());

    #[cfg(target_os = "linux")]
    start_display(display_reader);

    #[cfg(not(target_os = "linux"))]
    start_display(display_reader);
}
