use nom::{
    branch::alt,
    bytes::complete::{escaped, is_not, tag, take_while1},
    character::complete::{char, one_of},
    combinator::{map, map_res, verify},
    multi::many0,
    sequence::{preceded, tuple},
    IResult,
};
use nom_locate::LocatedSpan;
use tui::text::Spans;

use crate::{item::Item, tag::Tag};

type LSpan<'a> = LocatedSpan<&'a str>;

fn tag_name(s: LSpan) -> IResult<LSpan, LSpan> {
    take_while1(|c: char| c.is_alphanumeric() || c == ':' || c == '+' || c == '-' || c == ',')(s)
}

fn element_start(s: LSpan) -> IResult<LSpan, Tag> {
    let (s, tag) = preceded(char('<'), map_res(tag_name, |s| s.parse()))(s)?;
    let (s, _) = char(' ')(s)?;
    Ok((s, tag))
}

fn element_end(s: LSpan) -> IResult<LSpan, LSpan> {
    tag(">")(s)
}

fn element(s: LSpan) -> IResult<LSpan, Item> {
    let (s, (tag, parts, _)) = tuple((element_start, items, element_end))(s)?;
    Ok((s, Item::Element(tag, parts)))
}

fn plain_text(s: LSpan) -> IResult<LSpan, LSpan> {
    escaped(is_not("<>\\"), '\\', one_of("<>\\"))(s)
}

fn item(s: LSpan) -> IResult<LSpan, Item> {
    alt((map(plain_text, |s| Item::PlainText(s.fragment())), element))(s)
}

fn items(s: LSpan) -> IResult<LSpan, Vec<Item>> {
    many0(verify(item, |p| !matches!(p, Item::PlainText(""))))(s)
}

pub fn parse(s: &str) -> Result<Spans, (&str, usize)> {
    // if s.is_empty() {
    //     return Ok(Spans::default());
    // }

    let located = nom_locate::LocatedSpan::from(s);

    let (remain, items) = items(located).unwrap();
    if !remain.fragment().is_empty() {
        return Err((remain.fragment(), remain.get_column()));
    }

    let x = items
        .into_iter()
        .flat_map(|item| item.into_spans(None).0)
        .collect::<Vec<_>>();

    Ok(x.into())
}
