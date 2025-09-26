
use crate::modules::state::*;

use embedded_graphics::image::ImageDrawableExt;
use embedded_graphics::prelude::{Primitive, Size};
use embedded_graphics::text::LineHeight;
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

// Convert Spritesheets to correct format
// ffmpeg -i sheet-test.bmp -pix_fmt bgr24 sheet-test-01.bmp  
// ffmpeg -i sheet-test.bmp -frames:v 1 -pix_fmt bgr24 sheet-test-01.bmp
// Somehow this is needed

const RANGE12: &'static [u8] = include_bytes!("../../assets/range-12/range-12.bmp");
const RANGE12BASE: &'static [u8] = include_bytes!("../../assets/range-12/range-12-base.bmp");
const SCREEN001: &'static [u8] = include_bytes!("../../assets/screen-001/screen-001.bmp");
const SHEETTEST: &'static [u8] = include_bytes!("../../assets/screen-001/sheet-test.bmp");


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

pub fn draw_preview_screen<T>(display: &mut T, state: &PlaceholderState) -> Result<(), T::Error>
where
    T: DrawTarget<Color = BinaryColor>,
{
    let image = Bmp::from_slice(SHEETTEST).unwrap();
    Image::new(&image, Point::new(0, 0)).draw(display)?;

    Ok(())
}

pub fn draw_sprite<T>(display: &mut T, state: &PlaceholderState, num: f64) -> Result<(), T::Error>
where
    T: DrawTarget<Color = BinaryColor>,
{

    let index = (num % 4.0) as usize;
    draw_sheet(display, Point::new(0, 0), index)?;

    Ok(())
}

fn draw_sheet<T>(display: &mut T, position: Point, index: usize) -> Result<(), T::Error>
where
    T: DrawTarget<Color = BinaryColor>,
{
    let width: i32 = 18;
    let height: i32 = 53;
    let x_offset = index as i32 * width;

    let area = Rectangle::new(Point::new(x_offset, 0), Size::new(width as u32, height as u32));

    let spritesheet_bmp = Bmp::from_slice(SHEETTEST).unwrap();

    let image = spritesheet_bmp.sub_image(&area);

    Image::new(&image, position).draw(display)?;

    Ok(())
}

fn draw_bar<T>(display: &mut T, position: Point, val: f32, label: &str) -> Result<(), T::Error>
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