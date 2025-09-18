use crate::utils18::state::*;

use std::thread;
use std::time::Duration;
use triple_buffer::Output;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use fundsp::hacker32::*;

const DEBUG: bool = false;

pub fn start_audio_thread(mut audio_reader: Output<State>) {
    thread::spawn(move || {

        let host = cpal::default_host();
        let device = host.default_output_device().expect("No output device available.");
        let config = device.default_output_config().expect("No default output config found.");
        let sample_format = config.sample_format();
        let stream_config: cpal::StreamConfig = config.into();
        let sample_rate = stream_config.sample_rate.0 as f64;
        let channels = stream_config.channels as usize;

        println!("[AUDIO] Output device: {}", device.name().unwrap_or_default());
        println!("[AUDIO] Sample rate: {}", sample_rate);

        let sh_freq1 = shared(200.0);
        let sh_freq2 = shared(200.0);
        let sh_cutoff = shared(400.0);
        let sh_modulate = shared(4.0);

        // Linear: input * (max - min) + min
        let freq1 = var(&sh_freq1) * (800.0 - 40.0) + 40.0;
        let freq2 = var(&sh_freq2) * (800.0 - 40.0) + 40.0;
        let cutoff = var(&sh_cutoff) * (800.0 - 200.0) + 200.0;
        let modulate = var(&sh_modulate) * (4.0 - 0.1) + 0.1;

        let osc1 = freq1 >> saw();
        let osc2 = freq2 >> saw();
        let osc = osc1 + osc2;
        let amp = modulate >> (sine() + 1.0) * 0.5;

        let synth = (osc | cutoff | dc(1.0)) >> lowrez() * amp;

        let mut node = synth * 0.2;
        node.set_sample_rate(sample_rate);

        let mut next_value = move || node.get_stereo();

        let audio_callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
            let state = audio_reader.read();

            sh_freq1.set_value(state.values[0] as f32);
            sh_freq2.set_value(state.values[1] as f32);
            sh_cutoff.set_value(state.values[2] as f32);
            sh_modulate.set_value(state.values[3] as f32);

            for frame in data.chunks_mut(channels) {
                let (l, r) = next_value();
                frame[0] = l;
                if channels > 1 {
                    frame[1] = r;
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
        println!("[AUDIO] Audio pipeline is running.");

        loop {
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}