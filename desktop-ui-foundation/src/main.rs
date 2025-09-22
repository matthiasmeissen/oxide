
mod utils18;
use utils18::{
    state::*, 
    coordinator::*, 
    midi::*, 
    audio::*,
    graphics::*,
};

use std::thread;
use std::time::Duration;
use crossbeam_channel;
use triple_buffer::TripleBuffer;


fn main() {
    let (sender, receiver) = crossbeam_channel::bounded(5);
    let (mut window_writer, mut window_reader) = TripleBuffer::new(&State::default()).split();
    let (mut audio_writer, mut audio_reader) = TripleBuffer::new(&State::default()).split();

    start_coordinator_thread(receiver, window_writer, audio_writer);

    start_midi_thread(sender.clone());

    start_audio_thread(audio_reader);

    start_graphics_thread(sender.clone(), window_reader);

    thread::sleep(Duration::from_secs(1));
}
