fn main() {
    println!("Hello, world!");
}
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyleBuilder},
    text::{Alignment, Text},
};
use sh1106::{prelude::*, Builder};
use display_interface_i2c::I2CInterface;

// This is the I2C driver from our new, modern HAL
use linux_embedded_hal::{I2cdev, Delay};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1) Open a connection to the I²C bus
    //    On a Raspberry Pi, I²C-1 is the default bus on the GPIO header.
    let i2c = I2cdev::new("/dev/i2c-1")?;

    // 2) Create a display interface, providing the I2C bus and the display's address.
    //    The address for most SH1106 boards is 0x3C.
    let interface = I2CInterface::new(i2c, 0x3C, ());

    // 3) Build the SH1106 driver
    let mut display: GraphicsMode<_> = Builder::new()
        .with_size(DisplaySize::Display128x64)
        .connect(interface)
        .into();

    // 4) Initialize the display
    display.init()?;
    display.clear(BinaryColor::Off)?;

    // 5) Create styles and draw shapes
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    let stroke_style = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(1)
        .build();

    // Draw a frame and some text
    let bounds = display.bounding_box();
    bounds.into_styled(stroke_style).draw(&mut display)?;

    Text::with_alignment(
        "Hello Rust!",
        bounds.center() - Point::new(0, 8),
        text_style,
        Alignment::Center,
    ).draw(&mut display)?;

    // Draw a circle
    Circle::new(Point::new(104, 24), 20)
        .into_styled(stroke_style)
        .draw(&mut display)?;

    // 6) Flush the buffer to the screen
    display.flush()?;

    // Use a simple delay from the HAL
    let mut delay = Delay;
    embedded_hal::delay::DelayNs::delay_ms(&mut delay, 2000);

    Ok(())
}