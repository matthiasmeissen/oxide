use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::f32;

#[derive(Clone, Copy, Debug)]
pub enum Waveform {
    Sine,
    Saw,
    Triangle,
    Square,
}

// FAUST-inspired wavetable oscillator with multiple waveforms
struct WavetableOscillator {
    sine_table: [f32; 65536],
    saw_table: [f32; 65536],
    triangle_table: [f32; 65536],
    square_table: [f32; 65536],
    phase: f32,
    phase_increment: f32,
    amplitude: f32,
    waveform: Waveform,
}

impl WavetableOscillator {
    fn new(frequency: f32, sample_rate: f32, amplitude: f32, waveform: Waveform) -> Self {
        let mut sine_table = [0.0f32; 65536];
        let mut saw_table = [0.0f32; 65536];
        let mut triangle_table = [0.0f32; 65536];
        let mut square_table = [0.0f32; 65536];
        
        // Generate all waveform lookup tables
        for i in 0..65536 {
            let phase = i as f32 / 65536.0;
            let angle = 2.0 * f32::consts::PI * phase;
            
            // Sine wave
            sine_table[i] = f32::sin(angle);
            
            // Sawtooth wave: linear ramp from -1 to 1
            saw_table[i] = 2.0 * phase - 1.0;
            
            // Triangle wave: ramp up to 1, then ramp down to -1
            triangle_table[i] = if phase < 0.5 {
                4.0 * phase - 1.0  // Rising edge: -1 to 1
            } else {
                3.0 - 4.0 * phase  // Falling edge: 1 to -1
            };
            
            // Square wave: -1 for first half, 1 for second half
            square_table[i] = if phase < 0.5 { -1.0 } else { 1.0 };
        }
        
        let phase_increment = frequency / sample_rate;
        
        Self {
            sine_table,
            saw_table,
            triangle_table,
            square_table,
            phase: 0.0,
            phase_increment,
            amplitude,
            waveform,
        }
    }
    
    fn set_frequency(&mut self, frequency: f32, sample_rate: f32) {
        self.phase_increment = frequency / sample_rate;
    }
    
    fn set_waveform(&mut self, waveform: Waveform) {
        self.waveform = waveform;
    }
    
    fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude;
    }
    
    fn get_sample(&mut self) -> f32 {
        // Convert phase to table index
        let table_index = (self.phase * 65536.0) as usize % 65536;
        
        // Select the appropriate wavetable based on waveform
        let raw_sample = match self.waveform {
            Waveform::Sine => self.sine_table[table_index],
            Waveform::Saw => self.saw_table[table_index],
            Waveform::Triangle => self.triangle_table[table_index],
            Waveform::Square => self.square_table[table_index],
        };
        
        let sample = raw_sample * self.amplitude;
        
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
    
    // Create FAUST-inspired wavetable oscillator with different waveforms
    let mut oscillator = WavetableOscillator::new(440.0, sample_rate, 0.2, Waveform::Sine);
    
    // Demonstrate switching between waveforms over time
    let start_time = std::time::Instant::now();
    
    println!("Using wavetable oscillator at {}Hz with {}% amplitude", 440.0, 0.2 * 100.0);
    println!("Waveforms will cycle: Sine -> Saw -> Triangle -> Square");
    
    let audio_callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        // Switch waveform every second
        let elapsed = start_time.elapsed().as_secs();
        let current_waveform = match elapsed % 4 {
            0 => Waveform::Sine,
            1 => Waveform::Saw,
            2 => Waveform::Triangle,
            _ => Waveform::Square,
        };
        oscillator.set_waveform(current_waveform);
        
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
    
    println!("Audio pipeline is running with FAUST-inspired wavetables...");
    println!("Listen as it cycles through: Sine -> Saw -> Triangle -> Square");
    std::thread::sleep(std::time::Duration::from_secs(8)); // Extended to hear all waveforms
    println!("Program finished.");
}