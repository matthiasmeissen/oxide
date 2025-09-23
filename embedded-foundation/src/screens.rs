
use crate::state::*;

use embedded_graphics::{
    image::Image, mono_font::{ascii::FONT_4X6, MonoTextStyle}, pixelcolor::BinaryColor, prelude::{Dimensions, Point, Primitive, Size}, primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, Triangle}, text::{Text, TextStyleBuilder}, *
};
use embedded_graphics_simulator::*;
use tinybmp::*;

pub fn draw_basic_screen(display: &mut SimulatorDisplay<BinaryColor>, state: &PlaceholderState) -> Result<(), std::convert::Infallible> {
    let (xoff, yoff, base) = (16, 16, 16);

    // Styles

    let stroke1 = PrimitiveStyle::with_stroke(BinaryColor::On, 1);

    let stroke2 = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(2)
        .stroke_alignment(primitives::StrokeAlignment::Inside)
        .build();

    let fill = PrimitiveStyle::with_fill(BinaryColor::On);

    let character_style = MonoTextStyle::new(&FONT_4X6, BinaryColor::On);
    let text_style = TextStyleBuilder::new()
        .baseline(text::Baseline::Top)
        .alignment(text::Alignment::Left)
        .build();

    
    // Shapes
    
    display.bounding_box().into_styled(stroke2).draw(display)?;

    Triangle::new(
        Point::new(xoff, yoff), 
        Point::new(xoff, yoff + base), 
        Point::new(xoff + base, yoff + base)
    ).into_styled(stroke1).draw(display)?;

    Rectangle::new(Point::new(xoff + 2 * base - 2, yoff), Size::new(base as u32, base as u32))
        .into_styled(fill)
        .draw(display)?;

    let button_data = include_bytes!("../assets/button/button-default.bmp");
    let button = Bmp::from_slice(button_data).unwrap();

    Image::new(&button, Point::new(xoff + 3 * base, yoff)).draw(display)?;

    Text::with_text_style(
        "Hello", 
        Point::new(xoff * 5 * base + 2, yoff), 
        character_style, 
        text_style)
        .draw(display)?;

    let text = format!("{:.2}", state.time);
        Text::with_text_style(
            &text,
            Point::new(xoff * 5 + 2, yoff),
            character_style,
            text_style,
        )
        .draw(display)?;

    Ok(())
}


pub fn draw_elektron_test(display: &mut SimulatorDisplay<BinaryColor>, state: &PlaceholderState) -> Result<(), std::convert::Infallible> {
    let (base, xoff, yoff) = (25, 16, 8);

    let knob_data = include_bytes!("../assets/elektron-test/elektron-test-knob.bmp");
    let knob = Bmp::from_slice(knob_data).unwrap();
    Image::new(&knob, Point::new(xoff, yoff)).draw(display)?;

    let wave_data = include_bytes!("../assets/elektron-test/elektron-test-wave.bmp");
    let wave = Bmp::from_slice(wave_data).unwrap();
    Image::new(&wave, Point::new(xoff + base, yoff)).draw(display)?;

    Ok(())
}

