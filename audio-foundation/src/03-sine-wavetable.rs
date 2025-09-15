use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::f32;

// FAUST-inspired wavetable oscillator
struct WavetableOscillator {
    table: [f32; 65536],
    phase: f32,
    phase_increment: f32,
    amplitude: f32,
}

impl WavetableOscillator {
    fn new(frequency: f32, sample_rate: f32, amplitude: f32) -> Self {
        let mut table = [0.0f32; 65536];
        
        // Generate sine wave lookup table (like FAUST's mydspSIG0)
        for i in 0..65536 {
            table[i] = f32::sin(2.0 * f32::consts::PI * i as f32 / 65536.0);
        }
        
        let phase_increment = frequency / sample_rate;
        
        Self {
            table,
            phase: 0.0,
            phase_increment,
            amplitude,
        }
    }
    
    fn set_frequency(&mut self, frequency: f32, sample_rate: f32) {
        self.phase_increment = frequency / sample_rate;
    }
    
    fn get_sample(&mut self) -> f32 {
        // Convert phase to table index (like FAUST's approach)
        let table_index = (self.phase * 65536.0) as usize % 65536;
        let sample = self.table[table_index] * self.amplitude;
        
        // Advance phase and wrap
        self.phase += self.phase_increment;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        
        sample
    }
    
    fn get_stereo(&mut self) -> (f32, f32) {
        let sample = self.get_sample();
        (sample, sample)
    }
}

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
    
    let sample_rate = stream_config.sample_rate.0 as f32;
    let channels = stream_config.channels as usize;
    
    // Create FAUST-inspired wavetable oscillator
    let mut oscillator = WavetableOscillator::new(440.0, sample_rate, 0.2);
    
    println!("Using wavetable oscillator at {}Hz with {}% amplitude", 440.0, 0.2 * 100.0);
    
    let audio_callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
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
        cpal::SampleFormat::F32 => device.build_output_stream(
            &stream_config,
            audio_callback,
            err_fn,
            None
        ),
        _ => Err(cpal::BuildStreamError::StreamConfigNotSupported),
    }.expect("Could not build f32 output stream.");
    
    stream.play().expect("Could not start audio stream.");
    
    println!("Audio pipeline is running with FAUST-inspired wavetable...");
    std::thread::sleep(std::time::Duration::from_secs(4));
    println!("Program finished.");
}