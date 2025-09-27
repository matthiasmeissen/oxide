#![cfg(not(target_os = "linux"))]

use crate::modules::{
    state::*, 
    screens::*
};

use embedded_graphics::{
    pixelcolor::{BinaryColor}, 
    prelude::{Size}, 
    draw_target::*};
use embedded_graphics_simulator::*;

use triple_buffer::*;

use std::{thread, time::Duration};

pub fn start_display_simulator(mut display_reader: Output<State>) {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledWhite)
        .build();

    let mut window = Window::new("Window Title", &output_settings);

    'running: loop {
        let state = display_reader.read();

        display.clear(BinaryColor::Off).unwrap();

        match state.screen_index {
            0 => draw_screen(&mut display, &state),
            _ => draw_debug(&mut display, &state),
        }

        window.update(&display);

        if window.events().any(|e| e == SimulatorEvent::Quit) {
            break 'running;
        }

        thread::sleep(Duration::from_millis(40));
    }
}
