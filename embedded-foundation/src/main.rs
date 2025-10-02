
use linux_embedded_hal::I2cdev;
use embedded_hal::blocking::i2c::Read;
use std::thread;
use std::time::Duration;
use std::convert::TryInto;

const STATE_SIZE: usize = 7;

#[derive(Debug, PartialEq, Eq)]
struct DeviceState {
    button1: bool,
    button2: bool,
    encoder_button: bool,
    encoder_value: i32,
}

fn main() {
    let mut i2c = I2cdev::new("/dev/i2c-1").unwrap();
    i2c.set_slave_address(0x08).unwrap();

    println!("Listening for button press events from ESP32 at address {:#04x}...", 0x08);

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
                }
            }
            Err(e) => {
                eprintln!("Error reading from I2C bus: {:?}", e);
            }
        }
        
        // Wait for 100 milliseconds before polling again to avoid spamming the I2C bus
        thread::sleep(Duration::from_millis(10));
    }
}
