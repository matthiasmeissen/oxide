
// Tasks
// Export basic sine wave graph from Faust to Rust    
// Modify to work with cpal
// Place DSP code in module

// Export graph from Faust using parameters for frequency
// Add miniquad and control frequency with mouse position
// Export graph from Faust with adsr trigger parameters
// Use keypress from miniquad to trigger envelope
// Add more DSP modules and switch on runtime bewteen them


use std::io;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

mod dsp;
use dsp::simple_sine::SimpleSine;

fn main() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("no output device available");
    let config = device.default_output_config().expect("no default config");
    let sample_rate = config.sample_rate().0 as i32;
    
    let mut dsp = SimpleSine::new();
    dsp.init(sample_rate);

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        &config.into(),
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            // Since our DSP is mono, we can just treat the whole buffer as our one output.
            // If it were stereo, we would need to de-interleave the `data` buffer first.
            let buffer_size = data.len();
            let mut outputs: [&mut [f32]; 1] = [&mut data[..buffer_size]];
            let inputs: [&[f32]; 0] = [];

            dsp.compute(buffer_size as usize, &inputs, &mut outputs);
        },
        err_fn,
        None
    ).unwrap();

    stream.play().unwrap();

    println!("Playing audio, press Enter to quit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
