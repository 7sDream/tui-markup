//! Parsing step of compile.

mod error;
mod item;
#[cfg(test)]
mod test;

use nom::{
    branch::alt,
    bytes::complete::{escaped, is_not, tag, take_while1},
    character::complete::{char, one_of},
    combinator::{eof, map, verify},
    multi::{many0, many_till, separated_list1},
    sequence::tuple,
    Err as NomErr, IResult,
};
use nom_locate::LocatedSpan;

pub use error::{Error, ErrorKind};
pub use item::Item;

/// String with location info.
pub type LSpan<'a> = LocatedSpan<&'a str, usize>;

type ParseResult<'a, O = LSpan<'a>> = IResult<LSpan<'a>, O, Error<'a>>;

fn force_failure<E>(err: NomErr<E>) -> NomErr<E> {
    match err {
        e @ NomErr::Incomplete(_) | e @ NomErr::Failure(_) => e,
        NomErr::Error(e) => NomErr::Failure(e),
    }
}

fn one_tag(s: LSpan) -> ParseResult {
    take_while1(|c: char| c.is_alphanumeric() || c == ':' || c == '+' || c == '-')(s)
}

fn tag_list(s: LSpan) -> ParseResult<Vec<LSpan>> {
    separated_list1(char(','), one_tag)(s)
}

fn element_inner_space(s: LSpan) -> ParseResult<char> {
    char(' ')(s)
}

fn element_end(s: LSpan) -> ParseResult {
    tag(">")(s).map_err(|e: NomErr<Error>| e.map(|e| e.attach(ErrorKind::ElementNotClose)))
}

fn element(s: LSpan) -> ParseResult<Item> {
    let input = s;

    let (s, (_, tags, _)) = tuple((char('<'), tag_list, element_inner_space))(s)?;
    let (s, (parts, _)) = tuple((items, element_end))(s)
        .map_err(|e: NomErr<Error>| {
            e.map(|mut e| {
                if e.kind() == Some(ErrorKind::ElementNotClose) {
                    e.span = input;
                }
                e
            })
        })
        .map_err(force_failure)?;

    Ok((s, Item::Element(tags, parts)))
}

fn plain_text_normal(s: LSpan) -> ParseResult {
    is_not("<>\\")(s)
}

fn escapable_char(s: LSpan) -> ParseResult<char> {
    one_of("<>\\")(s).map_err(|e: NomErr<Error>| e.map(|e| e.attach(ErrorKind::UnescapableChar)))
}

fn plain_text(s: LSpan) -> ParseResult<Item> {
    map(
        verify(escaped(plain_text_normal, '\\', escapable_char), |ls| !ls.is_empty()),
        |ls: LSpan| Item::PlainText(ls.fragment()),
    )(s)
    .map_err(|e| {
        e.map(|mut e| {
            use nom::Slice;

            if !s.is_empty()
                && e.kind().map(|x| x == ErrorKind::UnescapedChar).unwrap_or(true)
                && s.trim_start_matches(['\\', '<', '>']) == *s.fragment()
            {
                e.span = e.span.slice(e.span.len() - 1..);
            }

            e.attach(ErrorKind::UnescapedChar)
        })
    })
}

fn item(s: LSpan) -> ParseResult<Item> {
    alt((element, plain_text))(s)
}

fn items(s: LSpan) -> ParseResult<Vec<Item>> {
    many0(item)(s)
}

fn output(s: LSpan) -> ParseResult<Vec<Item>> {
    let (s, result) = many_till(item, eof)(s)?;
    Ok((s, result.0))
}

fn parse_line((line, s): (usize, &str)) -> Result<Vec<Item>, Error> {
    let located = LSpan::new_extra(s, line + 1);

    match output(located) {
        Ok((_, result)) => Ok(result),
        Err(NomErr::Error(e) | NomErr::Failure(e)) => Err(e),
        Err(NomErr::Incomplete(_)) => unreachable!("Parser is not streaming, so incomplete state shouldn't appear"),
    }
}

/// Parse markup source into IR.
pub fn parse(s: &str) -> Result<Vec<Vec<Item>>, Error> {
    s.lines().enumerate().map(parse_line).collect()
}
