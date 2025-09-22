
use eframe::egui::{self, SliderOrientation, Ui};
use std::thread;
use std::time::Duration;
use crossbeam_channel::{Sender, bounded};

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

    let (sender, receiver) = bounded(1);

    thread::spawn(move || {
        let mut current_parameters = ControlParameters::new();

        loop {
            let cross = receiver.try_iter().last();

            match cross {
                Some(new_parameters) => {
                    current_parameters = new_parameters;
                    println!("Parameters updated to: {:?}", current_parameters);
                },
                None => (),
            };

            thread::sleep(Duration::from_millis(100));
        }
    });

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
