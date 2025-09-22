
use eframe::egui::{self, SliderOrientation, Ui};
use std::thread;
use std::time::Duration;
use crossbeam_channel::{bounded, Receiver, Sender};
use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, *};
use fundsp::hacker32::*;

#[derive(Clone, Copy, Debug)]
struct ControlParameters {
    value_01: f64,
    value_02: f64,
}

impl ControlParameters {
    fn new() -> Self {
        Self { 
            value_01: 0.2, 
            value_02: 0.8 
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let (sender, receiver): (Sender<ControlParameters>, Receiver<ControlParameters>) = bounded(1);

    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("Could not find output device.");
    println!("Output Device: {:?}", device.name());
    let config = device.default_output_config().unwrap();
    let sample_format = config.sample_format();
    let stream_config: cpal::StreamConfig = config.into();
    println!("Stream config: {:?}", stream_config);
    let sample_rate = stream_config.sample_rate.0 as f64;
    let channels = stream_config.channels as usize;

    let freq1 = shared(440.0);
    let freq2 = shared(880.0);

    let osc1 = var(&freq1) * 8000.0 >> sine();
    let osc2 = var(&freq2) * 2000.0 >> sine();
    let synth = (osc1 + osc2) * 0.5;

    let mut graph = synth * 0.2;
    graph.set_sample_rate(sample_rate);

    let mut next_value = move || graph.get_stereo();

    let audio_callback = move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {

        while let Ok(params) = receiver.try_recv() {
            freq1.set_value(params.value_01 as f32);
            freq2.set_value(params.value_02 as f32);
        }

        for frame in data.chunks_mut(channels) {
                let (l, r) = next_value();
                frame[0] = l;
                if channels > 1 {
                    frame[1] = r;
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

    let app = MyApp::new(sender);

    eframe::run_native(
        "App Title", 
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            println!("Initializing Window with {:?}", cc.gl);
            Ok(Box::new(app))
        }),
    )
}

struct MyApp {
    sender: Sender<ControlParameters>,
    parameters: ControlParameters,
}

impl MyApp {
    fn new(sender: Sender<ControlParameters>) -> Self {
        Self { 
            sender,
            parameters: ControlParameters::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Title");

            let mut has_changed = false;

            if ui.add(egui::Slider::new(&mut self.parameters.value_01, 0.0..=1.0).text("Value 1")).changed() {
                has_changed = true;
            };

            if ui.add(egui::Slider::new(&mut self.parameters.value_02, 0.0..=1.0).text("Value 2")).changed() {
                has_changed = true;
            };

            if has_changed {
                self.sender.try_send(self.parameters).ok();
            }
        });
    }
}
