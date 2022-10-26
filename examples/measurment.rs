use std::convert::Infallible;

use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{Point, Size},
    Drawable,
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};
use widget::measurment::{Measurement, MeasurementStyle};

fn main() -> Result<(), Infallible> {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(400, 300));

    Measurement::new(
        BinaryColor::On,
        Point::new(50, 100),
        -24.3,
        "°C",
        MeasurementStyle::default(),
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
