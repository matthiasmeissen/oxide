use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Line, PrimitiveStyle},
};
use linux_embedded_hal::I2cdev;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // --- 1. SET UP THE DISPLAY ---

    // Initialize a connection to the I2C bus on the Raspberry Pi.
    // "/dev/i2c-1" is the standard I2C bus for the GPIO pins.
    let i2c = I2cdev::new("/dev/i2c-1").expect("Failed to open I2C device");

    // Create an interface to the display.
    let interface = I2CDisplayInterface::new(i2c);

    // Create a driver instance for an SSD1306 display with a size of 128x64 pixels.
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    // Initialize the display driver. This will send initialization commands to the screen.
    display.init().expect("Failed to initialize display");

    // --- 2. DRAW ON THE DISPLAY ---

    // First, clear the display's internal buffer of any old data.
    display.clear(BinaryColor::Off)?;

    // Define the style for our line: a 1-pixel wide, white line.
    let line_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);

    // Create a line from the top-left corner (0,0) to the bottom-right (127, 63).
    Line::new(Point::new(0, 0), Point::new(127, 63))
        .into_styled(line_style)
        .draw(&mut display)?; // Draw the line into the display's buffer.

    // --- 3. SHOW THE DRAWING ---

    // Flush the buffer to the display.
    // This sends the data from the Pi's memory to the screen's memory.
    display.flush()?;

    println!("Line should be visible on the OLED screen.");
    println!("Program will now loop forever. Press Ctrl+C to exit.");

    // Loop forever to keep the image on the screen.
    // If the program exits, the screen will likely go blank.
    loop {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}