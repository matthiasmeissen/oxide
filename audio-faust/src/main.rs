
// Tasks
// Export basic sine wave graph from Faust to Rust    
// Modify to work with cpal
// Place DSP code in module

// Export graph from Faust using parameters for frequency
// Add miniquad and control frequency with mouse position
// Export graph from Faust with adsr trigger parameters
// Use keypress from miniquad to trigger envelope
// Add more DSP modules and switch on runtime bewteen them


mod dsp;
mod audio;
use audio::*;

use std::io;

fn main()  {
    start_audio_thread();

    println!("\nAudio is playing. Press Enter to quit.");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    println!("Exiting.");
}
