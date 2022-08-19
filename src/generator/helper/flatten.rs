use crate::{
    generator::{Tag, TagConvertor},
    parser::{Item, ItemC},
};

use super::unescape;

/// Requirements trait for style to used in [`flatten`] function.
pub trait FlattenableStyle: Default + Clone {
    /// Patch self with other style, so multi style can be flatten into one.
    #[must_use]
    fn patch(self, other: Self) -> Self;
}

/// Requirements trait for span to used in [`flatten`] function.
pub trait FlattenableSpan<'a, S: FlattenableStyle> {
    /// Create a span from str and a optional style.
    ///
    /// In flatten process, we will store each plaintext with current state of style.
    fn with_style(s: &'a str, style: Option<S>) -> Self;
}

#[allow(clippy::needless_pass_by_value)] // Style usually is a small type
fn plain_text<'a, R, S>(escaped: &'a str, style: Option<S>) -> Vec<R>
where
    R: FlattenableSpan<'a, S>,
    S: FlattenableStyle,
{
    unescape(escaped).map(|s| R::with_style(s, style.clone())).collect()
}

fn element<'a, C, R, S>(tags: Vec<Tag<'a, C>>, children: Vec<ItemC<'a, C>>, style: Option<S>) -> Vec<R>
where
    C: TagConvertor<'a>,
    R: FlattenableSpan<'a, S>,
    S: FlattenableStyle + From<Tag<'a, C>>,
{
    let style = tags.into_iter().map(S::from).fold(style.unwrap_or_default(), S::patch);
    items(children, Some(style))
}

fn item<'a, C, R, S>(item: ItemC<'a, C>, style: Option<S>) -> Vec<R>
where
    C: TagConvertor<'a>,
    R: FlattenableSpan<'a, S>,
    S: FlattenableStyle + From<Tag<'a, C>>,
{
    match item {
        Item::PlainText(t) => plain_text(t.fragment(), style),
        Item::Element(tags, children) => element(tags, children, style),
    }
}

#[allow(clippy::needless_pass_by_value)] // Style usually is a small type
fn items<'a, C, R, S>(items: Vec<ItemC<'a, C>>, style: Option<S>) -> Vec<R>
where
    C: TagConvertor<'a>,
    R: FlattenableSpan<'a, S>,
    S: FlattenableStyle + From<Tag<'a, C>>,
{
    items
        .into_iter()
        .flat_map(|x| item(x, style.clone()).into_iter())
        .collect()
}

/// Flatten a line of ast tree item into a vector of target spans.
///
/// ## Why need this
///
/// Each line of markup source will be convert into a `Vec<Item>` after parsing and tag conversion stage.
///
/// But [Item] itself is a tree, so there are many tree to process.
///
/// With this function, We can flatten each line into a flat `Vec<Span>`.
///
/// ## How can I use this?
///
/// If these requirements are met:
///
/// - [Tag] of a convertor `C` can be converted into a uniform `Style` struct, by impl the `From<Tag<C>>` trait.
/// - the `Style` type can impl [`FlattenableStyle`] trait.
/// - You have a struct `Span` can impl [`FlattenableSpan`] trait to store styled text span.
///
/// Then this function can be used to convert `Vec<ItemC<C>>` into `Vec<Span>`.
pub fn flatten<'a, C, R, S>(line: Vec<ItemC<'a, C>>) -> Vec<R>
where
    C: TagConvertor<'a>,
    R: FlattenableSpan<'a, S>,
    S: FlattenableStyle + From<Tag<'a, C>>,
{
    items(line, None)
}
