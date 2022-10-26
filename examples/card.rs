use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{Point, Size},
    primitives::{Line, PrimitiveStyle, StyledDrawable},
    Drawable,
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};
use widget::card::{Card, CardStyleBuilder};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(400, 300));

    Line::new(Point::zero(), Point::new(399, 299)).draw_styled(
        &PrimitiveStyle::with_stroke(BinaryColor::On, 1),
        &mut display,
    )?;

    Line::new(Point::new(399, 0), Point::new(0, 299)).draw_styled(
        &PrimitiveStyle::with_stroke(BinaryColor::On, 1),
        &mut display,
    )?;

    let card_size = Size::new(132, 150);
    // let card_style = CardStyleBuilder::medium().build();

    Card::new(
        BinaryColor::On,
        BinaryColor::Off,
        Point::new(0, 150),
        card_size,
        "Kuchnia",
        CardStyleBuilder::big().build(),
    )
    .draw(&mut display)?;

    Card::new(
        BinaryColor::On,
        BinaryColor::Off,
        Point::new(134, 150),
        card_size,
        "Sypialnia",
        CardStyleBuilder::medium().build(),
    )
    .draw(&mut display)?;

    Card::new(
        BinaryColor::On,
        BinaryColor::Off,
        Point::new(268, 150),
        card_size,
        "Balkon",
        CardStyleBuilder::small().build(),
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
