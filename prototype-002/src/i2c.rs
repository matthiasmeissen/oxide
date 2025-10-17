#![cfg(target_os = "linux")]

use crate::state::*;
use linux_embedded_hal::I2cdev;
use embedded_hal::blocking::i2c::Read;
use crossbeam_channel::Sender;
use std::thread;
use std::time::Duration;
use std::convert::TryInto;

const STATE_SIZE: usize = 17;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct DeviceState {
    button1: bool,
    button2: bool,
    button3: bool,
    button4: bool,
    pot1_value: i16,
    pot2_value: i16,
    pot3_value: i16,
    pot4_value: i16,
    encoder_button: bool,
    encoder_value: i32,
}

pub fn start_i2c_thread(sender: Sender<Message>) {
    thread::spawn(move || {
        let mut i2c = I2cdev::new("/dev/i2c-1").expect("Failed to open I2C connection.");
        i2c.set_slave_address(0x08).unwrap();

        println!("I2C thread started, listening to ESP32");

        let mut last_state: Option<DeviceState> = None;

        loop {
            let mut buffer = [0u8; STATE_SIZE];

            match i2c.read(0x08, &mut buffer) {
                Ok(_) => {
                    let current_state = DeviceState {
                        button1: buffer[0] != 0,
                        button2: buffer[1] != 0,
                        button3: buffer[2] != 0,
                        button4: buffer[3] != 0,
                        pot1_value: i16::from_le_bytes(buffer[4..6].try_into().unwrap_or_default()),
                        pot2_value: i16::from_le_bytes(buffer[6..8].try_into().unwrap_or_default()),
                        pot3_value: i16::from_le_bytes(buffer[8..10].try_into().unwrap_or_default()),
                        pot4_value: i16::from_le_bytes(buffer[10..12].try_into().unwrap_or_default()),
                        encoder_button: buffer[12] != 0,
                        encoder_value: i32::from_le_bytes(buffer[13..17].try_into().unwrap_or_default()),
                    };

                    if let Some(prev_state) = last_state {
                        handle_button_change(prev_state.button1, current_state.button1, 4, &sender);
                        handle_button_change(prev_state.button2, current_state.button2, 5, &sender);
                        handle_button_change(prev_state.button3, current_state.button3, 6, &sender);
                        handle_button_change(prev_state.button4, current_state.button4, 7, &sender);

                        handle_pot_change(prev_state.pot1_value, current_state.pot1_value, 0, &sender);
                        handle_pot_change(prev_state.pot2_value, current_state.pot2_value, 1, &sender);
                        handle_pot_change(prev_state.pot3_value, current_state.pot3_value, 2, &sender);
                        handle_pot_change(prev_state.pot4_value, current_state.pot4_value, 3, &sender);

                        match (prev_state.encoder_button, current_state.encoder_button) {
                            (false, true) => sender.send(Message::IncrementScreenIndex).unwrap(),
                            _ => (),
                        }

                        if current_state.encoder_value != prev_state.encoder_value {
                            sender.send(Message::SetShaderIndex(current_state.encoder_value as usize)).unwrap();
                        }
                    }

                    last_state = Some(current_state);
                }
                Err(e) => {
                    eprintln!("Error reading from I2C bus: {:?}", e);
                }
            }
            
            thread::sleep(Duration::from_millis(10));
        }
    });
}

fn normalize_enc(value: i32) -> f32 {
    const ENC_MIN: i32 = -128;
    const ENC_MAX: i32 = 128;
    let value = value.clamp(ENC_MIN, ENC_MAX);
    (value - ENC_MIN) as f32 / (ENC_MAX - ENC_MIN) as f32
}

fn normalize_pot(value: i16) -> f32 {
    value as f32 / 1024.0
}

fn handle_button_change(prev: bool, current: bool, index: usize, sender: &Sender<Message>) {
    if prev != current {
        let value = if current { 1.0 } else { 0.0 };
        sender.send(Message::SetValue(index, value)).unwrap();
    }
}

fn handle_pot_change(prev: i16, current: i16, index: usize, sender: &Sender<Message>) {
    if prev != current {
        sender.send(Message::SetValue(index, normalize_pot(current))).unwrap();
    }
}
