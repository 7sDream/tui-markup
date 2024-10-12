//! Generator implementations for ratatui crate.

mod span;
mod tag;

#[cfg(test)]
mod test;

use ratatui::{
    style::Style,
    text::{Line, Text},
};

use crate::{
    generator::{
        helper::{flatten, CustomTagParser, GeneratorInfallible, NoopCustomTagParser},
        Generator,
    },
    parser::ItemG,
};

pub use tag::RatatuiTagConvertor;

/// Generator for [ratatui crate][ratatui]'s [Text] type.
///
/// See [docs/ratatui-tags.ebnf] for supported tags.
///
/// ## Example
///
/// ```
/// # use ratatui::prelude::*;
/// use tui_markup::{compile, generator::RatatuiTextGenerator};
///
/// assert_eq!(
///     compile::<RatatuiTextGenerator>("I have a <green green text>"),
///     Ok(Text::from(vec![Line::from(vec![
///         Span::raw("I have a "),
///         Span::styled("green text", Style::default().fg(Color::Green)),
///     ])])),
/// );
///
/// assert_eq!(
///     compile::<RatatuiTextGenerator>("I can set <bg:blue background>"),
///     Ok(Text::from(vec![Line::from(vec![
///         Span::raw("I can set "),
///         Span::styled("background", Style::default().bg(Color::Blue)),
///     ])])),
/// );
///
/// assert_eq!(
///     compile::<RatatuiTextGenerator>("I can add <b bold>, <d dim>, <i italic> modifiers"),
///     Ok(Text::from(vec![Line::from(vec![
///         Span::raw("I can add "),
///         Span::styled("bold", Style::default().add_modifier(Modifier::BOLD)),
///         Span::raw(", "),
///         Span::styled("dim", Style::default().add_modifier(Modifier::DIM)),
///         Span::raw(", "),
///         Span::styled("italic", Style::default().add_modifier(Modifier::ITALIC)),
///         Span::raw(" modifiers"),
///     ])])),
/// );
///
///
/// assert_eq!(
///     compile::<RatatuiTextGenerator>("I can <bg:blue combine <green them <b <i all>>>>"),
///     Ok(Text::from(vec![Line::from(vec![
///         Span::raw("I can "),
///         Span::styled("combine ", Style::default().bg(Color::Blue)),
///         Span::styled("them ", Style::default().bg(Color::Blue).fg(Color::Green)),
///         Span::styled("all", Style::default()
///             .bg(Color::Blue).fg(Color::Green).add_modifier(Modifier::BOLD | Modifier::ITALIC)),
///     ])])),
/// );
///
/// assert_eq!(
///     compile::<RatatuiTextGenerator>("I can use <bg:66ccff custom color>"),
///     Ok(Text::from(vec![Line::from(vec![
///         Span::raw("I can use "),
///         Span::styled("custom color", Style::default().bg(Color::Rgb(0x66, 0xcc, 0xff))),
///     ])])),
/// );
///
/// assert_eq!(
///     compile::<RatatuiTextGenerator>("I can set <bg:blue,green,b,i many style> in one tag"),
///     Ok(Text::from(vec![Line::from(vec![
///         Span::raw("I can set "),
///         Span::styled("many style", Style::default()
///             .bg(Color::Blue).fg(Color::Green).add_modifier(Modifier::BOLD | Modifier::ITALIC)),
///         Span::raw(" in one tag"),
///     ])])),
/// );
/// ```
///
/// ### With custom tags
///
/// ```
/// # use ratatui::prelude::*;
/// use tui_markup::{compile_with, generator::RatatuiTextGenerator};
///
/// let gen =RatatuiTextGenerator::new(|tag: &str| match tag {
///     "keyboard" => Some(Style::default().bg(Color::White).fg(Color::Green).add_modifier(Modifier::BOLD)),
///     _ => None,
/// });
///
/// assert_eq!(
///     compile_with("Press <keyboard W> to move up", gen),
///     Ok(Text::from(vec![Line::from(vec![
///         Span::raw("Press "),
///         Span::styled("W", Style::default().bg(Color::White).fg(Color::Green).add_modifier(Modifier::BOLD)),
///         Span::raw(" to move up"),
///     ])])),
/// );
/// ```
///
/// ### Show output
///
/// Use any widget of [ratatui] crate that supports it's [Text] type, for example: [`ratatui::widgets::Paragraph`].
///
/// Note that the Paragraph widget includes a [`wrap`][ratatui::widgets::Paragraph::wrap] option
/// that defaults to trimming leading whitespace. You need to turn this option off if you require
/// full control over the output.
///
/// [docs/ratatui-tags.ebnf]: https://github.com/7sDream/tui-markup/blob/master/docs/ratatui-tags.ebnf
#[derive(Debug)]
pub struct RatatuiTextGenerator<P = NoopCustomTagParser<Style>> {
    convertor: RatatuiTagConvertor<P>,
}

impl<P> Default for RatatuiTextGenerator<P> {
    fn default() -> Self {
        Self {
            convertor: RatatuiTagConvertor::<P>::default(),
        }
    }
}

impl<P> RatatuiTextGenerator<P> {
    /// Create a new generator, with a custom tag parser.
    pub fn new(p: P) -> Self {
        RatatuiTextGenerator {
            convertor: RatatuiTagConvertor::new(p),
        }
    }
}

impl<'a, P> Generator<'a> for RatatuiTextGenerator<P>
where
    P: CustomTagParser<Output = Style>,
{
    type Convertor = RatatuiTagConvertor<P>;

    type Output = Text<'a>;

    type Err = GeneratorInfallible;

    fn convertor(&mut self) -> &mut Self::Convertor {
        &mut self.convertor
    }

    fn generate(&mut self, items: Vec<Vec<ItemG<'a, Self>>>) -> Result<Self::Output, Self::Err> {
        Ok(Text::from(
            items
                .into_iter()
                .map(|line| Line::from(flatten(line)))
                .collect::<Vec<_>>(),
        ))
    }
}
