use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

use crate::generator::{
    helper::{FlattenableSpan, FlattenableStyle},
    Tag, TagConvertor,
};

impl<'a, C> From<Tag<'a, C>> for Style
where
    C: TagConvertor<'a, Color = Color, Modifier = Modifier, Custom = Style>,
{
    fn from(t: Tag<'a, C>) -> Self {
        match t {
            Tag::Fg(c) => Style::default().fg(c),
            Tag::Bg(c) => Style::default().bg(c),
            Tag::Modifier(m) => Style::default().add_modifier(m),
            Tag::Custom(style) => style,
        }
    }
}

impl FlattenableStyle for Style {
    fn patch(self, other: Self) -> Self {
        self.patch(other)
    }
}

impl<'a> FlattenableSpan<'a, Style> for Span<'a> {
    fn with_style(s: &'a str, style: Option<Style>) -> Self {
        match style {
            Some(style) => Span::styled(s, style),
            None => Span::raw(s),
        }
    }
}
