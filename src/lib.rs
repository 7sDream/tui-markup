#![warn(clippy::all)]
#![warn(missing_docs, missing_debug_implementations)]
#![deny(warnings)]
#![forbid(unsafe_code)]

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
//! ## Examples
//!
//! ```
//! # use tui::{style::{Style, Color, Modifier}, text::{Text, Spans, Span}};
//! use tui_markup::parse;
//!
//! assert_eq!(
//!     parse("I have a <green green text>"),
//!     Ok(Text { lines: vec![Spans(vec![
//!         Span::raw("I have a "),
//!         Span::styled("green text", Style::default().fg(Color::Green)),
//!     ])] }),
//! );
//!
//! assert_eq!(
//!     parse("I can set <bg:blue background>"),
//!     Ok(Text { lines: vec![Spans(vec![
//!         Span::raw("I can set "),
//!         Span::styled("background", Style::default().bg(Color::Blue)),
//!     ])] }),
//! );
//!
//! assert_eq!(
//!     parse("I can add <b bold>, <d dim>, <i italic> modifiers"),
//!     Ok(Text { lines: vec![Spans(vec![
//!         Span::raw("I can add "),
//!         Span::styled("bold", Style::default().add_modifier(Modifier::BOLD)),
//!         Span::raw(", "),
//!         Span::styled("dim", Style::default().add_modifier(Modifier::DIM)),
//!         Span::raw(", "),
//!         Span::styled("italic", Style::default().add_modifier(Modifier::ITALIC)),
//!         Span::raw(" modifiers"),
//!     ])] }),
//! );
//!
//!
//! assert_eq!(
//!     parse("I can <bg:blue combine <green them <b <i all>>>>"),
//!     Ok(Text { lines: vec![Spans(vec![
//!         Span::raw("I can "),
//!         Span::styled("combine ", Style::default().bg(Color::Blue)),
//!         Span::styled("them ", Style::default().bg(Color::Blue).fg(Color::Green)),
//!         Span::styled("all", Style::default()
//!             .bg(Color::Blue).fg(Color::Green).add_modifier(Modifier::BOLD | Modifier::ITALIC)),
//!     ])] }),
//! );
//!
//! assert_eq!(
//!     parse("I can use <bg:66ccff custom color>"),
//!     Ok(Text { lines: vec![Spans(vec![
//!         Span::raw("I can use "),
//!         Span::styled("custom color", Style::default().bg(Color::Rgb(0x66, 0xcc, 0xff))),
//!     ])] }),
//! );
//!
//! assert_eq!(
//!     parse("I can set <bg:blue,green,b,i many style> in one tag"),
//!     Ok(Text { lines: vec![Spans(vec![
//!         Span::raw("I can set "),
//!         Span::styled("many style", Style::default()
//!             .bg(Color::Blue).fg(Color::Green).add_modifier(Modifier::BOLD | Modifier::ITALIC)),
//!         Span::raw(" in one tag"),
//!     ])] }),
//! );
//! ```
//!
//! [tui]: https://docs.rs/tui/latest/tui
//! [help-text-screenshot]: https://rikka.7sdre.am/files/37778eea-660b-47a6-bfd1-43979b5c703b.png
//! [help.txt]: https://github.com/7sDream/tui-markup/blob/master/examples/help.txt
//! [syntax.ebnf]: https://github.com/7sDream/tui-markup/blob/master/syntax.ebnf

mod item;
mod parser;
mod tag;

use tui::text::{Spans, Text};

/// Parse string in markup language to tui Text.
///
/// ## Errors
///
/// If provided string has invalid markup syntax, it will return `Err((s, (x, y)))`,
/// Where `s` is remainder string after error position, which is `x` line, `y` column.
pub fn parse(s: &str) -> Result<Text, (&str, (usize, usize))> {
    Ok(s.lines()
        .map(parser::parse)
        .enumerate()
        .map(|(line, spans)| spans.map_err(|(remain, col)| (remain, (line + 1, col))))
        .collect::<Result<Vec<Spans>, (&str, (usize, usize))>>()?
        .into())
}

#[cfg(test)]
mod lib_test {
    use super::parse;
    use tui::text::Text;

    #[test]
    fn test_empty_input() {
        assert_eq!(parse(""), Ok(Text { lines: vec![] }));
    }
}
