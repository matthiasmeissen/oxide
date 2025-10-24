
// Use cargo run --features "simulator"
// To run the display simulator

pub mod state;
pub mod coordinator;
pub mod graphics;
pub mod graphics_simulator;
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
    let (display_writer, display_reader) = TripleBuffer::new(&DisplayState::default()).split();
    let (audio_writer, audio_reader) = TripleBuffer::new(&State::default()).split();

    coordinator::start_coordinator_thread(receiver, window_writer, display_writer, audio_writer);
    midi::start_midi_thread(sender.clone());
    audio::start_audio_thread(audio_reader);

    #[cfg(feature = "simulator")]
    {
        println!("Running with display simulator");
        graphics_simulator::start_graphics_simulator_thread(sender.clone(), window_reader, display_reader);
    }

    #[cfg(not(feature = "simulator"))]
    {
        #[cfg(target_os = "linux")]
        {
            println!("Running on target hardware");
            display::start_display(display_reader);
            i2c::start_i2c_thread(sender.clone());
        }
    
        graphics::start_graphics_thread(sender.clone(), window_reader);
    }
}
