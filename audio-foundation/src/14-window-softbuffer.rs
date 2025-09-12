use std::{num::NonZeroU32, sync::mpsc, time::Instant};
use winit::{event::*, event_loop::{ControlFlow, EventLoop}, keyboard::{PhysicalKey, KeyCode}, window::{Window, WindowBuilder}};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use fundsp::hacker::*;
use midir::{Ignore, MidiInput};
// We need to specify the full path for Context and Surface when using them with lifetimes
use softbuffer::{Context, Surface};

// --- MODIFIED ---
// 1. We add a lifetime `'a` to the struct.
// 2. The struct now holds the context and surface.
// 3. The `window` is now a borrowed reference `&'a Window`.
struct State<'a> {
    window: &'a Window,
    context: Context<&'a Window>,
    surface: Surface<&'a Window, &'a Window>,
    start_time: Instant,
    notes: Vec<f64>,
    bpm: f64,
    tempo_index: usize,
    tempo_options: Vec<f64>,
}

// --- MODIFIED ---
// The impl block is also generic over the lifetime `'a`.
impl<'a> State<'a> {
    // `new` now accepts a reference to the window that it will borrow.
    fn new(window: &'a Window) -> Self {
        let context = Context::new(window).unwrap();
        let surface = Surface::new(&context, window).unwrap();

        let start_time = Instant::now();
        let notes = vec![
            midi_hz(60.0), midi_hz(62.0), midi_hz(64.0), midi_hz(67.0),
        ];
        let tempo_options = vec![0.1, 0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 2.0];

        Self {
            window,
            context,
            surface,
            start_time,
            notes,
            bpm: 120.0,
            tempo_index: 4,
            tempo_options,
        }
    }

    // No changes needed for the methods below
    fn get_sequencer_value(&self) -> f64 {
        let elapsed_time = self.start_time.elapsed().as_secs_f64();
        let period = 60.0 / self.bpm / self.tempo_options[self.tempo_index];
        let phasor = (elapsed_time % period) / period;
        let index = (phasor * self.notes.len() as f64).floor() as usize;
        self.notes[index]
    }

    fn trigger_envelope(&self) -> f64 {
        let elapsed_time = self.start_time.elapsed().as_secs_f64();
        let period = 60.0 / self.bpm / self.tempo_options[self.tempo_index] / self.notes.len() as f64;
        let phasor = (elapsed_time % period) / period;
        if phasor < 0.2 { 1.0 } else { 0.0 }
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
    // --- Audio and MIDI setup is unchanged ---
    let host = cpal::default_host();
    let device = host.default_output_device().expect("No output device available.");
    let config = device.default_output_config().expect("No default output config found.");
    let sample_format = config.sample_format();
    let stream_config: cpal::StreamConfig = config.into();
    let sample_rate = stream_config.sample_rate.0 as f64;
    let channels = stream_config.channels as usize;

    let freq = shared(440.0);
    let modulator = shared(5.0);
    let trigger = shared(0.0);
    let cutoff = shared(800.0);

    let fm_synth = oversample(var(&freq) >> sine() * var(&freq) * var(&modulator) + var(&freq) >> sine());
    let filter = (pass() | var(&cutoff) | dc(0.8)) >> lowrez();
    let env = var(&trigger) >> adsr_live(0.002, 0.0, 1.0, 0.1);
    let synth= fm_synth >> filter * env;

    let mut node = synth * 0.2;
    node.set_sample_rate(sample_rate);

    let mut next_value = move || node.get_stereo();
    let audio_callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
        for frame in data.chunks_mut(channels) {
            let (l, r) = next_value();
            frame[0] = l as f32;
            if channels > 1 { frame[1] = r as f32; }
        }
    };

    let err_fn = |err| eprintln!("An error occurred on the audio stream: {}", err);

    let stream = match sample_format {
        cpal::SampleFormat::F32 => device.build_output_stream(&stream_config, audio_callback, err_fn, None),
        _ => Err(cpal::BuildStreamError::StreamConfigNotSupported),
    }.expect("Could not build f32 output stream.");

    stream.play().expect("Could not start audio stream.");

    let (sender, receiver) = mpsc::channel();
    std::thread::spawn(move || {
        let mut midi_in = MidiInput::new("Midi Input 1").unwrap();
        midi_in.ignore(Ignore::None);
        if let Some(port) = midi_in.ports().get(0) {
            let _connection = midi_in.connect(port, "Midi Input", move|_, message, _| {
                sender.send(message.to_vec()).unwrap();
            }, ()).unwrap();
            loop {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    });

    let event_loop = EventLoop::new().unwrap();

    // --- MODIFIED ---
    // The window is now created and owned here in `main`.
    let window = WindowBuilder::new()
        .with_title("Audio Foundation")
        .build(&event_loop)
        .unwrap();

    // We pass a reference `&window` to create the state.
    let mut state = State::new(&window);

    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);

        if let Ok(message) = receiver.try_recv() {
            if message[0] == 176 {
                let input = message[2] as f64 / 128.0;
                let val = xerp11(100.0, 4000.0, input);
                cutoff.set_value(val);
            }
            if message[0] == 144 {
                let midi_note = message[1] as f64;
                if state.notes.last() != Some(&midi_hz(midi_note)) {
                    state.notes.push(midi_hz(midi_note));
                }
            }
        }

        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested {} => elwt.exit(),
                    WindowEvent::CursorMoved { position, .. } => {
                        let y = position.y / state.window.inner_size().height as f64;
                        modulator.set_value(y * 10.0);
                    },
                    WindowEvent::KeyboardInput { event, .. } => {
                        if event.state == ElementState::Pressed {
                             match event.physical_key {
                                PhysicalKey::Code(KeyCode::ArrowUp) => state.bpm += 1.0,
                                PhysicalKey::Code(KeyCode::ArrowDown) => state.bpm -= 1.0,
                                PhysicalKey::Code(KeyCode::ArrowRight) => state.increase_tempo(),
                                PhysicalKey::Code(KeyCode::ArrowLeft) => state.decrease_tempo(),
                                _ => ()
                            }
                        }
                    }
                    WindowEvent::RedrawRequested {} => {
                        freq.set_value(state.get_sequencer_value());
                        trigger.set_value(state.trigger_envelope());

                        let (width, height) = {
                            let size = state.window.inner_size();
                            (size.width, size.height)
                        };

                        // --- MODIFIED ---
                        // Now we call resize on the surface inside the state
                        state.surface.resize(
                            NonZeroU32::new(width).unwrap(),
                            NonZeroU32::new(height).unwrap(),
                        ).unwrap();

                        let mut buffer = state.surface.buffer_mut().unwrap();
                        //let color = if state.trigger_envelope() > 0.0 { 0x00FF00 } else { 0x101010 };
                        buffer.fill(0x00FF00);
                        buffer.present().unwrap();

                        state.window.request_redraw();
                    }
                    _ => (),
                }
            },
            _ => (),
        }
    }).unwrap();
}