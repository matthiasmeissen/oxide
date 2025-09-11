use std::time::Instant;
use winit::{event::*, event_loop::{ControlFlow, EventLoop}, keyboard::{PhysicalKey, KeyCode}, window::{Window, WindowBuilder}};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use fundsp::hacker::*;

struct State {
    window: Window,
    start_time: Instant,
    notes: Vec<f64>,
    bpm: f64,
}

impl State {
    fn new(event_loop: &EventLoop<()>) -> Self {
        let window = WindowBuilder::new()
            .with_title("Audio Foundation")
            .build(&event_loop)
            .unwrap();

        let start_time = Instant::now();
        let notes = vec![
            midi_hz(60.0),
            midi_hz(62.0),
            midi_hz(64.0),
            midi_hz(67.0),
        ];

        Self { window, start_time, notes, bpm: 120.0 }
    }

    fn get_sequencer_value(&self) -> f64 {
        let elapsed_time = self.start_time.elapsed().as_secs_f64();
        let period= 60.0 / self.bpm;
        let phasor = (elapsed_time % period) / period;

        let index = (phasor * self.notes.len() as f64).floor() as usize;
        self.notes[index]
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
    let sample_rate = stream_config.sample_rate.0 as f64;
    let channels = stream_config.channels as usize;

    let freq = shared(440.0);
    let synth= var(&freq) >> sine();
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

    let mut state = State::new(&event_loop);

    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);
        match event {
            Event::WindowEvent {event, ..} => {
                match event {
                    WindowEvent::CloseRequested {} => {
                        println!("The close button was pressed.");
                        elwt.exit();
                    }
                    WindowEvent::CursorMoved { position , ..} => {
                        let x = position.x / state.window.inner_size().width as f64;
                        let y = position.y / state.window.inner_size().height as f64;
                        println!("Mouse position: {x}, {y}");

                        state.bpm = lerp(80.0, 120.0, x);
                    },
                    WindowEvent::KeyboardInput { event, .. } => {
                        match (event.physical_key, event.state) {
                            (PhysicalKey::Code(KeyCode::KeyA), ElementState::Pressed) => {
                                println!("Key A pressed.");
                                state.notes.push(midi_hz(48.0));

                                println!("Notes is now: {:?}", state.notes);

                            },
                            (PhysicalKey::Code(KeyCode::KeyA), ElementState::Released) => {
                                println!("Key A Released.");
                            }
                            _ => ()
                        }
                    }
                    WindowEvent::RedrawRequested {} => {
                        freq.set_value(state.get_sequencer_value());

                        state.window.request_redraw();
                    }
                    _ => (),
                }
            },
            _ => ()
        }
    }).unwrap();
}
