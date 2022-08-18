use crate::parser::LSpan;

use crate::generator::{Tag, TagG};

/// AST item.
///
/// In parsing stage, each line of source code will be parsed as a `Vec<Item>`, so the final result is `Vec<Vec<Item>>`.
///
/// Tag conversion stage, Each tag will be converted from [`LSpan`] into [Tag] type the generator needed,
/// by using [TagConvertor][crate::generator::TagConvertor] of the generator.
///
/// In generating stage, generator will convert `Vec<Vec<Item<'_, Tag>>>>` to final output.
#[derive(Debug, Clone, PartialEq)]
pub enum Item<'a, Tag = LSpan<'a>> {
    /// Plain text(escaped) without any style.
    PlainText(LSpan<'a>),
    /// A styled element, contains a series tag name and subitems.
    Element(Vec<Tag>, Vec<Item<'a, Tag>>),
}

/// Item type for tag convertor C.
pub type ItemC<'a, C> = Item<'a, Tag<'a, C>>;

/// Item type for generator G.
pub type ItemG<'a, G> = Item<'a, TagG<'a, G>>;
