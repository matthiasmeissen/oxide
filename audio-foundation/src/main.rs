use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use fundsp::hacker::*;
use std::thread;
use std::time::Duration;

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


    // Create and clone shared variable to move into sequencer thread
    let frequency = shared(261.63);
    let frequency_controller = frequency.clone();

    // Sequencer Thread
    thread::spawn(move || {
        let notes = [261.63, 329.63, 392.00, 493.88];
        let mut note_index = 0;
        
        let tempo_bpm = 120.0;
        let note_duration = Duration::from_secs_f64(60.0 / tempo_bpm / 2.0);

        println!("Sequencer thread started.");
        loop {
            frequency_controller.set_value(notes[note_index]);
            note_index = (note_index + 1) % notes.len();
            thread::sleep(note_duration);
        }
    });

    // Setup and Run Audio Stream
    let synth = var(&frequency) >> sine();
    let mut graph = synth * 0.2;
    graph.set_sample_rate(sample_rate);

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

    // Keep the main thread alive to allow the audio and sequencer threads to run.
    std::thread::sleep(std::time::Duration::from_secs(10));
    println!("Program finished.");
}