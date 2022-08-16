use ansi_term::{Color, Style};

use crate::{
    generator::{
        helper::{CustomTagParser, NoopCustomTagParser},
        TagConvertor,
    },
    parser::hex_rgb,
};

/// Tag convertor for ansi term.
#[cfg_attr(docsrs, doc(cfg(feature = "ansi")))]
#[derive(Debug)]
pub struct ANSITermTagConvertor<CP = NoopCustomTagParser<Style>> {
    custom_parser: Option<CP>,
}

impl<CP> Default for ANSITermTagConvertor<CP> {
    fn default() -> Self {
        Self {
            custom_parser: Default::default(),
        }
    }
}

impl<CP> ANSITermTagConvertor<CP> {
    /// Create a new tag convertor with custom tag parser.
    pub fn new(cp: CP) -> Self {
        Self {
            custom_parser: Some(cp),
        }
    }
}

impl<'a, CP> TagConvertor<'a> for ANSITermTagConvertor<CP>
where
    CP: CustomTagParser<Output = Style>,
{
    type Color = Color;

    type Modifier = Style;

    type Custom = Style;

    fn parse_color(&mut self, s: &str) -> Option<Self::Color> {
        Some(match s {
            "black" => Color::Black,
            "red" => Color::Red,
            "green" => Color::Green,
            "yellow" => Color::Yellow,
            "blue" => Color::Blue,
            "purple" | "magenta" => Color::Purple,
            "cyan" => Color::Cyan,
            "white" => Color::White,
            s => hex_rgb(s).map(|(r, g, b)| Color::RGB(r, g, b))?,
        })
    }

    fn parse_modifier(&mut self, s: &str) -> Option<Self::Modifier> {
        Some(match s {
            "b" => Style::default().bold(),
            "d" => Style::default().dimmed(),
            "i" => Style::default().italic(),
            "u" => Style::default().underline(),
            "r" => Style::default().reverse(),
            "sb" | "rb" => Style::default().blink(),
            "h" => Style::default().hidden(),
            "s" => Style::default().strikethrough(),
            _ => return None,
        })
    }

    fn parse_custom_tag(&mut self, s: &str) -> Option<Self::Custom> {
        self.custom_parser.as_mut().and_then(|p| p.parse(s))
    }
}
