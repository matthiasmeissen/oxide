use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
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

    // Define the node graph
    let mut node = sine_hz(440.0) * 0.2;
    node.set_sample_rate(sample_rate);

    // The iterator that when called is getting the values for the left and right channel
    let mut next_value = move || node.get_stereo();

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
