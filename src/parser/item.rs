use crate::parser::LSpan;

use crate::generator::{Tag, TagG};

/// AST item.
///
/// In parsing step, each line of source code will be parsed to `Vec<Item>`, so the final result is `Vec<Vec<Item>>`.
///
/// In convert step, Each tag will be convert from [`LSpan`] into [Tag] type the generator needed,
/// by using [TagConvertor][crate::generator::TagConvertor] of that generator.
///
/// In generating step, generator will convert `Vec<Vec<Item<'_, Tag>>>>` to final output.
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
