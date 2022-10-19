use crossterm::style::{Attribute, Attributes, Color, ContentStyle};

use crate::{
    generator::{
        helper::{CustomTagParser, NoopCustomTagParser},
        TagConvertor,
    },
    parser::hex_rgb,
};

/// Tag convertor for [`CrosstermCommandsGenerator`](super::CrosstermCommandsGenerator).
#[cfg_attr(docsrs, doc(cfg(feature = "crossterm")))]
#[derive(Debug)]
pub struct CrosstermTagConvertor<P = NoopCustomTagParser<ContentStyle>> {
    custom_tag_parser: Option<P>,
}

impl<P> Default for CrosstermTagConvertor<P> {
    fn default() -> Self {
        Self {
            custom_tag_parser: None,
        }
    }
}

impl<P> CrosstermTagConvertor<P> {
    /// Create a new tag convertor with custom tag parser.
    pub fn new(p: P) -> Self {
        Self {
            custom_tag_parser: Some(p),
        }
    }
}

impl<'a, P> TagConvertor<'a> for CrosstermTagConvertor<P>
where
    P: CustomTagParser<Output = ContentStyle>,
{
    type Color = Color;

    type Modifier = Attributes;

    type Custom = ContentStyle;

    fn parse_color(&mut self, s: &str) -> Option<Self::Color> {
        Some(match s {
            "black" => Color::Black,
            "red" => Color::DarkRed,
            "green" => Color::DarkGreen,
            "yellow" => Color::DarkYellow,
            "blue" => Color::DarkBlue,
            "magenta" | "purple" => Color::DarkMagenta,
            "cyan" => Color::DarkCyan,
            "gray" => Color::Grey,
            "gray+" => Color::DarkGrey,
            "red-" => Color::Red,
            "green-" => Color::Green,
            "yellow-" => Color::Yellow,
            "blue-" => Color::Blue,
            "magenta-" | "purple-" => Color::Magenta,
            "cyan-" => Color::Cyan,
            "white" => Color::White,
            s => hex_rgb(s)
                .map(|(r, g, b)| Color::Rgb { r, g, b })
                .or_else(|| s.parse::<u8>().ok().map(Color::AnsiValue))?,
        })
    }

    fn parse_modifier(&mut self, s: &str) -> Option<Self::Modifier> {
        Some(
            match s {
                "b" => Attribute::Bold,
                "d" => Attribute::Dim,
                "i" => Attribute::Italic,
                "u" => Attribute::Underlined,
                "r" => Attribute::Reverse,
                "sb" => Attribute::SlowBlink,
                "rb" => Attribute::RapidBlink,
                "h" => Attribute::Hidden,
                "s" => Attribute::CrossedOut,
                _ => return None,
            }
            .into(),
        )
    }

    fn parse_custom_tag(&mut self, s: &str) -> Option<Self::Custom> {
        self.custom_tag_parser.as_mut().and_then(|f| f.parse(s))
    }
}
