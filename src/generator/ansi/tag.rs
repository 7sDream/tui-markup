use anstyle::{Ansi256Color, AnsiColor, Color, RgbColor, Style};

use crate::{
    generator::{
        TagConvertor,
        helper::{CustomTagParser, NoopCustomTagParser},
    },
    parser::hex_rgb,
};

/// Tag convertor for [`ANSIStringsGenerator`][super::ANSIStringsGenerator].
///
/// The generic type parameter `P` is a [`CustomTagParser`] that produces [`Style`] values
/// for custom tags.
#[derive(Debug)]
pub struct ANSITagConvertor<P = NoopCustomTagParser<Style>> {
    custom_parser: Option<P>,
}

impl<P> Default for ANSITagConvertor<P> {
    fn default() -> Self {
        Self {
            custom_parser: None,
        }
    }
}

impl<P> ANSITagConvertor<P> {
    /// Create a new tag convertor with custom tag parser.
    pub fn new(cp: P) -> Self {
        Self {
            custom_parser: Some(cp),
        }
    }
}

impl<'a, P> TagConvertor<'a> for ANSITagConvertor<P>
where
    P: CustomTagParser<Output = Style>,
{
    type Color = Color;
    type Custom = Style;
    type Modifier = Style;

    fn parse_color(&mut self, s: &str) -> Option<Self::Color> {
        Some(match s {
            "black" => AnsiColor::Black.into(),
            "red" => AnsiColor::Red.into(),
            "green" => AnsiColor::Green.into(),
            "yellow" => AnsiColor::Yellow.into(),
            "blue" => AnsiColor::Blue.into(),
            "purple" | "magenta" => AnsiColor::Magenta.into(),
            "cyan" => AnsiColor::Cyan.into(),
            "white" => AnsiColor::White.into(),
            s => hex_rgb(s)
                .map(|(r, g, b)| RgbColor(r, g, b).into())
                .or_else(|| s.parse::<u8>().ok().map(|n| Ansi256Color(n).into()))?,
        })
    }

    fn parse_modifier(&mut self, s: &str) -> Option<Self::Modifier> {
        Some(match s {
            "b" => Style::new().bold(),
            "d" => Style::new().dimmed(),
            "i" => Style::new().italic(),
            "u" => Style::new().underline(),
            "r" => Style::new().invert(),
            "sb" | "rb" => Style::new().blink(),
            "h" => Style::new().hidden(),
            "s" => Style::new().strikethrough(),
            _ => return None,
        })
    }

    fn parse_custom_tag(&mut self, s: &str) -> Option<Self::Custom> {
        self.custom_parser.as_mut().and_then(|p| p.parse(s))
    }
}
