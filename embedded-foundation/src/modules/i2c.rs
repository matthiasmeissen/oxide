
use crate::modules::state::*;
use linux_embedded_hal::I2cdev;
use embedded_hal::blocking::i2c::Read;
use crossbeam_channel::Sender;
use std::thread;
use std::time::Duration;
use std::convert::TryInto;

const STATE_SIZE: usize = 7;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct DeviceState {
    button1: bool,
    button2: bool,
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
                        encoder_button: buffer[2] != 0,
                        encoder_value: {
                            let bytes: [u8; 4] = buffer[3..7].try_into().unwrap_or_default();
                            i32::from_le_bytes(bytes)
                        },
                    };

                    if let Some(prev_state) = last_state {
                        if current_state.encoder_value != prev_state.encoder_value {
                            let normalized_value = normalize_encoder(current_state.encoder_value);
                            sender.send(Message::SetValue(0, normalized_value)).unwrap();
                        }

                        if current_state.button1 != prev_state.button1 {
                            let value = if current_state.button1 { 1.0 } else { 0.0 };
                            sender.send(Message::SetValue(4, value)).unwrap();
                        }

                        if current_state.button2 != prev_state.button2 {
                            let value = if current_state.button2 { 1.0 } else { 0.0 };
                            sender.send(Message::SetValue(5, value)).unwrap();
                        }

                        if current_state.encoder_button != prev_state.encoder_button {
                            let value = if current_state.encoder_button { 1.0 } else { 0.0 };
                            sender.send(Message::SetValue(6, value)).unwrap();
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

fn normalize_encoder(value: i32) -> f64 {
    const ENC_MIN: i32 = -128;
    const ENC_MAX: i32 = 128;
    let value = value.clamp(ENC_MIN, ENC_MAX);
    (value - ENC_MIN) as f64 / (ENC_MAX - ENC_MIN) as f64
}
