use embedded_graphics::{
    mono_font::{
        iso_8859_10::{FONT_10X20, FONT_6X10, FONT_9X15},
        MonoFont, MonoTextStyle,
    },
    pixelcolor::Rgb888,
    prelude::{DrawTarget, PixelColor, Point, Size},
    primitives::{CornerRadiiBuilder, Primitive, PrimitiveStyle, Rectangle, RoundedRectangle},
    Drawable,
};
use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::TextBoxStyleBuilder,
    TextBox,
};

pub struct Card<'a, C> {
    fg_color: C,
    bg_color: C,
    top_left: Point,
    size: Size,
    text: &'a str,
    style: CardStyle<'a>,
}

impl<'a, C> Card<'a, C> {
    pub fn new(
        fg_color: C,
        bg_color: C,
        top_left: Point,
        size: Size,
        text: &'a str,
        style: CardStyle<'a>,
    ) -> Self {
        Self {
            fg_color,
            bg_color,
            top_left,
            size,
            text,
            style,
        }
    }
}

impl<'a, C> Drawable for Card<'a, C>
where
    C: PixelColor + From<Rgb888>,
{
    type Color = C;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let main_rectalngle = Rectangle::new(
            self.top_left,
            self.size - Size::new(0, self.style.bar_height),
        );

        main_rectalngle
            .into_styled(PrimitiveStyle::with_fill(self.bg_color))
            .draw(target)?;
        main_rectalngle
            .into_styled(PrimitiveStyle::with_stroke(self.fg_color, 1))
            .draw(target)?;

        let bar_rectangle = Rectangle::new(
            self.top_left + Point::new(0, self.size.height as i32 - self.style.bar_height as i32),
            Size::new(self.size.width, self.style.bar_height),
        );
        let radii = CornerRadiiBuilder::new().bottom(Size::new(6, 6)).build();
        RoundedRectangle::new(bar_rectangle, radii)
            .into_styled(PrimitiveStyle::with_fill(self.fg_color))
            .draw(target)?;

        let character_style = MonoTextStyle::new(&self.style.font, self.bg_color);

        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();

        TextBox::with_textbox_style(self.text, bar_rectangle, character_style, textbox_style)
            .draw(target)?;

        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct CardStyle<'a> {
    bar_height: u32,
    font: MonoFont<'a>,
}

impl<'a> CardStyle<'a> {
    pub fn with_bar_height(bar_height: u32) -> CardStyle<'a> {
        CardStyleBuilder::new().bar_height(bar_height).build()
    }

    pub fn with_font(font: MonoFont<'a>) -> CardStyle<'a> {
        CardStyleBuilder::new().font(font).build()
    }
}

impl<'a> Default for CardStyle<'a> {
    fn default() -> Self {
        CardStyleBuilder::new().build()
    }
}

pub struct CardStyleBuilder<'a> {
    style: CardStyle<'a>,
}

impl<'a> CardStyleBuilder<'a> {
    pub fn new() -> CardStyleBuilder<'a> {
        CardStyleBuilder {
            style: CardStyle {
                bar_height: 20,
                font: FONT_10X20,
            },
        }
    }

    pub fn medium() -> CardStyleBuilder<'a> {
        CardStyleBuilder {
            style: CardStyle {
                bar_height: 15,
                font: FONT_9X15,
            },
        }
    }

    pub fn small() -> CardStyleBuilder<'a> {
        CardStyleBuilder {
            style: CardStyle {
                bar_height: 10,
                font: FONT_6X10,
            },
        }
    }

    pub fn bar_height(mut self, bar_height: u32) -> CardStyleBuilder<'a> {
        self.style.bar_height = bar_height;
        self
    }

    pub fn font(mut self, font: MonoFont<'a>) -> CardStyleBuilder<'a> {
        self.style.font = font;
        self
    }

    pub fn build(self) -> CardStyle<'a> {
        self.style
    }
}
