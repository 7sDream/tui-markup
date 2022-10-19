use crossterm::{
    style::{Attributes, Color, ContentStyle, Print, PrintStyledContent, Stylize},
    Command,
};

use crate::generator::{
    helper::{FlattenableSpan, FlattenableStyle},
    Tag, TagConvertor,
};

impl<'a, C> From<Tag<'a, C>> for ContentStyle
where
    C: TagConvertor<'a, Color = Color, Modifier = Attributes, Custom = ContentStyle>,
{
    fn from(t: Tag<'a, C>) -> Self {
        match t {
            Tag::Fg(c) => ContentStyle::new().with(c),
            Tag::Bg(c) => ContentStyle::new().on(c),
            Tag::Modifier(m) => {
                let mut c = ContentStyle::new();
                c.attributes = m;
                c
            }
            Tag::Custom(style) => style,
        }
    }
}

impl FlattenableStyle for ContentStyle {
    fn patch(mut self, other: Self) -> Self {
        if let Some(c) = other.foreground_color {
            self = self.with(c);
        }

        if let Some(c) = other.background_color {
            self = self.on(c);
        }

        self.attributes.extend(other.attributes);

        self
    }
}

/// Span is a crossterm Command for print raw or styled text.
#[derive(Debug)]
pub enum Span<'a> {
    /// Print raw text
    NoStyle(Print<&'a str>),
    /// Print styled text
    Styled(PrintStyledContent<&'a str>),
}

impl<'a> FlattenableSpan<'a, ContentStyle> for Span<'a> {
    fn with_style(s: &'a str, style: Option<ContentStyle>) -> Self {
        match style {
            Some(style) => Span::Styled(PrintStyledContent(style.apply(s))),
            None => Span::NoStyle(Print(s)),
        }
    }
}

impl<'a> Command for Span<'a> {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        match self {
            Self::NoStyle(p) => p.write_ansi(f),
            Self::Styled(p) => p.write_ansi(f),
        }
    }
}
