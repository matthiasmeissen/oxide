
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

            // Read 1 byte from the ESP32
            match i2c.read(0x08, &mut buffer) {
                Ok(_) => {
                    // byte 0: button1
                    let button1 = buffer[0] != 0;

                    // byte 1: button2
                    let button2 = buffer[1] != 0;

                    // byte 2: encoder_button
                    let encoder_button = buffer[2] != 0;

                    // bytes 3-6: encoder_value (i32)
                    let encoder_bytes: [u8; 4] = buffer[3..7].try_into().unwrap();
                    let encoder_value = i32::from_le_bytes(encoder_bytes);

                    let current_state = DeviceState {
                        button1,
                        button2,
                        encoder_button,
                        encoder_value,
                    };

                    if last_state.as_ref() != Some(&current_state) {
                        println!("State Change -> {:?}", current_state);
                        last_state = Some(current_state);

                        let normalized_value = normalize_encoder(current_state.encoder_value);
                        sender.send(Message::SetValue(0, normalized_value)).unwrap();

                        let value = if current_state.button1 { 1.0 } else { 0.0 };
                        sender.send(Message::SetValue(4, value)).unwrap();
                    }
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
    const ENC_MIN: i32 = -1000;
    const ENC_MAX: i32 = 1000;
    let value = value.clamp(ENC_MIN, ENC_MAX);
    (value - ENC_MIN) as f64 / (ENC_MAX - ENC_MIN) as f64
}
