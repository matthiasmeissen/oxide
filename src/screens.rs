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

// To convert the bmp file
// Run: ffmpeg -i source.bmp -pix_fmt bgr24 target.bmp

const TRIGGER01: &'static [u8] = include_bytes!("../assets/bitmaps/trigger-01.bmp");
const HOME002: &'static [u8] = include_bytes!("../assets/bitmaps/home-002.bmp");
const HOME002RANGE1: &'static [u8] = include_bytes!("../assets/bitmaps/home-002-range-1.bmp");
const HOME002RANGE2: &'static [u8] = include_bytes!("../assets/bitmaps/home-002-range-2.bmp");
const HOME002RANGE3: &'static [u8] = include_bytes!("../assets/bitmaps/home-002-range-3.bmp");
const HOME002RANGE4: &'static [u8] = include_bytes!("../assets/bitmaps/home-002-range-4.bmp");

const SHADER_NAMES: &[&str] = &["Shader 1", "Shader 2", "Shader 3"];
const DSP_NAMES: &[&str] = &["Simple Sine", "Basic FM", "Drum Engine"];

pub fn draw<T>(display: &mut T, state: &DisplayState)
where
    T: DrawTarget<Color = BinaryColor>,
    T::Error: Debug,
{
    match state.ui {
        Screen::Home => screen_home(display, &state.app),
        Screen::HomeSettings {selected_index} => screen_home_settings(display, &state.app, selected_index),
        Screen::Shader => screen_shader(display, &state.app),
        Screen::ShaderSelect {selected_index} => screen_shader_select(display, &state.app, selected_index),
        Screen::Audio => screen_audio(display, &state.app),
        Screen::AudioSelect {selected_index} => screen_audio_select(display, &state.app, selected_index),
    }
}

// -------- Screens --------

fn screen_home<T>(display: &mut T, state: &State)
where
    T: DrawTarget<Color = BinaryColor>,
    T::Error: Debug,
{
    comp_trigger(display, Point::new(24, 1), state.values[4] as f32);
    comp_trigger(display, Point::new(24 + 27, 1), state.values[5] as f32);
    comp_trigger(display, Point::new(24 + 27 * 2, 1), state.values[6] as f32);
    comp_trigger(display, Point::new(24 + 27 * 3, 1), state.values[7] as f32);

    comp_value(display, Point::new(0, 8), state.values[0], 0);
    comp_value(display, Point::new(64, 8), state.values[1], 1);
    comp_value(display, Point::new(0, 36), state.values[2], 2);
    comp_value(display, Point::new(64, 36), state.values[3], 3);
}

fn screen_home_settings<T>(display: &mut T, state: &State, index: usize)
where
    T: DrawTarget<Color = BinaryColor>,
    T::Error: Debug,
{
    // let fps = format!("FPS: {:.2}", state.fps);
    // comp_text(display, Point { x: 20, y: 0 }, &fps);
    // let i = format!("Index: {}", index);
    // comp_text(display, Point { x: 20, y: 20 }, &i);

    comp_image(display, HOME002);
}

fn screen_shader<T>(display: &mut T, state: &State)
where
    T: DrawTarget<Color = BinaryColor>,
    T::Error: Debug,
{
    comp_text(display, Point { x: 20, y: 0 }, "Shader: Press to select");
}

fn screen_shader_select<T>(display: &mut T, state: &State, index: usize)
where
    T: DrawTarget<Color = BinaryColor>,
    T::Error: Debug,
{
    comp_select_list(display, index, SHADER_NAMES);
}

fn screen_audio<T>(display: &mut T, state: &State)
where
    T: DrawTarget<Color = BinaryColor>,
    T::Error: Debug,
{
    comp_text(display, Point { x: 20, y: 0 }, "Audio: Press to select");
}

fn screen_audio_select<T>(display: &mut T, state: &State, index: usize)
where
    T: DrawTarget<Color = BinaryColor>,
    T::Error: Debug,
{
    comp_select_list(display, index, DSP_NAMES);
}

// -------- Components --------

fn comp_text<T>(display: &mut T, position: Point, text: &str)
where
    T: DrawTarget<Color = BinaryColor>,
    T::Error: Debug,
{
    let character_style = MonoTextStyle::new(&FONT_4X6, BinaryColor::On);
    let text_style = TextStyleBuilder::new()
        .baseline(embedded_graphics::text::Baseline::Top)
        .alignment(embedded_graphics::text::Alignment::Left)
        .build();

    Text::with_text_style(&text, position, character_style, text_style)
        .draw(display).unwrap();
}

fn comp_select_list<T>(display: &mut T, index: usize, list: &[&str])
where
    T: DrawTarget<Color = BinaryColor>,
    T::Error: Debug,
{
    for (i, item) in list.iter().enumerate() {
        let offset_y = i as i32 * 8;
        if i == index {
            comp_text(display, Point { x: 0, y: offset_y }, ">");
        }
        comp_text(display, Point { x: 20, y: offset_y }, item);
    }
}

fn comp_value<T>(display: &mut T, position: Point, val: f32, index: usize)
where
    T: DrawTarget<Color = BinaryColor>,
    T::Error: Debug,
{
    match index {
        0 => {
            comp_spritesheet(display, position, 1.0 - val, 32, 64, 28, HOME002RANGE1);
            let text = format!("{:.1}", val);
            comp_text(display, position + Point::new(1, 22), &text);
        }
        1 => {
            comp_spritesheet(display, position, val, 32, 64, 28, HOME002RANGE2);
            let text = format!("{:.1}", val);
            comp_text(display, position + Point::new(51, 22), &text);
        }
        2 => {
            comp_spritesheet(display, position, 1.0 - val, 32, 64, 28, HOME002RANGE3);
            let text = format!("{:.1}", val);
            comp_text(display, position + Point::new(1, 1), &text);
        }
        3 => {
            comp_spritesheet(display, position, val, 32, 64, 28, HOME002RANGE4);
            let text = format!("{:.1}", val);
            comp_text(display, position + Point::new(51, 1), &text);
        }
        _ => ()
    }
}

fn comp_spritesheet<T>(display: &mut T, position: Point, val: f32, items: i32, width: i32, height: i32, bytes: &'static [u8])
where
    T: DrawTarget<Color = BinaryColor>,
    T::Error: Debug,
{
    let index = (val * items as f32).floor() as usize;
    let x_offset = index as i32 * width;

    let area = Rectangle::new(Point::new(x_offset, 0), Size::new(width as u32, height as u32));
    let spritesheet_bmp = Bmp::from_slice(bytes).unwrap();
    let image = spritesheet_bmp.sub_image(&area);
    Image::new(&image, position).draw(display).unwrap();
}

fn comp_image<T>(display: &mut T, bytes: &'static [u8])
where
    T: DrawTarget<Color = BinaryColor>,
    T::Error: Debug,
{
    let base = Bmp::from_slice(bytes).unwrap();
    Image::new(&base, Point::new(0, 0)).draw(display).unwrap();
}

fn comp_trigger<T>(display: &mut T, position: Point, val: f32)
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

fn lerp(min: f32, max: f32, val: f32) -> f32 {
    min * (1.0 - val) + max * val
}
