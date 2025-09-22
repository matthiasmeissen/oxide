
use embedded_graphics::{pixelcolor::BinaryColor, prelude::{Dimensions, Point, Primitive, Size}, primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Triangle}, *};
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

    display.bounding_box().into_styled(stroke2).draw(&mut display)?;

    Triangle::new(
        Point::new(xoffset, yoffset), 
        Point::new(xoffset, yoffset + unit), 
        Point::new(xoffset + unit, yoffset + unit)
    ).into_styled(stroke1).draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledWhite)
        .build();

    Window::new("Window Title", &output_settings).show_static(&display);
    Ok(())
}
