use tui::{
    style::Style,
    text::{Span, Spans},
};

use crate::tag::Tag;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Item<'a> {
    PlainText(&'a str),
    Element(Tag, Vec<Item<'a>>),
}

impl<'a> Item<'a> {
    fn plain_text_to_spans(escaped: &str, style: Option<Style>) -> Spans<'_> {
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

        Spans(spans)
    }

    pub fn into_spans(self, style: Option<Style>) -> Spans<'a> {
        match self {
            Item::PlainText(t) => Self::plain_text_to_spans(t, style),
            Item::Element(tag, children) => {
                let style = style.unwrap_or_default().patch(tag.0);
                children
                    .into_iter()
                    .flat_map(|part| part.into_spans(Some(style)).0)
                    .collect::<Vec<_>>()
                    .into()
            }
        }
    }
}

#[cfg(test)]
mod item_test {
    use tui::text::Span;

    use super::Item;

    macro_rules! test_plain_text {
        ($s:literal $(, $result:literal)*) => {
            assert_eq!(
                Item::PlainText($s).into_spans(None).0,
                vec![$(Span::raw($result),)*],
            );
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
}
