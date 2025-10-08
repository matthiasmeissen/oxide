
// Tasks
// Export basic sine wave graph from Faust to Rust    
// Modify to work with cpal
// Place DSP code in module
// Export graph from Faust using parameters for frequency

// Add miniquad and control frequency with mouse position
// Export graph from Faust with adsr trigger parameters
// Use keypress from miniquad to trigger envelope
// Add more DSP modules and switch on runtime between them


mod dsp;
mod audio;
mod state;
mod coordinator;
mod graphics;

use audio::*;
use state::*;
use coordinator::*;
use graphics::*;

use crossbeam_channel;
use triple_buffer::TripleBuffer;

fn main()  {
    let (sender, receiver) = crossbeam_channel::bounded(5);
    let (window_writer, window_reader) = TripleBuffer::new(&State::default()).split();
    let (audio_writer, audio_reader) = TripleBuffer::new(&State::default()).split();

    start_coordinator_thread(receiver, window_writer, audio_writer);
    
    start_audio_thread(audio_reader);

    start_graphics_thread(sender.clone(), window_reader);
}
