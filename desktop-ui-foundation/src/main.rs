
use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    let mut value = 0.0;

    eframe::run_simple_native(
        "Window Basics", 
        options, 
        move |ctx, _frame| {
            egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Simple Slider");
            ui.add(egui::Slider::new(&mut value, 0.0..=120.0).text("Value"));
        });
        }
    )
}
