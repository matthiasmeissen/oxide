use std::error::Error;
use std::thread;
use std::time::Duration;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Line, PrimitiveStyle, Rectangle},
    text::Text,
};
use linux_embedded_hal::I2cdev;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize I2C
    let i2c = I2cdev::new("/dev/i2c-1")?;
    
    // Initialize display interface
    let interface = I2CDisplayInterface::new(i2c);
    
    // Initialize display driver
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    
    // Initialize the display
    display.init()?;
    
    // Clear the display
    display.clear();
    
    // Create a text style
    let text_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    
    // Draw some text
    Text::new("Hello, Pi 5!", Point::new(10, 20), text_style)
        .draw(&mut display)?;
    
    Text::new("Rust + OLED", Point::new(10, 35), text_style)
        .draw(&mut display)?;
    
    // Draw some shapes
    let line_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let fill_style = PrimitiveStyle::with_fill(BinaryColor::On);
    
    // Draw a line
    Line::new(Point::new(0, 50), Point::new(127, 50))
        .into_styled(line_style)
        .draw(&mut display)?;
    
    // Draw a rectangle
    Rectangle::new(Point::new(100, 10), Size::new(20, 15))
        .into_styled(line_style)
        .draw(&mut display)?;
    
    // Draw a filled circle
    Circle::new(Point::new(5, 5), 8)
        .into_styled(fill_style)
        .draw(&mut display)?;
    
    // Send buffer to display
    display.flush()?;
    
    println!("Display updated! Press Ctrl+C to exit.");
    
    // Keep the program running
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}