
use eframe::egui::{self, SliderOrientation, Ui};

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    // Create App instance to use in run_native()
    let app = MyApp::new();

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
    value_01: f64,
    values: [f64; 4],
    description: String,
}

impl MyApp {
    fn new() -> Self {
        Self { 
            value_01: 0.0,
            values: [0.0; 4],
            description: String::from("How to use this."),
        }
    }

    fn create_infos(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.label("Change description");
            ui.text_edit_singleline(&mut self.description);
        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Title");
            ui.label("Adjust the value");

            ui.add(egui::Slider::new(&mut self.value_01, 0.0..=1.0)
                .text("Simple Value")
                .orientation(SliderOrientation::Vertical)
            );

            for val in &mut self.values {
                ui.add(egui::Slider::new(val, 0.0..=1.0).text("Value"));
            }

            self.create_infos(ui);
        });
    }
}
