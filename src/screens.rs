use crate::state::*;

use embedded_graphics::{
    image::{Image, ImageDrawableExt},
    mono_font::{ascii::{FONT_4X6, FONT_5X7}, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point, Primitive, Size},
    primitives::{PrimitiveStyle, Rectangle, Line},
    text::{Text, TextStyleBuilder},
    Drawable,
};
use tinybmp::Bmp;

use std::fmt::Debug;

const RANGE12BASE: &'static [u8] = include_bytes!("../assets/bitmaps/range-12-base.bmp");
const TRIGGER01: &'static [u8] = include_bytes!("../assets/bitmaps/trigger-01.bmp");
const SHADERFRAME: &'static [u8] = include_bytes!("../assets/bitmaps/shader-frame-001.bmp");
const GRAPHIC001: &'static [u8] = include_bytes!("../assets/bitmaps/graphic-001.bmp");

pub fn draw<T>(display: &mut T, state: &State)
where
    T: DrawTarget<Color = BinaryColor>,
    T::Error: Debug,
{
    draw_home(display, state);
}

fn draw_home<T>(display: &mut T, state: &State)
where
    T: DrawTarget<Color = BinaryColor>,
    T::Error: Debug,
{
    draw_trigger(display, Point::new(24, 1), state.values[4] as f32);
    draw_trigger(display, Point::new(24 + 27, 1), state.values[5] as f32);
    draw_trigger(display, Point::new(24 + 27 * 2, 1), state.values[6] as f32);
    draw_trigger(display, Point::new(24 + 27 * 3, 1), state.values[7] as f32);

    draw_rounded(display);

    draw_bar(display, Point::new(24, 13), state.values[0] as f32, "CV1");
    draw_bar(display, Point::new(24 + 27, 13), state.values[1] as f32, "CV2");
    draw_bar(display, Point::new(24 + 27 * 2, 13), state.values[2] as f32, "CV3");
    draw_bar(display, Point::new(24 + 27 * 3, 13), state.values[3] as f32, "CV4");

    draw_shader_frame(display, Point::new(0, 0), state.shader_index + 1);

    draw_graphic_sprite(display, Point::new(1, 10), state.values[0] as f32);
}

pub fn draw_debug<T>(display: &mut T, state: &State)
where
    T: DrawTarget<Color = BinaryColor>,
    T::Error: Debug,
{
    let character_style = MonoTextStyle::new(&FONT_4X6, BinaryColor::On);
    let text_style = TextStyleBuilder::new()
        .baseline(embedded_graphics::text::Baseline::Top)
        .alignment(embedded_graphics::text::Alignment::Left)
        .build();

    let fps = format!("FPS: {:.2}", state.fps);
    Text::with_text_style(&fps, Point::new(32, 2), character_style, text_style)
        .draw(display).unwrap();

    draw_graphic_sprite(display, Point::new(1, 10), state.values[0] as f32);
}

fn draw_graphic_sprite<T>(display: &mut T, position: Point, val: f32)
where
    T: DrawTarget<Color = BinaryColor>,
    T::Error: Debug,
{
    let num_items = 4;
    let index = (val * num_items as f32).floor() as usize;

    let width: i32 = 18;
    let height: i32 = 53;
    let x_offset = index as i32 * width;

    let area = Rectangle::new(Point::new(x_offset, 0), Size::new(width as u32, height as u32));
    let spritesheet_bmp = Bmp::from_slice(GRAPHIC001).unwrap();
    let image = spritesheet_bmp.sub_image(&area);
    Image::new(&image, position).draw(display).unwrap();
}

fn draw_trigger<T>(display: &mut T, position: Point, val: f32)
where
    T: DrawTarget<Color = BinaryColor>,
    T::Error: Debug,
{
    let index = if val > 0.5 {
        1
    } else {
        0
    };

    let width: i32 = 23;
    let height: i32 = 8;
    let x_offset = index as i32 * width;

    let area = Rectangle::new(Point::new(x_offset, 0), Size::new(width as u32, height as u32));
    let spritesheet_bmp = Bmp::from_slice(TRIGGER01).unwrap();
    let image = spritesheet_bmp.sub_image(&area);
    Image::new(&image, position).draw(display).unwrap();
}

fn draw_shader_frame<T>(display: &mut T, position: Point, index: usize)
where
    T: DrawTarget<Color = BinaryColor>,
    T::Error: Debug,
{
    let character_style = MonoTextStyle::new(&FONT_5X7, BinaryColor::Off);
    let text_style = TextStyleBuilder::new()
        .baseline(embedded_graphics::text::Baseline::Top)
        .alignment(embedded_graphics::text::Alignment::Center)
        .build();


    let image = Bmp::from_slice(SHADERFRAME).unwrap();
    Image::new(&image, position).draw(display).unwrap();

    let text = format!("S0{}", index);
    Text::with_text_style(&text, position + Point::new(10, 2), character_style, text_style)
        .draw(display).unwrap();
}

fn draw_rounded<T>(display: &mut T)
where
    T: DrawTarget<Color = BinaryColor>,
    T::Error: Debug,
{
    Line::new(Point::new(24, 0), Point::new(126, 0))
    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
    .draw(display).unwrap();

    Line::new(Point::new(127, 1), Point::new(127, 8))
    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
    .draw(display).unwrap();

    Line::new(Point::new(126, 9), Point::new(24, 9))
    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
    .draw(display).unwrap();

    Line::new(Point::new(23, 8), Point::new(23, 1))
    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
    .draw(display).unwrap();
}


fn draw_bar<T>(display: &mut T, position: Point, val: f32, label: &str)
where
    T: DrawTarget<Color = BinaryColor>,
    T::Error: Debug,
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
