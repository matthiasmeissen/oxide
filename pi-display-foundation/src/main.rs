
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

use sh1106::{prelude::*, Builder};
use linux_embedded_hal::I2cdev;

use std::thread;
use std::time::Duration;

fn main() {
    let mut i2c = I2cdev::new("/dev/i2c-1").unwrap();
    i2c.set_slave_address(0x3C).unwrap();

    start_display(i2c);
}

fn start_display(i2c_address: I2cdev) {
    let mut display: GraphicsMode<_> = Builder::new().connect_i2c(i2c_address).into();

    display.init().unwrap();
    display.flush().unwrap();

    let mut num = 0.0;

    loop {
        display.clear();

        draw_text(&mut display, num);

        num += 0.1;
    
        display.flush().unwrap();

        thread::sleep(Duration::from_millis(100));
    }

}

fn draw_text(display: &mut GraphicsMode<I2cInterface<I2cdev>>, num: f64) {
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    let text = format!("{num}");
    Text::with_baseline(&text, Point::new(0, 16), text_style, Baseline::Top)
        .draw(display)
        .unwrap();
}
