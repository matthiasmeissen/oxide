use winit::{
    event::*, event_loop::EventLoop, keyboard::{Key, KeyCode, PhysicalKey}, window::WindowBuilder
};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use fundsp::hacker::*;

fn main() {
    let trigger = shared(0.0);
    let pitch = shared(440.0);
    let gain = shared(0.0);

    
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
    let osc1 = var(&pitch) >> sine();
    let osc2 = var(&pitch) >> saw();
    let osc = (osc1 + osc2) * 0.5;
    let filter = osc >> lowrez_hz(800.0, 1.0);
    let synth= filter * env * var(&gain);

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
            Event::WindowEvent {event, ..} => {
                match event {
                    WindowEvent::CloseRequested {} => {
                        println!("The close button was pressed.");
                        elwt.exit();
                    }
                    WindowEvent::CursorMoved { device_id , position } => {
                        println!("Mouse position: {:?}", position);
                        let pitch_hz = xerp(100.0, 2000.0, position.x / window_size.width as f64);
                        pitch.set_value(pitch_hz);
                    },
                    WindowEvent::KeyboardInput { event, .. } => {
                        match (event.physical_key, event.state) {
                            (PhysicalKey::Code(KeyCode::KeyA), ElementState::Pressed) => {
                                println!("Key A pressed.");
                                trigger.set_value(1.0);
                                gain.set_value(1.0);
                            },
                            (PhysicalKey::Code(KeyCode::KeyA), ElementState::Released) => {
                                println!("Key A Released.");
                                trigger.set_value(0.0);
                            }
                            _ => ()
                        }
                    }
                    _ => (),
                }
            },
            _ => ()
        }
    }).unwrap();
}
