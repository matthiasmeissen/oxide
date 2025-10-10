use std::thread;

use crate::state::*;
use crate::dsp::{FaustDsp, ParamIndex};

use crate::dsp::{
    basic_fm::BasicFm,
    simple_sine::SimpleSine,
    drum_engine::DrumEngine,
};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use triple_buffer::Output;


fn dsp_factory(dsp_type: DspType, sample_rate: u32) -> Box<dyn FaustDsp<T = f32> + Send> {
    let mut dsp: Box<dyn FaustDsp<T = f32> + Send> = match dsp_type {
        DspType::BasicFm => Box::new(BasicFm::new()),
        DspType::SimpleSine => Box::new(SimpleSine::new()),
        DspType::DrumEngine => Box::new(DrumEngine::new()),
    };
    dsp.init(sample_rate as i32);
    dsp
}

pub fn start_audio_thread(mut audio_reader: Output<State>) {
    thread::spawn(move || {
        let host = cpal::default_host();
            let device = host.default_output_device().expect("No output device found");
            let config_range = device.supported_output_configs().unwrap()
                .find(|c| c.sample_format() == cpal::SampleFormat::F32)
                .expect("No f32 audio configuration found on this device.");
            let config = config_range.with_max_sample_rate().config();
            
            let sample_rate = config.sample_rate.0;
            let host_channels = config.channels as usize;
    
        println!("Audio Device Initialized:");
        println!("- Sample Rate: {} Hz", sample_rate);
        println!("- Host Channels: {}", host_channels);
    
        let mut current_dsp_type = DspType::default();
        let mut dsp = dsp_factory(current_dsp_type, sample_rate);
        let mut dsp_outputs = dsp.get_num_outputs() as usize;
        println!("- Faust DSP Channels: {}", dsp_outputs);
    
        let max_buffer_size = match *device.default_output_config().unwrap().buffer_size() {
            cpal::SupportedBufferSize::Range { max, .. } => max as usize,
            cpal::SupportedBufferSize::Unknown => 4096,
        };
    
        let mut dsp_output_buffers: Vec<Vec<f32>> = (0..dsp_outputs)
            .map(|_| vec![0.0; max_buffer_size])
            .collect();
    
        let err_fn = |err| eprintln!("an error occurred on the stream: {}", err);
    
        let stream = device.build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let num_frames = data.len() / host_channels;
                let state = audio_reader.read();

                if state.dsp_type != current_dsp_type {
                    dsp = dsp_factory(state.dsp_type, sample_rate);
                    current_dsp_type = state.dsp_type;

                    let new_dsp_outputs = dsp.get_num_outputs() as usize;
                    if new_dsp_outputs != dsp_outputs {
                        dsp_outputs = new_dsp_outputs;
                        dsp_output_buffers = (0..dsp_outputs).map(|_| vec![0.0; max_buffer_size]).collect();
                    }
                }

                for (i, &value) in state.values.iter().enumerate() {
                    if i < 8 {
                        dsp.set_param(ParamIndex(i as i32), value);
                    }
                }
    
                let mut dsp_output_slices: Vec<&mut [f32]> = dsp_output_buffers.iter_mut().map(|buf| &mut buf[..num_frames]).collect();
    
                let dsp_input_slices: Vec<&[f32]> = Vec::new();
    
                dsp.compute(num_frames as i32, &dsp_input_slices, &mut dsp_output_slices);
    
                for i in 0..num_frames {
                    for c in 0..host_channels {
                        let dsp_channel_index = std::cmp::min(c, dsp_outputs - 1);
                        let sample = dsp_output_slices[dsp_channel_index][i];
                        data[i * host_channels + c] = sample;
                    }
                }
            },
            err_fn,
            None,
        ).unwrap();
    
        stream.play().unwrap();
    
        println!("Audio engine initialized and running.");

        loop {
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });
}
