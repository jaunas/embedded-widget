use std::convert::Infallible;

use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{Point, Size},
    Drawable,
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};
use widget::{
    card::{Card, CardStyleBuilder},
    measurment::{Measurement, MeasurementStyle, Precision},
};

fn main() -> Result<(), Infallible> {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(400, 300));

    Card::new(
        BinaryColor::On,
        BinaryColor::Off,
        Point::new(0, 150),
        Size::new(132, 150),
        "Garage",
        CardStyleBuilder::big().build(),
    )
    .draw(&mut display)?;

    Measurement::new(
        BinaryColor::On,
        Point::new(22, 210),
        -10.3,
        "°C",
        MeasurementStyle::default(),
    )
    .draw(&mut display)?;

    Measurement::new(
        BinaryColor::On,
        Point::new(50, 270),
        62.,
        "%",
        MeasurementStyle::with_precision(Precision::None),
    )
    .draw(&mut display)?;

    Window::new(
        "Demo",
        &OutputSettingsBuilder::new()
            .theme(BinaryColorTheme::Inverted)
            .scale(2)
            .pixel_spacing(0)
            .build(),
    )
    .show_static(&display);

    Ok(())
}
