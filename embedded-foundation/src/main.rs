
use linux_embedded_hal::{I2cdev, i2c::I2c};
use std::thread;
use std::time::Duration;

const ESP32_ADDRESS: u16 = 0x08;
const BUTTON_PRESSED_MSG: u8 = 0x01;

fn main() -> Result<(), linux_embedded_hal::i2c::Error> {
    println!("Embedded Foundation");
    // Power ESP from PI
    // The Power LED lights up

    // Connect ESP and PI over I2C 
    // Specified I2C Address from ESP shows up in PI monitor

    // Connect OLED and ESP over I2C
    // Draw basic screen
    // Use PI to request rata from ESP

    // Initialize I2C on bus 1 (/dev/i2c-1)
    let mut i2c = I2cdev::new("/dev/i2c-1").unwrap();

    // Set the I2C slave address we want to communicate with
    i2c.set_slave_address(ESP32_ADDRESS).unwrap();

    println!("Listening for button press events from ESP32 at address {:#04x}...", ESP32_ADDRESS);

    // Main loop to continuously poll the ESP32
    loop {
        // Create a buffer to store the single byte we're reading
        let mut buffer = [0u8; 1];

        // Read 1 byte from the ESP32
        match i2c.read(&mut buffer) {
            Ok(_) => {
                // Check if the received byte is our "button pressed" message
                if buffer[0] == BUTTON_PRESSED_MSG {
                    println!("EVENT: Button press detected on ESP32!");
                }
                // If buffer[0] is 0x00 or anything else, we just ignore it and loop again.
            }
            Err(e) => {
                eprintln!("Error reading from I2C bus: {:?}", e);
            }
        }
        
        // Wait for 100 milliseconds before polling again to avoid spamming the I2C bus
        thread::sleep(Duration::from_millis(100));
    }
}
