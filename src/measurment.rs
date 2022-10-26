use embedded_graphics::{
    prelude::{Dimensions, DrawTarget, PixelColor, Point},
    text::Text,
    Drawable,
};
use u8g2_fonts::{
    fonts::{
        u8g2_font_logisoso16_tf, u8g2_font_logisoso18_tf, u8g2_font_logisoso22_tf,
        u8g2_font_logisoso42_tf,
    },
    U8g2TextStyle,
};

pub struct Measurement<'a, C> {
    color: C,
    position: Point,
    value: f32,
    unit: &'a str,
    style: MeasurementStyle,
}

impl<'a, C> Measurement<'a, C> {
    pub fn new(
        color: C,
        position: Point,
        value: f32,
        unit: &'a str,
        style: MeasurementStyle,
    ) -> Self {
        Self {
            color,
            position,
            value,
            unit,
            style,
        }
    }
}

impl<'a, C> Drawable for Measurement<'a, C>
where
    C: PixelColor,
{
    type Color = C;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        match self.style.precision {
            Precision::None => {
                Text::new(
                    &format!("{}{}", self.value, self.unit),
                    self.position,
                    U8g2TextStyle::new(u8g2_font_logisoso42_tf, self.color),
                )
                .draw(target)?;
            }
            Precision::Single | Precision::Double => {
                let integer = &format!("{}", self.value.trunc());

                let integer_text = Text::new(
                    integer,
                    self.position,
                    U8g2TextStyle::new(u8g2_font_logisoso42_tf, self.color),
                );
                integer_text.draw(target)?;

                let unit_text = Text::new(
                    self.unit,
                    integer_text.bounding_box().bottom_right().unwrap() + Point::new(4, 0),
                    U8g2TextStyle::new(u8g2_font_logisoso18_tf, self.color),
                );
                unit_text.draw(target)?;

                match self.style.precision {
                    Precision::Single => {
                        let fract = &format!("{}", (self.value.abs().fract() * 10.).round());
                        Text::new(
                            fract,
                            unit_text.bounding_box().top_left + Point::new(2, 0),
                            U8g2TextStyle::new(u8g2_font_logisoso22_tf, self.color),
                        )
                        .draw(target)?;
                    }
                    Precision::Double => {
                        let fract = &format!("{:02}", (self.value.abs().fract() * 100.).round());
                        Text::new(
                            fract,
                            unit_text.bounding_box().top_left + Point::new(-2, -6),
                            U8g2TextStyle::new(u8g2_font_logisoso16_tf, self.color),
                        )
                        .draw(target)?;
                    }
                    _ => {}
                }
            }
        };

        Ok(())
    }
}

pub struct MeasurementStyle {
    precision: Precision,
}

impl MeasurementStyle {
    pub fn with_precision(precision: Precision) -> MeasurementStyle {
        MeasurementStyleBuilder::new().precision(precision).build()
    }
}

impl Default for MeasurementStyle {
    fn default() -> Self {
        MeasurementStyleBuilder::new().build()
    }
}

pub struct MeasurementStyleBuilder {
    style: MeasurementStyle,
}

impl MeasurementStyleBuilder {
    pub fn new() -> MeasurementStyleBuilder {
        Self {
            style: MeasurementStyle {
                precision: Precision::Single,
            },
        }
    }

    pub fn precision(mut self, precision: Precision) -> MeasurementStyleBuilder {
        self.style.precision = precision;
        self
    }

    pub fn build(self) -> MeasurementStyle {
        self.style
    }
}

pub enum Precision {
    None,
    Single,
    Double,
}
