#![cfg(target_os = "linux")]

use crate::modules::state::*;

use embedded_graphics::{
    image::Image,
    mono_font::{ascii::FONT_4X6, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point},
    primitives::{PrimitiveStyle, Rectangle},
    text::{Text, TextStyleBuilder},
    Drawable,
};
use tinybmp::Bmp;

use sh1106::{prelude::*, Builder};
use linux_embedded_hal::I2cdev;

use triple_buffer::*;

use std::thread;
use std::time::Duration;

const RANGE12BASE: &'static [u8] = include_bytes!("../../assets/range-12-base.bmp");

pub fn start_display(mut display_reader: Output<PlaceholderState>) {
    let mut i2c = I2cdev::new("/dev/i2c-1").unwrap();
    i2c.set_slave_address(0x3C).unwrap();

    let mut display: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();

    display.init().unwrap();
    display.flush().unwrap();

    loop {
        let state = display_reader.read();

        display.clear();

        draw_range(&mut display, Point::new(19, 13), state.values[0] as f32, "CV1");
        draw_range(&mut display, Point::new(19 + 27, 13), state.values[1] as f32, "CV2");
        draw_range(&mut display, Point::new(19 + 27 * 2, 13), state.values[2] as f32, "CV3");
        draw_range(&mut display, Point::new(19 + 27 * 3, 13), state.values[3] as f32, "CV4");
    
        display.flush().unwrap();

        thread::sleep(Duration::from_millis(40));
    }

}

pub fn draw_range(display: &mut GraphicsMode<I2cInterface<I2cdev>>, position: Point, val: f32, label: &str) {
    const BAR_TOP_LEFT: Point = Point::new(9, 9);
    const BAR_BOTTOM_RIGHT: Point = Point::new(13, 41);
    const LABEL_POS: Point = Point::new(11, 0);
    const VALUE_POS: Point = Point::new(11, 46);

    let character_style = MonoTextStyle::new(&FONT_4X6, BinaryColor::On);
    let text_style = TextStyleBuilder::new()
        .baseline(embedded_graphics::text::Baseline::Top)
        .alignment(embedded_graphics::text::Alignment::Center)
        .build();
    let fill: PrimitiveStyle<BinaryColor> = PrimitiveStyle::with_fill(BinaryColor::On);

    // Label
    Text::with_text_style(label, position + LABEL_POS, character_style, text_style)
        .draw(display).unwrap();

    // Base
    let base = Bmp::from_slice(RANGE12BASE).unwrap();
    Image::new(&base, Point::new(position.x, position.y + 7)).draw(display).unwrap();

    // Bar
    let bar_y_max = position.y + BAR_TOP_LEFT.y;
    let bar_y_min = position.y + BAR_BOTTOM_RIGHT.y;

    let top = lerp(bar_y_min as f32, bar_y_max as f32, val.clamp(0.0, 1.0)) as i32;
    let bar_top_left = position + BAR_TOP_LEFT;
    let bar_bottom_right = position + BAR_BOTTOM_RIGHT;

    Rectangle::with_corners(Point::new(bar_top_left.x, top), bar_bottom_right)
        .into_styled(fill)
        .draw(display).unwrap();

    // Value
    let text = format!("{:.2}", val);
    Text::with_text_style(&text, position + VALUE_POS, character_style, text_style)
        .draw(display).unwrap();
}

fn lerp(min: f32, max: f32, val: f32) -> f32 {
    min * (1.0 - val) + max * val
}
