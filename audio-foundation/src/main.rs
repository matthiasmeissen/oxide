use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use fundsp::hacker::*;

fn main() {
    let trigger = shared(0.0);
    let pitch = shared(440.0);

    
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


    let synth= var(&pitch) >> sine();

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


    // Window Creation
    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_title("Audio Foundation")
        .build(&event_loop)
        .unwrap();
    let window_size = window.inner_size();

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                println!("The close button was pressed.");
                elwt.exit();
            }
            Event::WindowEvent { event: WindowEvent::CursorMoved { device_id, position }, .. } => {
                println!("Mouse position: {:?}", position);
                let pitch_hz = xerp(100.0, 2000.0, position.x / window_size.width as f64);
                    // Set the pitch from the main thread.
                    pitch.set_value(pitch_hz);
            }
            _ => ()
        }
    }).unwrap();
}
