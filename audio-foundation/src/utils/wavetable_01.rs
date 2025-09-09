use std::f32;

#[derive(Clone, Copy, Debug, Default)]
pub enum Waveform {
    #[default]
    Sine,
    Saw,
    Triangle,
    Square,
}

// FAUST-inspired wavetable oscillator with multiple waveforms
pub struct WavetableOscillator {
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
    pub fn new(frequency: f32, sample_rate: f32, amplitude: f32, waveform: Waveform) -> Self {
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
    
    pub fn set_frequency(&mut self, frequency: f32, sample_rate: f32) {
        self.phase_increment = frequency / sample_rate;
    }
    
    pub fn set_waveform(&mut self, waveform: Waveform) {
        self.waveform = waveform;
    }
    
    pub fn set_amplitude(&mut self, amplitude: f32) {
        self.amplitude = amplitude;
    }
    
    pub fn get_sample(&mut self) -> f32 {
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
    
    pub fn get_stereo(&mut self) -> (f32, f32) {
        let sample = self.get_sample();
        (sample, sample)
    }
}