
// Use cargo run --features "simulator"
// And export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
// To run the display simulator

pub mod state;
pub mod coordinator;
pub mod graphics;
pub mod audio;
pub mod dsp;
pub mod midi;

#[cfg(feature = "simulator")]
pub mod display_simulator;

#[cfg(not(feature = "simulator"))]
pub mod display;
#[cfg(not(feature = "simulator"))]
pub mod i2c;
pub mod screens;

use state::*;
use coordinator::*;
use graphics::*;
use audio::*;
use midi::*;

#[cfg(feature = "simulator")]
use display_simulator::*;

#[cfg(all(target_os = "linux", not(feature = "simulator")))]
use display::*;
#[cfg(all(target_os = "linux", not(feature = "simulator")))]
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

    #[cfg(feature = "simulator")]
    {
        println!("Running with display simulator");
        start_display_simulator(sender.clone(), display_reader);
    }

    #[cfg(not(feature = "simulator"))]
    {
        #[cfg(target_os = "linux")]
        {
            println!("Running on target hardware");
            start_display(display_reader);
            start_i2c_thread(sender.clone());
        }
    
        start_graphics_thread(sender.clone(), window_reader);
    }
}
