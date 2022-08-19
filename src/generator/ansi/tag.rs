use ansi_term::{Color, Style};

use crate::{
    generator::{
        helper::{CustomTagParser, NoopCustomTagParser},
        TagConvertor,
    },
    parser::hex_rgb,
};

/// Tag convertor for [`ANSIStringsGenerator`][super::ANSIStringsGenerator].
#[cfg_attr(docsrs, doc(cfg(feature = "ansi")))]
#[derive(Debug)]
pub struct ANSITermTagConvertor<P = NoopCustomTagParser<Style>> {
    custom_parser: Option<P>,
}

impl<P> Default for ANSITermTagConvertor<P> {
    fn default() -> Self {
        Self { custom_parser: None }
    }
}

impl<P> ANSITermTagConvertor<P> {
    /// Create a new tag convertor with custom tag parser.
    pub fn new(cp: P) -> Self {
        Self {
            custom_parser: Some(cp),
        }
    }
}

impl<'a, P> TagConvertor<'a> for ANSITermTagConvertor<P>
where
    P: CustomTagParser<Output = Style>,
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
            s => hex_rgb(s)
                .map(|(r, g, b)| Color::RGB(r, g, b))
                .or_else(|| s.parse::<u8>().ok().map(Color::Fixed))?,
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
