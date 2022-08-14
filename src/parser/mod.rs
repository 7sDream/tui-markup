//! Parsing step of compile.

mod error;
mod item;

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

#[cfg(test)]
mod parser_test {
    use crate::parser::{ErrorKind, Item};

    macro_rules! test_ok {
        ($s:expr $(, $item:expr)*) => {
            assert_eq!(crate::parser::parse_line((0, $s)), Ok((vec![$($item,)*])));
        };
    }

    macro_rules! test_fail {
        ($s:expr, $column:expr, $kind:expr) => {
            let e = crate::parser::parse_line((0, $s)).unwrap_err();
            println!("parse failed error: {:?}", e);
            println!("parse failed message: {}", e);
            assert_eq!(crate::error::LocatedError::location(&e), (1, $column));
            assert_eq!(e.kind().unwrap(), $kind);
        };
    }

    macro_rules! lspan {
        ($s:expr, $offset:expr) => {
            unsafe { crate::parser::LSpan::new_from_raw_offset($offset, 1, $s, 1) }
        };
        ($s:expr) => {
            unsafe { crate::parser::LSpan::new_from_raw_offset(0, 1, $s, 1) }
        };
    }

    #[test]
    fn test_escaped_char() {
        test_ok!("\\<", Item::PlainText("\\<"));
        test_ok!("\\>", Item::PlainText("\\>"));
        test_ok!("\\\\", Item::PlainText("\\\\"));
    }

    #[test]
    fn test_unescaped_string() {
        test_fail!("<456", 1, ErrorKind::UnescapedChar);
        test_fail!(">456", 1, ErrorKind::UnescapedChar);

        test_fail!("123<456", 4, ErrorKind::UnescapedChar);
        test_fail!("123>456", 4, ErrorKind::UnescapedChar);

        test_fail!("123<", 4, ErrorKind::UnescapedChar);
        test_fail!("123>", 4, ErrorKind::UnescapedChar);
        test_fail!("123\\", 4, ErrorKind::UnescapedChar);

        test_fail!("\\", 1, ErrorKind::UnescapedChar);
    }

    #[test]
    fn test_unescapable_string() {
        test_fail!("\\456", 2, ErrorKind::UnescapableChar);
        test_fail!("123\\456", 5, ErrorKind::UnescapableChar);
    }

    #[test]
    fn test_no_space_element() {
        test_fail!("<green>", 1, ErrorKind::UnescapedChar);
    }

    #[test]
    fn test_unclosed_element() {
        test_fail!("<b ", 1, ErrorKind::ElementNotClose);
        test_fail!("<b aaa", 1, ErrorKind::ElementNotClose);
        test_fail!("123<b aaa", 4, ErrorKind::ElementNotClose);
    }

    #[test]
    fn test_empty_input() {
        test_ok!("");
    }

    #[test]
    fn test_no_content_element() {
        test_ok!("<green >", Item::Element(vec![lspan!("green", 1)], vec![]));
    }

    #[test]
    fn test_foreground_element() {
        test_ok!(
            "<fg:green text>",
            Item::Element(vec![lspan!("fg:green", 1)], vec![Item::PlainText("text")])
        );
    }

    #[test]
    fn test_foreground_element_without_mode() {
        test_ok!(
            "<blue text>",
            Item::Element(vec![lspan!("blue", 1)], vec![Item::PlainText("text")])
        );
    }

    #[test]
    fn test_foreground_element_with_only_colon() {
        test_ok!(
            "<:white text>",
            Item::Element(vec![lspan!(":white", 1)], vec![Item::PlainText("text")])
        );
    }

    #[test]
    fn test_background_element() {
        test_ok!(
            "<bg:red text>",
            Item::Element(vec![lspan!("bg:red", 1)], vec![Item::PlainText("text")])
        );
    }

    #[test]
    fn test_modifier_element() {
        test_ok!(
            "<mod:b text>",
            Item::Element(vec![lspan!("mod:b", 1)], vec![Item::PlainText("text")])
        );
    }

    #[test]
    fn test_modifier_element_without_mode() {
        test_ok!(
            "<i text>",
            Item::Element(vec![lspan!("i", 1)], vec![Item::PlainText("text")])
        );
    }

    #[test]
    fn test_modifier_element_with_only_colon() {
        test_ok!(
            "<:d text>",
            Item::Element(vec![lspan!(":d", 1)], vec![Item::PlainText("text")])
        );
    }

    #[test]
    fn test_nested_element() {
        test_ok!(
            "<bg:cyan <yellow one> two>",
            Item::Element(
                vec![lspan!("bg:cyan", 1)],
                vec![
                    Item::Element(vec![lspan!("yellow", 10)], vec![Item::PlainText("one")]),
                    Item::PlainText(" two"),
                ]
            )
        );
    }

    #[test]
    fn test_multi_style_element() {
        test_ok!(
            "<bg:magenta,gray,mod:u,x text>",
            Item::Element(
                vec![
                    lspan!("bg:magenta", 1),
                    lspan!("gray", 12),
                    lspan!("mod:u", 17),
                    lspan!("x", 23)
                ],
                vec![Item::PlainText("text"),]
            )
        );
    }

    #[test]
    fn test_custom_color() {
        test_ok!(
            "<bg:ff8000,66ccff text>",
            Item::Element(
                vec![lspan!("bg:ff8000", 1), lspan!("66ccff", 11)],
                vec![Item::PlainText("text")]
            )
        );
    }
}
