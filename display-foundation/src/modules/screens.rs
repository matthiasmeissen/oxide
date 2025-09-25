// screens.rs

use crate::modules::bmpdata::*;
use crate::modules::state::*;

use embedded_graphics::prelude::Primitive;
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

pub fn draw_screen<T>(display: &mut T, state: &PlaceholderState) -> Result<(), T::Error>
where
    T: DrawTarget<Color = BinaryColor>,
{
    draw_bar(display, Point::new(19, 13), state.values[0] as f32, "CV1")?;
    draw_bar(display, Point::new(19 + 27, 13), state.values[1] as f32, "CV2")?;
    draw_bar(display, Point::new(19 + 27 * 2, 13), state.values[2] as f32, "CV3")?;
    draw_bar(display, Point::new(19 + 27 * 3, 13), state.values[3] as f32, "CV4")?;

    Ok(())
}

pub fn draw_bar<T>(display: &mut T, position: Point, val: f32, label: &str) -> Result<(), T::Error>
where
    T: DrawTarget<Color = BinaryColor>,
{
    const BAR_TOP_LEFT: Point = Point::new(9, 9);
    const BAR_BOTTOM_RIGHT: Point = Point::new(13, 41);
    const LABEL_POS: Point = Point::new(11, 0);
    const VALUE_POS: Point = Point::new(11, 46);

    let character_style = MonoTextStyle::new(&FONT_4X6, BinaryColor::On);
    let text_style = TextStyleBuilder::new()
        .baseline(embedded_graphics::text::Baseline::Top)
        .alignment(embedded_graphics::text::Alignment::Center)
        .build();
    let fill = PrimitiveStyle::with_fill(BinaryColor::On);

    // Label
    Text::with_text_style(label, position + LABEL_POS, character_style, text_style)
        .draw(display)?;

    // Base
    let base = Bmp::from_slice(RANGE12BASE).unwrap();
    Image::new(&base, Point::new(position.x, position.y + 7)).draw(display)?;

    // Bar
    let bar_y_max = position.y + BAR_TOP_LEFT.y;
    let bar_y_min = position.y + BAR_BOTTOM_RIGHT.y;

    let top = lerp(bar_y_min as f32, bar_y_max as f32, val.clamp(0.0, 1.0)) as i32;
    let bar_top_left = position + BAR_TOP_LEFT;
    let bar_bottom_right = position + BAR_BOTTOM_RIGHT;

    Rectangle::with_corners(Point::new(bar_top_left.x, top), bar_bottom_right)
        .into_styled(fill)
        .draw(display)?;

    // Value
    let text = format!("{:.2}", val);
    Text::with_text_style(&text, position + VALUE_POS, character_style, text_style)
        .draw(display)?;

    Ok(())
}

fn lerp(min: f32, max: f32, val: f32) -> f32 {
    min * (1.0 - val) + max * val
}