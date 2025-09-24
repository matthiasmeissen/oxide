use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Line, PrimitiveStyle},
};
use linux_embedded_hal::I2cdev;

// --- THE FIX IS HERE ---
// We must explicitly import `I2CDisplayInterface` from the `sh1106` crate.
use sh1106::{prelude::*, I2CDisplayInterface, Sh1106};

fn main() {
    // I2C setup is exactly the same
    let i2c = I2cdev::new("/dev/i2c-1").expect("Failed to open I2C device");

    // Now the compiler knows what `I2CDisplayInterface` is.
    let interface = I2CDisplayInterface::new(i2c);

    // Create an Sh1106 driver instance
    let mut display = Sh1106::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    // The rest of the code remains the same
    display.init().expect("Failed to initialize display");

    display.clear(BinaryColor::Off).expect("Clear failed");

    let line_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    Line::new(Point::new(0, 0), Point::new(127, 63))
        .into_styled(line_style)
        .draw(&mut display)
        .expect("Line draw failed");

    display.flush().expect("Flush failed");

    println!("Line should be visible on your SH1106 screen!");
    println!("Program will now loop forever. Press Ctrl+C to exit.");

    loop {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}