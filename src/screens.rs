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

const HOME002: &'static [u8] = include_bytes!("../assets/bitmaps/home-002/home-002.bmp");
const TOP: &'static [u8] = include_bytes!("../assets/bitmaps/home-002/home-002-top.bmp");
const TRIGGER: &'static [u8] = include_bytes!("../assets/bitmaps/home-002/home-002-trigger.bmp");
const RANGE1: &'static [u8] = include_bytes!("../assets/bitmaps/home-002/home-002-range-1.bmp");
const RANGE2: &'static [u8] = include_bytes!("../assets/bitmaps/home-002/home-002-range-2.bmp");
const RANGE3: &'static [u8] = include_bytes!("../assets/bitmaps/home-002/home-002-range-3.bmp");
const RANGE4: &'static [u8] = include_bytes!("../assets/bitmaps/home-002/home-002-range-4.bmp");

const SHADERSELECT: &'static [u8] = include_bytes!("../assets/bitmaps/shader-001/shader-001-select.bmp");
const SHADER01: &'static [u8] = include_bytes!("../assets/bitmaps/shader-001/shader-001-01.bmp");
const SHADER02: &'static [u8] = include_bytes!("../assets/bitmaps/shader-001/shader-001-02.bmp");
const SHADER03: &'static [u8] = include_bytes!("../assets/bitmaps/shader-001/shader-001-03.bmp");

const AUDIO: &'static [u8] = include_bytes!("../assets/bitmaps/audio-001/audio-001.bmp");

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
    comp_image(display,Point::new(0, 0), TOP);
    let fps = format!("{:.0}", state.fps);
    comp_dark_text(display, Point::new(38, 1), &fps);
    let shader = format!("{}", state.shader_index);
    comp_dark_text(display, Point::new(55, 1), &shader);
    let dsp = format!("{}", state.dsp_type);
    comp_dark_text(display, Point::new(68, 1), &dsp);

    comp_value(display, Point::new(0, 8), state.values[0], 0);
    comp_value(display, Point::new(64, 8), state.values[1], 1);
    comp_value(display, Point::new(0, 36), state.values[2], 2);
    comp_value(display, Point::new(64, 36), state.values[3], 3);
    
    comp_trigger(display, Point::new(50, 9), state.values[4]);
    comp_trigger(display, Point::new(67, 9), state.values[5]);
    comp_trigger(display, Point::new(50, 59), state.values[6]);
    comp_trigger(display, Point::new(67, 59), state.values[7]);
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

    comp_image(display, Point::new(0, 0), HOME002);
}

fn screen_shader<T>(display: &mut T, state: &State)
where
    T: DrawTarget<Color = BinaryColor>,
    T::Error: Debug,
{
    comp_select_shader_image(display, state.shader_index);
    let num = format!("{}", state.shader_index);
    comp_dark_text(display, Point { x: 117, y: 4 }, &num);
    comp_image(display, Point::new(34, 51), SHADERSELECT);
}

fn screen_shader_select<T>(display: &mut T, state: &State, index: usize)
where
    T: DrawTarget<Color = BinaryColor>,
    T::Error: Debug,
{
    comp_select_shader_image(display, index);
    let num = format!("{}", index);
    comp_dark_text(display, Point { x: 117, y: 4 }, &num);
}

fn screen_audio<T>(display: &mut T, state: &State)
where
    T: DrawTarget<Color = BinaryColor>,
    T::Error: Debug,
{
    comp_spritesheet(display, Point::new(0, 0), SpritesheetIndex::Index(state.dsp_type.get_index()), 3, 128, 64, AUDIO);
    comp_image(display, Point::new(34, 51), SHADERSELECT);
}

fn screen_audio_select<T>(display: &mut T, state: &State, index: usize)
where
    T: DrawTarget<Color = BinaryColor>,
    T::Error: Debug,
{
    comp_spritesheet(display, Point::new(0, 0), SpritesheetIndex::Index(index), 3, 128, 64, AUDIO);
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

fn comp_dark_text<T>(display: &mut T, position: Point, text: &str)
where
    T: DrawTarget<Color = BinaryColor>,
    T::Error: Debug,
{
    let character_style = MonoTextStyle::new(&FONT_4X6, BinaryColor::Off);
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
        let offset_y = (i as i32 * 8) + 2;
        if i == index {
            comp_text(display, Point { x: 2, y: offset_y }, ">");
        }
        comp_text(display, Point { x: 12, y: offset_y }, item);
    }
}

fn comp_select_shader_image<T>(display: &mut T, index: usize)
where
    T: DrawTarget<Color = BinaryColor>,
    T::Error: Debug,
{
    match index {
        0 => comp_image(display, Point::new(0, 0), SHADER01),
        1 => comp_image(display, Point::new(0, 0), SHADER02),
        2 => comp_image(display, Point::new(0, 0), SHADER03),
        _ => comp_image(display, Point::new(0, 0), SHADER01),
    }
}

fn comp_value<T>(display: &mut T, position: Point, val: f32, index: usize)
where
    T: DrawTarget<Color = BinaryColor>,
    T::Error: Debug,
{
    let text = format!("{:.1}", val);
    match index {
        0 => {
            comp_spritesheet(display, position, SpritesheetIndex::Normalized(1.0 - val), 32, 64, 28, RANGE1);
            comp_text(display, position + Point::new(1, 22), &text);
        }
        1 => {
            comp_spritesheet(display, position, SpritesheetIndex::Normalized(val), 32, 64, 28, RANGE2);
            comp_text(display, position + Point::new(51, 22), &text);
        }
        2 => {
            comp_spritesheet(display, position, SpritesheetIndex::Normalized(1.0 - val), 32, 64, 28, RANGE3);
            comp_text(display, position + Point::new(1, 1), &text);
        }
        3 => {
            comp_spritesheet(display, position, SpritesheetIndex::Normalized(val), 32, 64, 28, RANGE4);
            comp_text(display, position + Point::new(51, 1), &text);
        }
        _ => ()
    }
}

enum SpritesheetIndex {
    Normalized(f32),
    Index(usize)
}

fn comp_spritesheet<T>(display: &mut T, position: Point, val: SpritesheetIndex, items: i32, width: i32, height: i32, bytes: &'static [u8])
where
    T: DrawTarget<Color = BinaryColor>,
    T::Error: Debug,
{
    let mut index = match val {
        SpritesheetIndex::Normalized(v) => {
            let clamped_val = v.max(0.0).min(1.0);
            (clamped_val * (items - 1) as f32).floor() as usize
        },
        SpritesheetIndex::Index(i) => i,
    };

    index = index % items as usize;

    let x_offset = index as i32 * width;

    let area = Rectangle::new(Point::new(x_offset, 0), Size::new(width as u32, height as u32));
    let spritesheet_bmp = Bmp::from_slice(bytes).unwrap();
    let image = spritesheet_bmp.sub_image(&area);
    Image::new(&image, position).draw(display).unwrap();
}

fn comp_image<T>(display: &mut T, position: Point, bytes: &'static [u8])
where
    T: DrawTarget<Color = BinaryColor>,
    T::Error: Debug,
{
    let base = Bmp::from_slice(bytes).unwrap();
    Image::new(&base, position).draw(display).unwrap();
}

fn comp_trigger<T>(display: &mut T, position: Point, val: f32)
where
    T: DrawTarget<Color = BinaryColor>,
    T::Error: Debug,
{
    let index = if val > 0.5 { 1 } else { 0 };

    comp_spritesheet(display, position, SpritesheetIndex::Index(index), 4, 11, 3, TRIGGER);
}

fn lerp(min: f32, max: f32, val: f32) -> f32 {
    min * (1.0 - val) + max * val
}
