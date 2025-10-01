
use linux_embedded_hal::I2cdev;
use embedded_hal::blocking::i2c::Read;
use std::thread;
use std::time::Duration;

const STATE_SIZE: usize = 5;

fn main() {
    let mut i2c = I2cdev::new("/dev/i2c-1").unwrap();
    i2c.set_slave_address(0x08).unwrap();

    println!("Listening for button press events from ESP32 at address {:#04x}...", 0x08);
    let mut last_button_state = false;

    loop {
        let mut buffer = [0u8; STATE_SIZE];

        // Read 1 byte from the ESP32
        match i2c.read(0x08, &mut buffer) {
            Ok(_) => {
                //println!("Received value: {:#04x} ({})", buffer[0], buffer[0]);

                // The first byte (buffer[0]) is the boolean. 1 = true, 0 = false.
                let button_is_pressed = buffer[0] != 0;

                // The next 4 bytes (buffer[1] through buffer[4]) are the i32.
                // We need to convert this slice of the buffer into a fixed-size array.
                //let encoder_bytes: [u8; 4] = buffer[1..5].try_into()?;
                
                // The ESP32 is little-endian, so we parse the bytes in that order.
                //let encoder_value = i32::from_le_bytes(encoder_bytes);

                // --- Act on the state ---
                // To avoid spamming the console, let's only print on change.
                if button_is_pressed != last_button_state {
                    println!("State Change -> Button Pressed: {}", button_is_pressed);
                    last_button_state = button_is_pressed;
                }
            }
            Err(e) => {
                eprintln!("Error reading from I2C bus: {:?}", e);
            }
        }
        
        // Wait for 100 milliseconds before polling again to avoid spamming the I2C bus
        thread::sleep(Duration::from_millis(100));
    }
}
