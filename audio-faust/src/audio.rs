use std::thread;

use crate::dsp::{basic_fm::*};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub fn start_audio_thread() {
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
    
        let mut dsp = BasicFm::new();
        dsp.init(sample_rate as i32);
        let dsp_outputs = dsp.get_num_outputs() as usize;
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

                dsp.set_param(ParamIndex(0), 200.0);
    
                let mut dsp_output_slices: Vec<&mut [f32]> = dsp_output_buffers
                    .iter_mut()
                    .map(|buf| &mut buf[..num_frames])
                    .collect();
    
                let dsp_input_slices: Vec<&[f32]> = Vec::new();
    
                dsp.compute(num_frames, &dsp_input_slices, &mut dsp_output_slices);
    
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
