//! Parsing stage of the compilation process.

pub use error::{Error, ErrorKind};
pub use item::{Item, ItemC, ItemG};
use nom::{
    Err as NomErr, IResult, Parser,
    branch::alt,
    bytes::complete::{escaped, is_not, tag, take_while_m_n, take_while1},
    character::complete::{char, one_of},
    combinator::{eof, map, map_res, verify},

    multi::{many_till, many0, separated_list1},
};
use nom_locate::LocatedSpan;

mod error;
mod item;

#[cfg(test)]
mod test;

/// Span with location info.
pub type LSpan<'a> = LocatedSpan<&'a str, usize>;

type ParseResult<'a, O = LSpan<'a>> = IResult<LSpan<'a>, O, Error<'a>>;

fn force_failure<E>(err: NomErr<E>) -> NomErr<E> {
    match err {
        e @ (NomErr::Incomplete(_) | NomErr::Failure(_)) => e,
        NomErr::Error(e) => NomErr::Failure(e),
    }
}

fn one_tag(s: LSpan<'_>) -> ParseResult<'_> {
    take_while1(|c: char| c.is_alphanumeric() || c == ':' || c == '+' || c == '-').parse(s)
}

fn tag_list(s: LSpan<'_>) -> ParseResult<'_, Vec<LSpan<'_>>> {
    separated_list1(char(','), one_tag).parse(s)
}

fn element_inner_space(s: LSpan<'_>) -> ParseResult<'_, char> {
    char(' ').parse(s)
}

fn element_end(s: LSpan<'_>) -> ParseResult<'_> {
    tag(">").parse(s).map_err(|e: NomErr<Error<'_>>| e.map(|e| e.attach(ErrorKind::ElementNotClose)))
}

fn element(s: LSpan<'_>) -> ParseResult<'_, Item<'_>> {
    let input = s;

    let (s, (_, tags, _)) = (char('<'), tag_list, element_inner_space).parse(s)?;
    let (s, (parts, _)) = (items, element_end).parse(s)
        .map_err(|e: NomErr<Error<'_>>| {
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

fn plain_text_normal(s: LSpan<'_>) -> ParseResult<'_> {
    is_not("<>\\").parse(s)
}

fn escapable_char(s: LSpan<'_>) -> ParseResult<'_, char> {
    one_of("<>\\").parse(s)
        .map_err(|e: NomErr<Error<'_>>| e.map(|e| e.attach(ErrorKind::UnescapableChar)))
}

fn plain_text(s: LSpan<'_>) -> ParseResult<'_> {
    verify(escaped(plain_text_normal, '\\', escapable_char), |ls| {
        !ls.is_empty()
    }).parse(s)
    .map_err(|e| {
        e.map(|mut e| {
            use nom::Input;

            if !s.is_empty()
                && e.kind().is_none_or(|x| x == ErrorKind::UnescapedChar)
                && !s.starts_with(['\\', '<', '>'])
            {
                e.span = e.span.take_from(e.span.len() - 1);
            }

            e.attach(ErrorKind::UnescapedChar)
        })
    })
}

fn item(s: LSpan<'_>) -> ParseResult<'_, Item<'_>> {
    alt((element, map(plain_text, Item::PlainText))).parse(s)
}

fn items(s: LSpan<'_>) -> ParseResult<'_, Vec<Item<'_>>> {
    many0(item).parse(s)
}

fn output(s: LSpan<'_>) -> ParseResult<'_, Vec<Item<'_>>> {
    let (s, result) = many_till(item, eof).parse(s)?;
    Ok((s, result.0))
}

fn parse_line((line, s): (usize, &str)) -> Result<Vec<Item<'_>>, Error<'_>> {
    let located = LSpan::new_extra(s, line + 1);

    match output(located) {
        Ok((_, result)) => Ok(result),
        Err(NomErr::Error(e) | NomErr::Failure(e)) => Err(e),
        Err(NomErr::Incomplete(_)) => {
            unreachable!("Parser is not streaming, so incomplete state shouldn't appear")
        }
    }
}

/// Parse tui markup source into ast.
///
/// ## Errors
///
/// If input source has invalid syntax.
pub fn parse(s: &str) -> Result<Vec<Vec<Item<'_>>>, Error<'_>> {
    s.lines()
        .enumerate()
        .map(parse_line)
        .collect::<Result<Vec<Vec<Item<'_>>>, Error<'_>>>()
}

fn hex_color_part(s: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, |c: char| c.is_ascii_hexdigit()), |x| {
        u8::from_str_radix(x, 16)
    }).parse(s)
}

/// Parse string of 6 hex digit into r, g, b value.
#[must_use]
pub fn hex_rgb(s: &str) -> Option<(u8, u8, u8)> {
    let (s, (r, g, b)) = (hex_color_part, hex_color_part, hex_color_part).parse(s).ok()?;

    if !s.is_empty() {
        return None;
    }

    Some((r, g, b))
}
