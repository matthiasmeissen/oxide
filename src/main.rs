
// Use cargo run --features "simulator"
// To run the display simulator

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

use crossbeam_channel;
use triple_buffer::TripleBuffer;

fn main() {
    println!("Prototype 2");

    let (sender, receiver) = crossbeam_channel::bounded(5);
    let (window_writer, window_reader) = TripleBuffer::new(&State::default()).split();
    let (graphics_display_writer, graphics_display_reader) = TripleBuffer::new(&DisplayState::default()).split();
    let (oled_display_writer, oled_display_reader) = TripleBuffer::new(&DisplayState::default()).split();
    let (audio_writer, audio_reader) = TripleBuffer::new(&State::default()).split();

    coordinator::start_coordinator_thread(receiver, window_writer, graphics_display_writer, oled_display_writer, audio_writer);
    midi::start_midi_thread(sender.clone());
    audio::start_audio_thread(audio_reader);

    graphics::start_graphics_thread(sender.clone(), window_reader, graphics_display_reader);

    #[cfg(target_os = "linux")]
    {
        println!("Running on target hardware");
        display::start_display(oled_display_reader);
        i2c::start_i2c_thread(sender.clone());
    }
}
