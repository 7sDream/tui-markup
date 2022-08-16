use crate::{
    generator::{GenericSpan, GenericStyle, Tag, TagConvertor},
    parser::{Item, ItemC},
};

use super::unescape;

fn plain_text<'a, R, S>(escaped: &'a str, style: Option<S>) -> Vec<R>
where
    R: GenericSpan<'a, S>,
    S: GenericStyle,
{
    unescape(escaped).map(|s| R::with_style(s, style.clone())).collect()
}

fn element<'a, C, R, S>(tags: Vec<Tag<'a, C>>, children: Vec<ItemC<'a, C>>, style: Option<S>) -> Vec<R>
where
    C: TagConvertor<'a>,
    R: GenericSpan<'a, S>,
    S: GenericStyle,
    S: From<Tag<'a, C>>,
{
    let style = tags.into_iter().map(S::from).fold(style.unwrap_or_default(), S::patch);
    items(children, Some(style))
}

fn item<'a, C, R, S>(item: ItemC<'a, C>, style: Option<S>) -> Vec<R>
where
    C: TagConvertor<'a>,
    R: GenericSpan<'a, S>,
    S: GenericStyle,
    S: From<Tag<'a, C>>,
{
    match item {
        Item::PlainText(t) => plain_text(t.fragment(), style),
        Item::Element(tags, children) => element(tags, children, style),
    }
}

fn items<'a, C, R, S>(items: Vec<ItemC<'a, C>>, style: Option<S>) -> Vec<R>
where
    C: TagConvertor<'a>,
    R: GenericSpan<'a, S>,
    S: GenericStyle,
    S: From<Tag<'a, C>>,
{
    items
        .into_iter()
        .flat_map(|x| item(x, style.clone()).into_iter())
        .collect()
}

/// Flatten
///
/// TODO: Doc
pub fn flatten<'a, C, R, S>(line: Vec<ItemC<'a, C>>) -> Vec<R>
where
    C: TagConvertor<'a>,
    R: GenericSpan<'a, S>,
    S: GenericStyle,
    S: From<Tag<'a, C>>,
{
    items(line, None)
}
