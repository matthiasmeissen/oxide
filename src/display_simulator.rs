#![cfg(not(target_os = "linux"))]

use crate::{state::*, screens::*};

use embedded_graphics::{
    pixelcolor::{BinaryColor}, 
    prelude::{Size}, 
    draw_target::*};
use embedded_graphics_simulator::*;
use embedded_graphics_simulator::sdl2::Keycode;

use crossbeam_channel::Sender;
use triple_buffer::*;

use std::{thread, time::Duration};

pub fn start_display_simulator(display_sender: Sender<Message>, mut display_reader: Output<DisplayState>) {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledWhite)
        .build();

    let mut window = Window::new("Window Title", &output_settings);

    'running: loop {
        let state = display_reader.read();

        display.clear(BinaryColor::Off).unwrap();

        draw(&mut display, &state);

        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::KeyDown { keycode, .. } => {
                    match keycode {
                        Keycode::Right => {display_sender.try_send(Message::UiInput(InputEvent::Next)).ok()},
                        Keycode::Down => {display_sender.try_send(Message::UiInput(InputEvent::Next)).ok()},
                        Keycode::Left => {display_sender.try_send(Message::UiInput(InputEvent::Prev)).ok()},
                        Keycode::Up => {display_sender.try_send(Message::UiInput(InputEvent::Prev)).ok()},
                        Keycode::SPACE => {display_sender.try_send(Message::UiInput(InputEvent::Enter)).ok()},
                        _ => Some(()),
                    };
                }
                _ => {}
            }
        }

        thread::sleep(Duration::from_millis(40));
    }
}
