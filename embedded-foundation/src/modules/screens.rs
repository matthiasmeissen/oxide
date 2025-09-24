
use crate::modules::state::*;
use crate::modules::bmpdata::*;

use embedded_graphics::{
    image::Image, mono_font::{ascii::FONT_4X6, MonoTextStyle}, pixelcolor::BinaryColor, prelude::{Dimensions, Point, Primitive, Size, DrawTarget}, primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, Triangle}, text::{Text, TextStyleBuilder}, *
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

    let button_data = include_bytes!("../../assets/button/button-default.bmp");
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

    let knob_data = include_bytes!("../../assets/elektron-test/elektron-test-knob.bmp");
    let knob = Bmp::from_slice(knob_data).unwrap();
    Image::new(&knob, Point::new(xoff, yoff)).draw(display)?;

    let wave_data = include_bytes!("../../assets/elektron-test/elektron-test-wave.bmp");
    let wave = Bmp::from_slice(wave_data).unwrap();
    Image::new(&wave, Point::new(xoff + base, yoff)).draw(display)?;

    Ok(())
}

pub fn draw_elektron_template(display: &mut SimulatorDisplay<BinaryColor>, state: &PlaceholderState) -> Result<(), std::convert::Infallible> {
    let image_data = include_bytes!("../../assets/elektron-test/elektron-template-001.bmp");
    let image = Bmp::from_slice(image_data).unwrap();
    Image::new(&image, Point::new(0, 0)).draw(display)?;
    Ok(())
}

pub fn draw_screen(display: &mut SimulatorDisplay<BinaryColor>, state: &PlaceholderState) -> Result<(), std::convert::Infallible> {
    draw_bar(display, Point::new(19, 13), state.values[0] as f32, "CV1")?;

    draw_bar(display, Point::new(19 + (23 + 4), 13), state.values[1] as f32, "CV2")?;

    draw_bar(display, Point::new(19 + (23 + 4) * 2, 13), state.values[2] as f32, "CV3")?;

    draw_bar(display, Point::new(19 + (23 + 4) * 3, 13), state.values[3] as f32, "CV3")?;

    Ok(())
}

pub fn draw_bar<T>(display: &mut T, position: Point, val: f32, label: &str ) -> Result<(), T::Error> where
    T: DrawTarget<Color = BinaryColor>,
{
    const BAR_TOP_LEFT: Point = Point::new(9, 9);
    const BAR_BOTTOM_RIGHT: Point = Point::new(13, 41);
    const LABEL_POS: Point = Point::new(11, 0);
    const VALUE_POS: Point = Point::new(11, 46);

    let character_style = MonoTextStyle::new(&FONT_4X6, BinaryColor::On);
    let text_style = TextStyleBuilder::new()
        .baseline(text::Baseline::Top)
        .alignment(text::Alignment::Center)
        .build();
    let fill = PrimitiveStyle::with_fill(BinaryColor::On);

    // Label
    Text::with_text_style(
        label,
        position + LABEL_POS,
        character_style,
        text_style,
    )
    .draw(display)?;

    // Base
    let base = Bmp::from_slice(RANGE12BASE).unwrap();
    Image::new(&base, Point::new(position.x, position.y +7)).draw(display)?;

    // Bar
    let bar_y_max = position.y + BAR_TOP_LEFT.y;
    let bar_y_min = position.y + BAR_BOTTOM_RIGHT.y;

    let top = lerp(bar_y_min as f32, bar_y_max as f32, val.clamp(0.0, 1.0)) as i32;
    let bar_top_left = position + BAR_TOP_LEFT;
    let bar_bottom_right = position + BAR_BOTTOM_RIGHT;

    Rectangle::with_corners(
        Point::new(bar_top_left.x, top),
        bar_bottom_right,
    )
    .into_styled(fill)
    .draw(display)?;

    // Value
    let text = format!("{:.2}", val);
    Text::with_text_style(
        &text,
        position + VALUE_POS,
        character_style,
        text_style,
    )
    .draw(display)?;

    Ok(())
}


fn lerp(min: f32, max: f32, val: f32) -> f32 {
    min * (1.0 - val) + max * val
}
