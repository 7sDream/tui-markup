#![warn(clippy::all)]
#![warn(missing_docs, missing_debug_implementations)]
#![deny(warnings)]
#![forbid(unsafe_code)]

//! # tui markup
//!
//! This crate provides a markup language to
//! quickly write colorful and styled terminal text(of [tui] crate) in plain text.
//!
//! The example and syntax section bellow is just for formal specifications,
//! for normal usage and learning purpose,
//! I suggest to see [help.txt] in examples folder, which generate this output:
//!
//! ![][help-text-screenshot]
//!
//! ## Examples
//!
//! ```
//! use tui::{style::{Style, Color, Modifier}, text::{Text, Spans, Span}};
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
//! ## Syntax
//!
//! Each line is parsed independently,
//! so there is no newline character in the input set of following grammar rules.
//!
//! ```none
//! items = item*
//!
//! item = text
//!      | element
//!
//! text = char*
//!
//! char = normal-char
//!      | escaped-char
//!
//! normal-char = all character except '\', '<', '>'
//!
//! escaped-char = "\\"
//!              | "\>"
//!              | "\>"
//!
//! element = '<' tag ' ' items '>'
//!
//! tag = style (',' style)*
//!
//! style = ("fg:" | "bg:")? color
//!       | "mod:"? modifier
//!
//! color = "black" | "white"
//!       | light-variant-colors '-'?
//!       | dark-variant-colors '+'?
//!       | custom-color
//!
//! light-variant-colors = "red" | "green" | "yellow" | "blue" | "magenta" | "cyan"
//!
//! dark-variant-colors = "gray"
//!
//! custom-color = hex-digit{6}
//!
//! hex-digit = '0' | '1' | ... | '9' | 'a' | 'b' | ... | 'f' | 'A' | 'B' | ... | 'F'
//!
//! modifier = 'b'            # bold
//!          | 'd'            # dim
//!          | 'i'            # italic
//!          | 'u'            # underline
//!          | 's'            # slow blink
//!          | 'r'            # rapid blink
//!          | 'h'            # hide
//!          | 'x'            # cross/delete line
//! ```
//!
//! PS: `-` after color means light color, `+` means dark,
//! e.g., `green-` = light green, `gray+` = dark gray.
//!
//! [tui]: https://docs.rs/tui/latest/tui
//! [help-text-screenshot]: https://raw.githubusercontent.com/7sDream/tui-markup/master/examples/help-screenshot.png
//! [help.txt]: https://github.com/7sDream/tui-markup/blob/master/examples/help.txt

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
mod test {
    use super::parse;
    use tui::text::Text;

    #[test]
    fn test_ok_with_empty_input() {
        assert_eq!(parse(""), Ok(Text { lines: vec![] }));
    }
}
