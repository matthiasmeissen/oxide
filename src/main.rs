
pub mod state;
pub mod coordinator;
pub mod graphics;
pub mod audio;
pub mod dsp;
pub mod midi;
pub mod display;
pub mod i2c;
pub mod screens;

use state::*;
use coordinator::*;
use graphics::*;
use audio::*;
use midi::*;

#[cfg(target_os = "linux")]
use display::*;
#[cfg(target_os = "linux")]
use i2c::*;

use crossbeam_channel;
use triple_buffer::TripleBuffer;

fn main() {
    println!("Prototype 2");

    let (sender, receiver) = crossbeam_channel::bounded(5);
    let (window_writer, window_reader) = TripleBuffer::new(&State::default()).split();
    let (display_writer, display_reader) = TripleBuffer::new(&DisplayState::default()).split();
    let (audio_writer, audio_reader) = TripleBuffer::new(&State::default()).split();

    start_coordinator_thread(receiver, window_writer, display_writer, audio_writer);
    start_midi_thread(sender.clone());
    start_audio_thread(audio_reader);
    start_graphics_thread(sender.clone(), window_reader, display_reader);

    #[cfg(target_os = "linux")]
    {
        start_display(display_reader);
        start_i2c_thread(sender.clone());
    }
}
