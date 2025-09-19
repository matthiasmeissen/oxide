use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::f32;
use std::sync::{Arc, Mutex};

mod utils;
use utils::wavetable_01::*;
use utils::gui_01::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct SharedState {
    frequency: f32,
    amplitude: f32,
    waveform: Waveform,
    sample_rate: f32,
}

impl SharedState {
    fn new(sample_rate: f32) -> Self {
        Self {
            frequency: 440.0,
            amplitude: 0.2,
            waveform: Waveform::Sine,
            sample_rate,
        }
    }
}

fn main() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("No output device available.");
    let config = device.default_output_config().expect("No default output config found.");
    let sample_format = config.sample_format();
    let stream_config: cpal::StreamConfig = config.into();
    let sample_rate = stream_config.sample_rate.0 as f32;
    let channels = stream_config.channels as usize;

    let audio_state = Arc::new(Mutex::new(SharedState::new(sample_rate)));
    let audio_state_clone = Arc::clone(&audio_state);

    let mut oscillator = WavetableOscillator::new(440.0, sample_rate, 0.2, Waveform::Sine);

    let audio_callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        if let Ok(state) = audio_state_clone.lock() {
            oscillator.set_frequency(state.frequency, state.sample_rate);
            oscillator.set_amplitude(state.amplitude);
            oscillator.set_waveform(state.waveform);
        }
        for frame in data.chunks_mut(channels) {
            let (l, r) = oscillator.get_stereo();
            frame[0] = l;
            if channels > 1 {
                frame[1] = r;
            }
        }
    };

    let err_fn = |err| eprintln!("An error occurred on the audio stream: {}", err);
    let stream = match sample_format {
        cpal::SampleFormat::F32 => device.build_output_stream(&stream_config, audio_callback, err_fn, None),
        _ => Err(cpal::BuildStreamError::StreamConfigNotSupported),
    }.expect("Could not build f32 output stream.");

    stream.play().expect("Could not start audio stream.");


    let app_ui = UserInterface::new(Arc::clone(&audio_state));
    app_ui.run();
}