use crate::parser::LSpan;

/// IR item.
///
/// In parsing step, each line of source code will be parsed to `Vec<Item>`, so the final result is `Vec<Vec<Item>>`.
#[derive(Debug, Clone, PartialEq)]
pub enum Item<'a> {
    /// Plain text(escaped) without any style.
    PlainText(&'a str),
    /// A styled element, contains a series tag name and subitems.
    Element(Vec<LSpan<'a>>, Vec<Item<'a>>),
}
