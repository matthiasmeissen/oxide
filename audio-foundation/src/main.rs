
use std::io::stdin;
use midir::{Ignore, MidiInput};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use fundsp::hacker::*;

fn main() {
    let trigger = shared(0.0);
    let freq = shared(440.0);

    
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

    let env = var(&trigger) >> adsr_live(0.01, 0.0, 1.0, 0.2);
    let osc1 = var(&freq) >> sine();
    let osc2 = var(&freq) >> saw();
    let osc = (osc1 + osc2) * 0.5;
    let filter = osc >> lowrez_hz(800.0, 1.0);
    let synth= filter * env;

    let mut node = synth * 0.2;
    node.set_sample_rate(sample_rate);

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


    // MIDI
    println!("Midi Input");

    let mut midi_in = MidiInput::new("Midi Input 1").unwrap();
    midi_in.ignore(Ignore::None);

    let in_ports = midi_in.ports();
    let port = match in_ports.len() {
        0 => {
            println!("No midi port available.");
            return;
        },
        1 => {
            println!("Connecting to port {}", midi_in.port_name(&in_ports[0]).unwrap());
            &in_ports[0]
        },
        _ => {
            println!("Too many ports.");
            return;
        },
    };

    let _connection = midi_in.connect(
        port, "Midi Input", move|timestamp, message, _| {
            println!("{}: {:?}", timestamp, message);
            if message[2] == 0 {
                trigger.set_value(0.0);
            } else {
                trigger.set_value(1.0);
            }
            let midi_note = message[1] as f64;
            let frequency = 440.0 * (2.0f64).powf((midi_note - 69.0) / 12.0);
            freq.set_value(frequency);
            println!("Playing note number {} with a frequency of {:.2}", midi_note, frequency);
        }, ()).unwrap();

    println!("Listening, press a key to quit program.");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
}
