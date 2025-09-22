
use embedded_graphics::{pixelcolor::{raw::RawU32, BinaryColor}, prelude::{Dimensions, Point, Primitive, Size}, primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, Triangle}, *};
use embedded_graphics_simulator::*;

// To run the simulator run this command in the terminal
// export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
// This will enable it for that session only

fn main() -> Result<(), std::convert::Infallible> {
    println!("Embedded Foundation");

    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));

    let (xoffset, yoffset, unit) = (16, 16, 16);

    let stroke1 = PrimitiveStyle::with_stroke(BinaryColor::On, 1);

    let stroke2 = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(2)
        .stroke_alignment(primitives::StrokeAlignment::Inside)
        .build();

    let fill = PrimitiveStyle::with_fill(BinaryColor::On);

    display.bounding_box().into_styled(stroke2).draw(&mut display)?;

    Triangle::new(
        Point::new(xoffset, yoffset), 
        Point::new(xoffset, yoffset + unit), 
        Point::new(xoffset + unit, yoffset + unit)
    ).into_styled(stroke1).draw(&mut display)?;

    Rectangle::new(Point::new(xoffset + 2 * unit, yoffset), Size::new(unit as u32, unit as u32))
        .into_styled(fill)
        .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledWhite)
        .build();

    Window::new("Window Title", &output_settings).show_static(&display);
    Ok(())
}
