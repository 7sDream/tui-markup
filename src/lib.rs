#![warn(clippy::all)]
#![warn(missing_docs, missing_debug_implementations)]
#![deny(warnings)]
#![cfg_attr(not(test), forbid(unsafe_code))]

//! # tui markup
//!
//! This crate provides a markup language to
//! quickly write colorful and styled terminal text(of [tui] crate) in plain text.
//!
//! I suggest to check [help.txt] in examples folder,
//! which generated this self-describing syntax help document:
//!
//! ![][help-text-screenshot]
//!
//! For formal syntax specification, see [syntax.ebnf].
//!
//! [tui]: https://docs.rs/tui/latest/tui
//! [help-text-screenshot]: https://rikka.7sdre.am/files/37778eea-660b-47a6-bfd1-43979b5c703b.png
//! [help.txt]: https://github.com/7sDream/tui-markup/blob/master/examples/help.txt
//! [syntax.ebnf]: https://github.com/7sDream/tui-markup/blob/master/syntax.ebnf

mod error;
mod item;
mod parser;
mod tag;

use tui::{style::Style, text::Text};

pub use error::Error;

/// Parse string in markup language to tui Text.
///
/// ## Examples
///
/// ```
/// # use tui::{style::{Style, Color, Modifier}, text::{Text, Spans, Span}};
/// use tui_markup::parse;
///
/// assert_eq!(
///     parse("I have a <green green text>"),
///     Ok(Text { lines: vec![Spans(vec![
///         Span::raw("I have a "),
///         Span::styled("green text", Style::default().fg(Color::Green)),
///     ])] }),
/// );
///
/// assert_eq!(
///     parse("I can set <bg:blue background>"),
///     Ok(Text { lines: vec![Spans(vec![
///         Span::raw("I can set "),
///         Span::styled("background", Style::default().bg(Color::Blue)),
///     ])] }),
/// );
///
/// assert_eq!(
///     parse("I can add <b bold>, <d dim>, <i italic> modifiers"),
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
///     parse("I can <bg:blue combine <green them <b <i all>>>>"),
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
///     parse("I can use <bg:66ccff custom color>"),
///     Ok(Text { lines: vec![Spans(vec![
///         Span::raw("I can use "),
///         Span::styled("custom color", Style::default().bg(Color::Rgb(0x66, 0xcc, 0xff))),
///     ])] }),
/// );
///
/// assert_eq!(
///     parse("I can set <bg:blue,green,b,i many style> in one tag"),
///     Ok(Text { lines: vec![Spans(vec![
///         Span::raw("I can set "),
///         Span::styled("many style", Style::default()
///             .bg(Color::Blue).fg(Color::Green).add_modifier(Modifier::BOLD | Modifier::ITALIC)),
///         Span::raw(" in one tag"),
///     ])] }),
/// );
/// ```
///
/// ## Errors
///
/// If provided string has invalid markup syntax or unknown tag.
pub fn parse(s: &str) -> Result<Text, Error> {
    parse_with_extra_tags(s, |_| None)
}

/// Parse string in markup language to tui Text, with custom defined tags.
///
/// ## Examples
///
/// ```
/// # use tui::{style::{Style, Color, Modifier}, text::{Text, Spans, Span}};
/// use tui_markup::parse_with_extra_tags as parse;
///
/// assert_eq!(
///     parse(
///         "Press <keyboard W> to move up",
///         |tag| match tag {
///             "keyboard" => Some(Style::default().bg(Color::White).fg(Color::Green).add_modifier(Modifier::BOLD)),
///             _ => None
///         },
///     ),
///     Ok(Text { lines: vec![Spans(vec![
///         Span::raw("Press "),
///         Span::styled("W", Style::default().bg(Color::White).fg(Color::Green).add_modifier(Modifier::BOLD)),
///         Span::raw(" to move up"),
///     ])] }),
/// );
/// ```
///
/// ## Errors
///
/// If provided string has invalid markup syntax or unknown tag.
pub fn parse_with_extra_tags<F>(s: &str, mut f: F) -> Result<Text, Error>
where
    F: FnMut(&str) -> Option<Style>,
{
    let mut result = vec![];

    for spans in s
        .lines()
        .enumerate()
        .map(|(line, source)| parser::parse(source, line, &mut f))
    {
        let spans = spans?;
        result.push(spans.into());
    }

    Ok(result.into())
}

#[cfg(test)]
mod lib_test {
    use super::parse;
    use tui::text::Text;

    #[test]
    fn test_empty_input() {
        assert_eq!(parse("").unwrap(), Text { lines: vec![] });
    }
}
