
mod screens;
mod state;
use screens::*;
use state::*;

use embedded_graphics::{
    pixelcolor::{BinaryColor}, 
    prelude::{Size}, 
    draw_target::*};
use embedded_graphics_simulator::*;

use std::thread;
use std::time::Duration;

// To run the simulator run this command in the terminal
// export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
// This will enable it for that session only


fn main() -> Result<(), std::convert::Infallible> {
    println!("Embedded Foundation");

    let mut state = PlaceholderState::new();

    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledWhite)
        .build();

    let mut window = Window::new("Window Title", &output_settings);

    'running: loop {
        display.clear(BinaryColor::Off)?;
        draw_basic_screen(&mut display, &state)?;
        window.update(&display);

        if window.events().any(|e| e == SimulatorEvent::Quit) {
            break 'running Ok(());
        }

        state.update_time(0.01);

        thread::sleep(Duration::from_millis(100));
    }
}
