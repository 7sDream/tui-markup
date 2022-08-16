use tui::style::{Color, Modifier, Style};

use crate::{generator::helper::CustomTagParser, generator::TagConvertor, parser::hex_rgb};

/// Tag convertor for tui crate.
#[cfg_attr(docsrs, doc(cfg(feature = "tui")))]
#[derive(Debug)]
pub struct TuiTagConvertor<CP> {
    custom_parser: Option<CP>,
}

impl<CP> Default for TuiTagConvertor<CP> {
    fn default() -> Self {
        Self {
            custom_parser: Default::default(),
        }
    }
}

impl<CP> TuiTagConvertor<CP> {
    /// Create a new tag convertor with custom tag parser.
    pub fn new(cp: CP) -> Self {
        Self {
            custom_parser: Some(cp),
        }
    }
}

impl<'a, CP> TagConvertor<'a> for TuiTagConvertor<CP>
where
    CP: CustomTagParser<Output = Style>,
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
            s => hex_rgb(s).map(|(r, g, b)| Color::Rgb(r, g, b))?,
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
        self.custom_parser.as_mut().and_then(|f| f.parse(s))
    }
}
