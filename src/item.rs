use tui::{
    style::Style,
    text::{Span, Spans},
};

use crate::tag::Tag;

#[derive(Debug)]
pub(crate) enum Item<'a> {
    PlainText(&'a str),
    Element(Tag, Vec<Item<'a>>),
}

impl<'a> Item<'a> {
    pub fn into_spans(self, style: Option<Style>) -> Spans<'a> {
        match self {
            Item::PlainText(t) => t
                .split('\\')
                .map(|t| {
                    if let Some(style) = style {
                        Span::styled(t, style)
                    } else {
                        Span::raw(t)
                    }
                })
                .collect::<Vec<_>>()
                .into(),
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
