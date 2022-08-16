//! Generator implementations for tui crate.

mod span;
mod tag;

#[cfg(test)]
mod test;

use tui::{
    style::Style,
    text::{Spans, Text},
};

use super::{
    helper::{flatten, CustomTagParser, GeneratorInfallible, NoopCustomTagParser},
    Generator,
};
use crate::{generator::TagG, parser::Item};

pub use tag::TuiTagConvertor;

/// Generator for [tui crate][tui]'s [Text] type.
///
/// See [docs/tui-tags.ebnf] for supported tags.
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
/// ### Show output
///
/// Use any widget of [tui] crate that supports it's [Text] type, for example: [`tui::widgets::Paragraph`].
///
/// [docs/tui-tags.ebnf]: https://github.com/7sDream/tui-markup/blob/master/docs/tui-tags.ebnf
#[cfg_attr(docsrs, doc(cfg(feature = "tui")))]
#[derive(Debug)]
pub struct TuiTextGenerator<P = NoopCustomTagParser<Style>> {
    convertor: TuiTagConvertor<P>,
}

impl<P> Default for TuiTextGenerator<P> {
    fn default() -> Self {
        Self {
            convertor: Default::default(),
        }
    }
}

impl<P> TuiTextGenerator<P> {
    /// Create a new generator, with a custom tag parser.
    pub fn new(p: P) -> Self {
        TuiTextGenerator {
            convertor: TuiTagConvertor::new(p),
        }
    }
}

impl<'a, P> Generator<'a> for TuiTextGenerator<P>
where
    P: CustomTagParser<Output = Style>,
{
    type Convertor = TuiTagConvertor<P>;

    type Output = Text<'a>;

    type Err = GeneratorInfallible;

    fn convertor(&mut self) -> &mut Self::Convertor {
        &mut self.convertor
    }

    fn generate(&mut self, items: Vec<Vec<Item<'a, TagG<'a, Self>>>>) -> Result<Self::Output, Self::Err> {
        Ok(Text::from(
            items
                .into_iter()
                .map(|line| Spans::from(flatten(line)))
                .collect::<Vec<_>>(),
        ))
    }
}
