//! Generator implementations tui crate.

mod error;
mod tag;
#[cfg(test)]
mod test;

pub use error::{Error, ErrorKind};

use tui::{
    style::Style,
    text::{Span, Spans, Text},
};

use super::{helper::unescape, Generator};
use crate::parser::{Item, LSpan};
use tag::{CustomTagProvider, Tags};

/// Generator for [tui crate][tui]'s [Text] type.
///
/// See [tui-tags.ebnf] for supported tags.
///
/// ## Example
///
/// ```
/// # use tui::{style::{Style, Color, Modifier}, text::{Text, Spans, Span}};
/// use tui_markup::{compile, generator::TuiTextGenerator};
///
/// assert_eq!(
///     compile::<TuiTextGenerator>("I have a <green green text>"),
///     Ok(Text { lines: vec![Spans(vec![
///         Span::raw("I have a "),
///         Span::styled("green text", Style::default().fg(Color::Green)),
///     ])] }),
/// );
///
/// assert_eq!(
///     compile::<TuiTextGenerator>("I can set <bg:blue background>"),
///     Ok(Text { lines: vec![Spans(vec![
///         Span::raw("I can set "),
///         Span::styled("background", Style::default().bg(Color::Blue)),
///     ])] }),
/// );
///
/// assert_eq!(
///     compile::<TuiTextGenerator>("I can add <b bold>, <d dim>, <i italic> modifiers"),
///     Ok(Text { lines: vec![Spans(vec![
///         Span::raw("I can add "),
///         Span::styled("bold", Style::default().add_modifier(Modifier::BOLD)),
///         Span::raw(", "),
///         Span::styled("dim", Style::default().add_modifier(Modifier::DIM)),
///         Span::raw(", "),
///         Span::styled("italic", Style::default().add_modifier(Modifier::ITALIC)),
///         Span::raw(" modifiers"),
///     ])] }),
/// );
///
///
/// assert_eq!(
///     compile::<TuiTextGenerator>("I can <bg:blue combine <green them <b <i all>>>>"),
///     Ok(Text { lines: vec![Spans(vec![
///         Span::raw("I can "),
///         Span::styled("combine ", Style::default().bg(Color::Blue)),
///         Span::styled("them ", Style::default().bg(Color::Blue).fg(Color::Green)),
///         Span::styled("all", Style::default()
///             .bg(Color::Blue).fg(Color::Green).add_modifier(Modifier::BOLD | Modifier::ITALIC)),
///     ])] }),
/// );
///
/// assert_eq!(
///     compile::<TuiTextGenerator>("I can use <bg:66ccff custom color>"),
///     Ok(Text { lines: vec![Spans(vec![
///         Span::raw("I can use "),
///         Span::styled("custom color", Style::default().bg(Color::Rgb(0x66, 0xcc, 0xff))),
///     ])] }),
/// );
///
/// assert_eq!(
///     compile::<TuiTextGenerator>("I can set <bg:blue,green,b,i many style> in one tag"),
///     Ok(Text { lines: vec![Spans(vec![
///         Span::raw("I can set "),
///         Span::styled("many style", Style::default()
///             .bg(Color::Blue).fg(Color::Green).add_modifier(Modifier::BOLD | Modifier::ITALIC)),
///         Span::raw(" in one tag"),
///     ])] }),
/// );
/// ```
///
/// ### With custom tags
///
/// ```
/// # use tui::{style::{Style, Color, Modifier}, text::{Text, Spans, Span}};
/// use tui_markup::{compile_with, generator::TuiTextGenerator};
///
/// let gen = TuiTextGenerator::new(|tag: &str| match tag {
///     "keyboard" => Some(Style::default().bg(Color::White).fg(Color::Green).add_modifier(Modifier::BOLD)),
///     _ => None,
/// });
///
/// assert_eq!(
///     compile_with("Press <keyboard W> to move up", gen),
///     Ok(Text { lines: vec![Spans(vec![
///         Span::raw("Press "),
///         Span::styled("W", Style::default().bg(Color::White).fg(Color::Green).add_modifier(Modifier::BOLD)),
///         Span::raw(" to move up"),
///     ])] }),
/// );
/// ```
///
/// [tui]: https://docs.rs/tui/latest/tui/
/// [tui-tags.ebnf]: https://github.com/7sDream/master/blob/docs/tui-tag.ebnf
#[cfg_attr(docsrs, doc(cfg(feature = "tui")))]
#[derive(Debug)]
pub struct TuiTextGenerator<P = ()> {
    custom_tags: Option<P>,
}

impl<P> Default for TuiTextGenerator<P> {
    fn default() -> Self {
        Self {
            custom_tags: Default::default(),
        }
    }
}

impl<F> TuiTextGenerator<F> {
    /// Create a new generator, with custom tags.
    pub fn new(f: F) -> Self {
        TuiTextGenerator { custom_tags: Some(f) }
    }

    fn plain_text<'a, 'b>(&'a self, escaped: &'b str, style: Option<Style>) -> Vec<Span<'b>> {
        unescape(escaped)
            .map(|s| match style {
                Some(style) => Span::styled(s, style),
                None => Span::raw(s),
            })
            .collect()
    }
}

impl<P> TuiTextGenerator<P>
where
    P: CustomTagProvider,
{
    fn element<'a, 'b>(
        &'a mut self, tags: Vec<LSpan<'b>>, children: Vec<Item<'b>>, style: Option<Style>,
    ) -> Result<Vec<Span<'b>>, Error<'b>> {
        let style = style
            .unwrap_or_default()
            .patch(Tags::parse(tags, &mut self.custom_tags)?.style());

        self.items(children, Some(style))
    }

    fn item<'a, 'b>(&'a mut self, item: Item<'b>, style: Option<Style>) -> Result<Vec<Span<'b>>, Error<'b>> {
        match item {
            Item::PlainText(t) => Ok(self.plain_text(t, style)),
            Item::Element(tags, children) => self.element(tags, children, style),
        }
    }

    fn items<'a, 'b>(&'a mut self, items: Vec<Item<'b>>, style: Option<Style>) -> Result<Vec<Span<'b>>, Error<'b>> {
        let mut result = vec![];

        for item in items {
            result.extend(self.item(item, style)?);
        }

        Ok(result)
    }
}

impl<'a, P> Generator<'a> for TuiTextGenerator<P>
where
    P: CustomTagProvider,
{
    type Output = Text<'a>;

    type Err = Error<'a>;

    fn generate(&mut self, items: Vec<Vec<Item<'a>>>) -> Result<Self::Output, Self::Err> {
        items
            .into_iter()
            .map(|line| self.items(line, None).map(Spans::from))
            .collect::<Result<Vec<_>, _>>()
            .map(Text::from)
    }
}
