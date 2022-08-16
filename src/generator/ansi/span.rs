use ansi_term::{ANSIString, Color, Style};

use crate::generator::{
    helper::{FlattenableSpan, FlattenableStyle},
    Tag, TagConvertor,
};

impl<'a, C> From<Tag<'a, C>> for Style
where
    C: TagConvertor<'a, Color = Color, Modifier = Style, Custom = Style>,
{
    fn from(t: Tag<'a, C>) -> Self {
        match t {
            Tag::Fg(c) => Style::default().fg(c),
            Tag::Bg(c) => Style::default().on(c),
            Tag::Modifier(style) => style,
            Tag::Custom(style) => style,
        }
    }
}

impl FlattenableStyle for Style {
    fn patch(mut self, other: Self) -> Self {
        if let Some(fg) = other.foreground {
            self.foreground = Some(fg);
        }
        if let Some(bg) = other.background {
            self.background = Some(bg);
        }
        if other.is_bold {
            self.is_bold = true;
        }
        if other.is_dimmed {
            self.is_dimmed = true;
        }
        if other.is_italic {
            self.is_italic = true;
        }
        if other.is_underline {
            self.is_underline = true;
        }
        if other.is_reverse {
            self.is_reverse = true;
        }
        if other.is_blink {
            self.is_blink = true;
        }
        if other.is_hidden {
            self.is_hidden = true;
        }
        if other.is_strikethrough {
            self.is_strikethrough = true;
        }

        self
    }
}

impl<'a> FlattenableSpan<'a, Style> for ANSIString<'a> {
    fn with_style(s: &'a str, style: Option<Style>) -> Self {
        style.unwrap_or_default().paint(s)
    }
}
