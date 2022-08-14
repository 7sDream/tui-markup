//! Generator implementations tui crate.

mod error;
mod tag;

pub use error::{Error, ErrorKind};

use tui::{
    style::Style,
    text::{Span, Spans, Text},
};

use super::Generator;
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

    fn generate_from_plain_text<'a, 'b>(&'a self, escaped: &'b str, style: Option<Style>) -> Vec<Span<'b>> {
        let mut spans = vec![];

        let mut push_span = |start: usize, end: usize| {
            if end > start {
                let t = &escaped[start..end];
                spans.push(if let Some(style) = style {
                    Span::styled(t, style)
                } else {
                    Span::raw(t)
                });
            }
        };

        let mut start = 0;
        let mut last_is_escape = false;
        for (idx, c) in escaped.char_indices() {
            if !last_is_escape && c == '\\' {
                push_span(start, idx);
                last_is_escape = true;
                start = idx + c.len_utf8();
            } else {
                last_is_escape = false;
            }
        }
        push_span(start, escaped.len());

        spans
    }
}

impl<P> TuiTextGenerator<P>
where
    P: CustomTagProvider,
{
    fn generate_from_element<'a, 'b>(
        &'a mut self, tags: Vec<LSpan<'b>>, children: Vec<Item<'b>>, style: Option<Style>,
    ) -> Result<Vec<Span<'b>>, Error<'b>> {
        let style = style
            .unwrap_or_default()
            .patch(Tags::parse(tags, &mut self.custom_tags)?.style());

        self.generate_from_items(children, Some(style))
    }

    fn generate_from_item<'a, 'b>(
        &'a mut self, item: Item<'b>, style: Option<Style>,
    ) -> Result<Vec<Span<'b>>, Error<'b>> {
        match item {
            Item::PlainText(t) => Ok(self.generate_from_plain_text(t, style)),
            Item::Element(tags, children) => self.generate_from_element(tags, children, style),
        }
    }

    fn generate_from_items<'a, 'b>(
        &'a mut self, line: Vec<Item<'b>>, style: Option<Style>,
    ) -> Result<Vec<Span<'b>>, Error<'b>> {
        let mut result = vec![];

        for item in line {
            result.extend(self.generate_from_item(item, style)?);
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
            .map(|line| self.generate_from_items(line, None).map(Spans::from))
            .collect::<Result<Vec<_>, _>>()
            .map(Text::from)
    }
}

#[cfg(test)]
mod tui_test {
    use tui::{
        style::{Color, Modifier, Style},
        text::Span,
    };

    use crate::generator::tui::ErrorKind;

    macro_rules! test_plain_text {
        ($s:literal $(, $result:literal)*) => {
            assert_eq!(
                crate::generator::TuiTextGenerator::<()>::default().generate_from_item(crate::parser::Item::PlainText($s), None),
                Ok(vec![$(::tui::text::Span::raw($result),)*]),
            );
        };
    }

    macro_rules! pt {
        ($text:literal) => {
            crate::parser::Item::PlainText($text)
        };
    }

    macro_rules! elem {
        (@tags, $($s:literal),+) => {
            vec![$(crate::parser::LSpan::new_extra($s, 1)),+]
        };
        ($($tags:tt),* ; $($items:expr),* $(,)?) => {
            crate::parser::Item::Element(elem!(@tags, $($tags),*), vec![$($items),*])
        };
    }

    macro_rules! test_element {
        ($elem:expr => $($result:tt)*) => {
            assert_eq!(
                crate::generator::TuiTextGenerator::<()>::default().generate_from_item($elem, None),
                Ok(vec![$($result)*]),
            )
        };
        ($custom:expr ; $elem:expr => $($result:tt)*) => {
            assert_eq!(
                crate::generator::TuiTextGenerator::new($custom).generate_from_item($elem, None),
                Ok(vec![$($result)*]),
            )
        };
    }

    macro_rules! test_fail {
        ($elem:expr => $span:literal, $kind:expr) => {
            let err = crate::generator::TuiTextGenerator::<()>::default()
                .generate_from_item($elem, None)
                .unwrap_err();
            assert_eq!(*err.span.fragment(), $span);
            assert_eq!(err.kind(), $kind);
        };
    }

    #[test]
    fn test_escaped_string() {
        test_plain_text!("a\\<b", "a", "<b");
        test_plain_text!("a\\>b", "a", ">b");
        test_plain_text!("a\\\\b", "a", "\\b");
    }

    #[test]
    fn test_escaped_string_at_begin() {
        test_plain_text!("\\<b", "<b");
        test_plain_text!("\\>b", ">b");
        test_plain_text!("\\\\b", "\\b");
    }

    #[test]
    fn test_escaped_string_at_end() {
        test_plain_text!("a\\<", "a", "<");
        test_plain_text!("a\\>", "a", ">");
        test_plain_text!("a\\\\", "a", "\\");
    }

    #[test]
    fn test_normal_element() {
        test_element!(elem!("green" ; pt!("xxx")) => Span::styled("xxx", Style::default().fg(Color::Green)));
        test_element!(elem!("fg:red" ; pt!("xxx")) => Span::styled("xxx", Style::default().fg(Color::Red)));
        test_element!(elem!("bg:yellow" ; pt!("xxx")) => Span::styled("xxx", Style::default().bg(Color::Yellow)));
        test_element!(elem!("b" ; pt!("xxx")) => Span::styled("xxx", Style::default().add_modifier(Modifier::BOLD)));
        test_element!(elem!("mod:i" ; pt!("xxx")) => Span::styled("xxx", Style::default().add_modifier(Modifier::ITALIC)));
    }

    #[test]
    fn test_nested_element() {
        test_element!(
            elem!("bg:blue" ; pt!("one "), elem!("green" ; pt!("two"))) =>
            Span::styled("one ", Style::default().bg(Color::Blue)),
            Span::styled("two", Style::default().bg(Color::Blue).fg(Color::Green)),
        );
    }

    #[test]
    fn test_multi_tag_element() {
        test_element!(
            elem!("bg:blue", "green", "b" ; pt!("one")) =>
            Span::styled("one", Style::default().bg(Color::Blue).fg(Color::Green).add_modifier(Modifier::BOLD)),
        );
    }

    #[test]
    fn test_custom_tag_element() {
        let s = Style::default()
            .bg(Color::Blue)
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD);
        test_element!(
            |tag: &str| match tag {
                "keyboard" => Some(s),
                _ => None,
            } ; elem!("keyboard" ; pt!("W")) =>
            Span::styled("W", s),
        );
    }

    #[test]
    fn test_invalid_element() {
        test_fail!(elem!("qwerty" ; pt!("one")) => "qwerty", ErrorKind::InvalidTag);
    }
}
