use ansi_term::{ANSIString, Color, Style};

use crate::generator::{GenericSpan, GenericStyle, Tag, TagConvertor};

/// Wrapped ansi term style.
#[derive(Debug, Default, Clone, Copy)]
pub struct WrappedStyle(pub Style);

impl<'a, C> From<Tag<'a, C>> for WrappedStyle
where
    C: TagConvertor<'a, Color = Color, Modifier = Style, Custom = Style>,
{
    fn from(t: Tag<'a, C>) -> Self {
        Self(match t {
            Tag::Fg(c) => Style::default().fg(c),
            Tag::Bg(c) => Style::default().on(c),
            Tag::Modifier(style) => style,
            Tag::Custom(style) => style,
        })
    }
}

impl GenericStyle for WrappedStyle {
    fn patch(mut self, other: Self) -> Self {
        let style = &mut self.0;
        let other = other.0;

        if let Some(fg) = other.foreground {
            style.foreground = Some(fg);
        }
        if let Some(bg) = other.background {
            style.background = Some(bg);
        }
        if other.is_bold {
            style.is_bold = true;
        }
        if other.is_dimmed {
            style.is_dimmed = true;
        }
        if other.is_italic {
            style.is_italic = true;
        }
        if other.is_underline {
            style.is_underline = true;
        }
        if other.is_reverse {
            style.is_reverse = true;
        }
        if other.is_blink {
            style.is_blink = true;
        }
        if other.is_hidden {
            style.is_hidden = true;
        }
        if other.is_strikethrough {
            style.is_strikethrough = true;
        }

        self
    }
}

impl<'a> GenericSpan<'a, WrappedStyle> for ANSIString<'a> {
    fn with_style(s: &'a str, style: Option<WrappedStyle>) -> Self {
        style.unwrap_or_default().0.paint(s)
    }
}
