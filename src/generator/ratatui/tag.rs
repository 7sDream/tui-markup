use ratatui::style::{Color, Modifier, Style};

use crate::{
    generator::helper::{CustomTagParser, NoopCustomTagParser},
    generator::TagConvertor,
    parser::hex_rgb,
};

/// Tag convertor for [`RatatuiTextGenerator`](super::RatatuiTextGenerator).
#[derive(Debug)]
pub struct RatatuiTagConvertor<P = NoopCustomTagParser<Style>> {
    custom_tag_parser: Option<P>,
}

impl<P> Default for RatatuiTagConvertor<P> {
    fn default() -> Self {
        Self {
            custom_tag_parser: None,
        }
    }
}

impl<P> RatatuiTagConvertor<P> {
    /// Create a new tag convertor with custom tag parser.
    pub fn new(p: P) -> Self {
        Self {
            custom_tag_parser: Some(p),
        }
    }
}

impl<'a, P> TagConvertor<'a> for RatatuiTagConvertor<P>
where
    P: CustomTagParser<Output = Style>,
{
    type Color = Color;
    type Modifier = Modifier;
    type Custom = Style;

    fn parse_color(&mut self, s: &str) -> Option<Color> {
        Some(match s {
            "black" => Color::Black,
            "red" => Color::Red,
            "green" => Color::Green,
            "yellow" => Color::Yellow,
            "blue" => Color::Blue,
            "magenta" | "purple" => Color::Magenta,
            "cyan" => Color::Cyan,
            "gray" => Color::Gray,
            "gray+" => Color::DarkGray,
            "red-" => Color::LightRed,
            "green-" => Color::LightGreen,
            "yellow-" => Color::LightYellow,
            "blue-" => Color::LightBlue,
            "magenta-" | "purple-" => Color::LightMagenta,
            "cyan-" => Color::LightCyan,
            "white" => Color::White,
            s => hex_rgb(s)
                .map(|(r, g, b)| Color::Rgb(r, g, b))
                .or_else(|| s.parse::<u8>().ok().map(Color::Indexed))?,
        })
    }

    fn parse_modifier(&mut self, s: &str) -> Option<Modifier> {
        Some(match s {
            "b" => Modifier::BOLD,
            "d" => Modifier::DIM,
            "i" => Modifier::ITALIC,
            "u" => Modifier::UNDERLINED,
            "r" => Modifier::REVERSED,
            "sb" => Modifier::SLOW_BLINK,
            "rb" => Modifier::RAPID_BLINK,
            "h" => Modifier::HIDDEN,
            "s" => Modifier::CROSSED_OUT,
            _ => return None,
        })
    }

    fn parse_custom_tag(&mut self, s: &str) -> Option<Style> {
        self.custom_tag_parser.as_mut().and_then(|f| f.parse(s))
    }
}
