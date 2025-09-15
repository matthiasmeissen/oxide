use std::{os::macos::raw::stat, time::Instant};
use winit::{event::*, event_loop::{ControlFlow, EventLoop}, keyboard::{PhysicalKey, KeyCode}, window::{Window, WindowBuilder}};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use fundsp::hacker::*;

#[derive(Debug)]
struct State {
    window: Window,
    start_time: Instant,
    notes: Vec<f64>,
    bpm: f64,
    tempo_index: usize,
    tempo_options: Vec<f64>,
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

        let tempo_options = vec![0.1, 0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 2.0];

        Self { 
            window, 
            start_time, 
            notes, 
            bpm: 120.0, 
            tempo_index: 4, 
            tempo_options 
        }
    }

    fn get_sequencer_value(&self) -> f64 {
        let elapsed_time = self.start_time.elapsed().as_secs_f64();
        let period= 60.0 / self.bpm / self.tempo_options[self.tempo_index];
        let phasor = (elapsed_time % period) / period;

        let index = (phasor * self.notes.len() as f64).floor() as usize;
        self.notes[index]
    }

    fn trigger_envelope(&self) -> f64 {
        let elapsed_time = self.start_time.elapsed().as_secs_f64();
        let period= 60.0 / self.bpm / self.tempo_options[self.tempo_index] / self.notes.len() as f64;
        let phasor = (elapsed_time % period) / period;

        if phasor < 0.2 {
            1.0
        } else {
            0.0
        }
    }

    fn increase_tempo(&mut self) {
        if self.tempo_index != self.tempo_options.len() - 1 {
            self.tempo_index += 1;
        }
    }

    fn decrease_tempo(&mut self) {
        if self.tempo_index != 0 {
            self.tempo_index -= 1;
        }
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
    let modulator = shared(5.0);
    let trigger = shared(0.0);

    let fm_synth = oversample(var(&freq) >> sine() * var(&freq) * var(&modulator) + var(&freq) >> sine());
    let filter = (pass() | dc(800.0) | dc(0.8)) >> lowrez();
    let env = var(&trigger) >> adsr_live(0.002, 0.0, 1.0, 0.1);
    let synth= fm_synth >> filter * env;

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

                        modulator.set_value(y * 10.0);
                    },
                    WindowEvent::KeyboardInput { event, .. } => {
                        match (event.physical_key, event.state) {
                            (PhysicalKey::Code(KeyCode::ArrowUp), ElementState::Pressed) => {
                                state.bpm += 1.0;
                                println!("Set BPM to {}", state.bpm);
                            },
                            (PhysicalKey::Code(KeyCode::ArrowDown), ElementState::Pressed) => {
                                state.bpm -= 1.0;
                                println!("Set BPM to {}", state.bpm);
                            },
                            (PhysicalKey::Code(KeyCode::ArrowRight), ElementState::Pressed) => {
                                println!("{:?}", state);
                                state.increase_tempo();
                            },
                            (PhysicalKey::Code(KeyCode::ArrowLeft), ElementState::Pressed) => {
                                println!("{:?}", state);
                                state.decrease_tempo();
                            },
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

                        trigger.set_value(state.trigger_envelope());

                        state.window.request_redraw();
                    }
                    _ => (),
                }
            },
            _ => ()
        }
    }).unwrap();
}
