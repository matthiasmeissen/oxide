
use crate::modules::state::*;
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
                        if current_state.button1 != prev_state.button1 {
                            let value = if current_state.button1 { 1.0 } else { 0.0 };
                            sender.send(Message::SetValue(4, value)).unwrap();
                        }

                        if current_state.button2 != prev_state.button2 {
                            let value = if current_state.button2 { 1.0 } else { 0.0 };
                            sender.send(Message::SetValue(5, value)).unwrap();
                        }

                        if current_state.button3 != prev_state.button3 {
                            let value = if current_state.button3 { 1.0 } else { 0.0 };
                            sender.send(Message::SetValue(6, value)).unwrap();
                        }

                        if current_state.button4 != prev_state.button4 {
                            let value = if current_state.button4 { 1.0 } else { 0.0 };
                            sender.send(Message::SetValue(7, value)).unwrap();
                        }

                        if current_state.pot1_value != prev_state.pot1_value {
                            sender.send(Message::SetValue(0, normalize_pot(current_state.pot1_value))).unwrap();
                        }

                        if current_state.pot2_value != prev_state.pot2_value {
                            sender.send(Message::SetValue(1, normalize_pot(current_state.pot2_value))).unwrap();
                        }

                        if current_state.pot3_value != prev_state.pot3_value {
                            sender.send(Message::SetValue(2, normalize_pot(current_state.pot3_value))).unwrap();
                        }

                        if current_state.pot4_value != prev_state.pot4_value {
                            sender.send(Message::SetValue(3, normalize_pot(current_state.pot4_value))).unwrap();
                        }

                        if current_state.encoder_button != prev_state.encoder_button {
                            let value = if current_state.encoder_button { 1.0 } else { 0.0 };
                            sender.send(Message::SetValue(6, value)).unwrap();
                        }

                        if current_state.encoder_value != prev_state.encoder_value {
                            sender.send(Message::SetValue(0, normalize_enc(current_state.encoder_value))).unwrap();
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

fn normalize_enc(value: i32) -> f64 {
    const ENC_MIN: i32 = -128;
    const ENC_MAX: i32 = 128;
    let value = value.clamp(ENC_MIN, ENC_MAX);
    (value - ENC_MIN) as f64 / (ENC_MAX - ENC_MIN) as f64
}

fn normalize_pot(value: i16) -> f64 {
    value as f64 / 1024.0
}
