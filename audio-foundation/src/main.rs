use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
// There are two types of fundsp: hacker which is 64bit and hacker32 which is 32 bit
use fundsp::hacker::*;

fn main() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("No output device available.");    
    let config = device.default_output_config().expect("No default output config found.");
    println!("Output device: {:?}", device.name()); 
    println!("Default output config: {:#?}", config);

    let sample_format = config.sample_format();
    println!("Expected sample format: {}", sample_format);

    let stream_config: cpal::StreamConfig = config.into();
    println!("Stream config: {:#?}", stream_config);

    let sample_rate = stream_config.sample_rate.0 as f64;
    let channels = stream_config.channels as usize;

    // The simplest graph is a single generator
    // Generators are nodes that have outputs but no inputs
    let sine = sine_hz(440.0);
    // You can create a stereo signal with the stack | operator
    let stereo_noise = noise() | noise();

    // The most common action is changing notes together
    // The pipe >> operator takes output of node A and connects it to input of node B
    let lowpass_synth = saw_hz(440.0) >> lowpass_hz(200.0, 1.0);

    // You can combine nodes with the stack | operator
    // This lets you create stereo signals or provide multiple parameters to nodes
    let stereo_osc = saw_hz(240.0) | saw_hz(320.0);
    let stereo_filter = lowpass_hz(200.0, 1.0) | lowpass_hz(300.0, 1.0);
    let stereo_synth = stereo_osc >> stereo_filter;

    // You can modulate an mix signals with the * and + operator
    let carrier = saw_hz(440.0);
    let modulator = sine_hz(8.0);
    let am_synth = carrier * modulator;

    // To create tiem varying signals you can use the lfo() or envelope() function
    // They are just functions that take time as an input
    let osc = sine_hz(440.0);
    // Filter control has three outputs 
    // pass() just pipes the input through andlfo has two outputs
    // First is a time based sin to control the cutoff and second is a constant signal for the Q of the filter
    let filter_control = pass() | lfo(
        |t| (xerp11(100.0, 2000.0, sin_hz(2.0, t)), 1.0));
    let synth = osc >> filter_control >> lowpass();

    // Define the node graph
    let mut graph = synth * 0.2;
    graph.set_sample_rate(sample_rate);

    // The iterator that when called is getting the values for the left and right channel
    let mut next_value = move || graph.get_stereo();

    let audio_callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        for frame in data.chunks_mut(channels) {
                let (l, r) = next_value();
                frame[0] = l as f32;
                if channels > 1 {
                    frame[1] = r as f32;
                }
            }
    };

    let err_fn = |err| eprintln!("An error occured on the audio stream: {}", err);

    let stream = match sample_format {
        cpal::SampleFormat::F32 => device.build_output_stream(
            &stream_config,
            audio_callback,
            err_fn,
            None
        ),
        _ => Err(cpal::BuildStreamError::StreamConfigNotSupported),
    }.expect("Could not build f32 output stream.");

    stream.play().expect("Could not start audio stream.");
    println!("Audio pipeline is running.");

    std::thread::sleep(std::time::Duration::from_secs(4));
    println!("Program finished.");
}
