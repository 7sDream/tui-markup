//! Parsing stage of the compilation process.

pub use error::{Error, ErrorKind};
pub use item::{Item, ItemC, ItemG};
use winnow::{
    ModalResult, Parser,
    ascii::take_escaped,
    combinator::{alt, cut_err, delimited, eof, repeat, repeat_till, separated, terminated},
    error::ErrMode,
    stream::{ContainsToken, LocatingSlice, Stream},
    token::{one_of, take_till, take_while},
};

use crate::parser::ErrorKind::UnescapedChar;

mod error;
mod item;

#[cfg(test)]
mod test;

/// Span with location info.
pub type LSpan<'a> = LocatingSlice<&'a str>;

type ParseResult<'a, O = &'a str> = ModalResult<O, Error<'a>>;

fn alpha_numeric_set() -> impl ContainsToken<char> {
    ('a'..='z', 'A'..='Z', '0'..='9')
}

fn one_tag<'i>(i: &mut LSpan<'i>) -> ParseResult<'i> {
    take_while(1.., (alpha_numeric_set(), ':', '+', '-')).parse_next(i)
}

fn tag_list<'i>(i: &mut LSpan<'i>) -> ParseResult<'i, Vec<&'i str>> {
    separated(1.., one_tag, ',').parse_next(i)
}

fn element<'i>(i: &mut LSpan<'i>) -> ParseResult<'i, Item<'i>> {
    let start = *i;

    let tags = delimited('<', tag_list, ' ').parse_next(i)?;
    let parts = cut_err(terminated(items, ">".context(ErrorKind::ElementNotClose)))
        .parse_next(i)
        .map_err(|e: ErrMode<Error<'_>>| {
            e.map(|e| {
                if e.kind() == Some(ErrorKind::ElementNotClose) {
                    e.with_input(&start)
                } else {
                    e
                }
            })
        })?;

    Ok(Item::Element(tags, parts))
}

fn special_chars() -> impl ContainsToken<char> {
    ('<', '>', '\\')
}

fn plain_text_normal<'i>(i: &mut LSpan<'i>) -> ParseResult<'i> {
    take_till(1.., special_chars()).parse_next(i)
}

fn escapable_char<'i>(i: &mut LSpan<'i>) -> ParseResult<'i, char> {
    one_of(special_chars()).parse_next(i)
}

fn plain_text<'i>(i: &mut LSpan<'i>) -> ParseResult<'i> {
    let mut start = *i;

    take_escaped(plain_text_normal, '\\', escapable_char.context(ErrorKind::UnescapableChar)
    )
        // empty result is an error, for used in repeat
        .verify(|s: &str| !s.is_empty())
        .map_err(|e: ErrMode<Error<'_>>| {
            e.map(|mut e| {
                // Handle trailing backslash at end of line: `\` has nothing to escape.
                // This should be reported as UnescapedChar at the position of `\`.
                if e.kind() == Some(ErrorKind::UnescapableChar) && e.input.is_empty() {
                    start.next_slice(start.eof_offset() - 1);
                    e = e.with_input(&start).with_kind(ErrorKind::UnescapedChar);
                }

                // 1. When `take_escaped` try '\\' but it fails, means it's `<` or `>' 
                // it stops and return consumed as OK result.
                // 2. next loop of items will try parse it by `element`, if it fails, it call this again
                // and `plain_text_normal` reject it again
                // 3. it reenter step 1, but this time "consumed" is empty, which will be rejected by
                // `verify`, So we get a error without kind.
                // 4. So we use UnescapedChar for this case.
                if e.kind().is_none() {
                    e = e.with_kind(UnescapedChar)
                }

                e
            })
        })
        .parse_next(i)
}

fn item<'i>(i: &mut LSpan<'i>) -> ParseResult<'i, Item<'i>> {
    alt((element, plain_text.map(Item::PlainText))).parse_next(i)
}

fn items<'i>(i: &mut LSpan<'i>) -> ParseResult<'i, Vec<Item<'i>>> {
    repeat(0.., item).parse_next(i)
}

fn output<'i>(i: &mut LSpan<'i>) -> ParseResult<'i, Vec<Item<'i>>> {
    let (result, _) = repeat_till(0.., item, eof).parse_next(i)?;
    Ok(result)
}

fn parse_line(line: usize, i: &str) -> Result<Vec<Item<'_>>, Error<'_>> {
    let mut located = LSpan::new(i);

    output
        .map_err(|e| {
            e.into_inner()
                .expect("incomplete should never happens")
                .with_line(line)
        })
        .parse_next(&mut located)
}

/// Parse tui markup source into ast.
///
/// ## Errors
///
/// If input source has invalid syntax.
pub fn parse(s: &str) -> Result<Vec<Vec<Item<'_>>>, Error<'_>> {
    s.lines()
        .enumerate()
        .map(|(i, line)| parse_line(i, line))
        .collect()
}

fn hex_digit() -> impl ContainsToken<char> {
    ('0'..='9', 'A'..='F', 'a'..='f')
}

fn hex_color_part(i: &mut &str) -> ModalResult<u8> {
    repeat(2..=2, one_of(hex_digit()))
        .map(|_: ()| ())
        .take()
        .try_map(|x| u8::from_str_radix(x, 16))
        .parse_next(i)
}

/// Parse string of 6 hex digit into r, g, b value.
pub fn hex_rgb(s: &str) -> Option<(u8, u8, u8)> {
    let (r, g, b) = (hex_color_part, hex_color_part, hex_color_part)
        .parse(s)
        .ok()?;

    // After all three hex pairs are parsed, verify nothing remains.
    // Since hex_color_part takes 2 hex chars, the input advances by 6 chars.
    // If the input was exactly 6 hex chars, the remaining is empty.
    Some((r, g, b))
}
