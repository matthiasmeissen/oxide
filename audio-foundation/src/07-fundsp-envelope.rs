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


    // AMPLITUDE MODULATION
    let carrier = sine_hz(440.0);
    // The lfo is a closure that takes time as an input
    let amplitude_lfo = lfo(|t| {
        // This creates a sin based on t in the range of -1 to 1
        let lfo_wave = sin_hz(8.0, t);
        // This maps a range from -1 to 1 into a new lower and upper values
        lerp11(0.1, 0.8, lfo_wave)
    });
    let synth = carrier * amplitude_lfo;


    // FREQUENCY MODULATION
    let base_freq = 440.0;
    // You need to use the move keyword with the closure to ensure that the base_freq is passed into it
    let pitch_lfo = lfo(move |t| {
        let lfo_wave = sin_hz(4.0, t);
        base_freq + 8.0 * lfo_wave
    });
    // The sine() function has an input for the frequency
    let synth = pitch_lfo >> sine();


    // MULTIPLE SIGNALS
    let source = noise();
    let cutoff_lfo = lfo(|t| {
        let lfo_wave = sin_hz(4.0, t);
        xerp11(100.0, 5000.0, lfo_wave)
    });
    // You can stack multiple signals with the stack | operator
    // This lets you use nodes that require multiple inputs (like lowrez, which expects audio, cutoff frequency and q factor)
    // The pass() function just takes the incoming signal and passes it to the output without changing it
    let synth = source >> (pass() | cutoff_lfo | dc(1.0)) >> lowrez();

    // CUSTOM NODES
    // You can create your own nodes that take input using pass(), process it and generate some output
    let lfo_filter = (pass() | lfo(|t| {xerp11(200.0, 2000.0, sin_hz(8.0, t))}) | dc(1.0)) >> lowpass();
    let synth = noise() >> lfo_filter;


    // TRIGGER ADSR ENVELOPE
    // This creates a repeating trigger which is used as an input for an adsr envelope
    let pulse_period = 2.0;
    let pulse_duration = 0.02; // A very short 20ms trigger pulse.
    let trigger = lfo(move |t| {
        // t % pulse_period gives a repeating ramp from 0.0 to 2.0
        // If we are in the first 20ms of that ramp, output 1.0.
        if t % pulse_period < pulse_duration {
            1.0
        } else {
            0.0
        }
    });
    let vca = trigger >> adsr_live(0.01, 0.0, 1.0, 0.4);
    let synth = sine_hz(440.0) * vca;


    // ARPEGGIO SEQUENCER
    let tempo_bpm = 120.0;
    // Define beat duration by dividing one minute by bpm, which gives you the duration for one beat
    let beat_tempo = 60.0 / tempo_bpm; 
    // Divide it by two so you get the duration for half beat
    let beat_duration = beat_tempo / 2.0;
    let notes = [261.63, 329.63, 392.00, 493.88];
    let sequencer = lfo(move |t| {
        let current_beat = (t / beat_duration) as i64;
        let note_index = (current_beat % 4) as usize;
        let frequency = notes[note_index];
        frequency
    });
    // Use the sequencer signal to set the frequency of the osciallator
    let synth = sequencer >> sine();


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
