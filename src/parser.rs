use nom::{
    branch::alt,
    bytes::complete::{escaped, is_not, tag, take_while1},
    character::complete::{char, one_of},
    combinator::{map, verify},
    multi::{many0, separated_list1},
    sequence::{preceded, tuple},
    IResult,
};
use nom_locate::LocatedSpan;
use tui::{style::Style, text::Span};

use crate::{item::Item, Error};

pub type LSpan<'a> = LocatedSpan<&'a str, usize>;

fn one_tag(s: LSpan) -> IResult<LSpan, LSpan> {
    take_while1(|c: char| c.is_alphanumeric() || c == ':' || c == '+' || c == '-')(s)
}

fn tag_list(s: LSpan) -> IResult<LSpan, Vec<LSpan>> {
    separated_list1(char(','), one_tag)(s)
}

fn element_start(s: LSpan) -> IResult<LSpan, Vec<LSpan>> {
    let (s, tag) = preceded(char('<'), tag_list)(s)?;
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

fn plain_text(s: LSpan) -> IResult<LSpan, Item> {
    map(
        verify(escaped(is_not("<>\\"), '\\', one_of("<>\\")), |t: &LSpan| !t.is_empty()),
        |ls: LSpan| Item::PlainText(ls.fragment()),
    )(s)
}

fn item(s: LSpan) -> IResult<LSpan, Item> {
    alt((plain_text, element))(s)
}

fn items(s: LSpan) -> IResult<LSpan, Vec<Item>> {
    many0(item)(s)
}

pub fn parse<F>(s: &str, line: usize, mut extra: F) -> Result<Vec<Span<'_>>, Error>
where
    F: FnMut(&str) -> Option<Style>,
{
    let located = LSpan::new_extra(s, line);

    let (remain, items) = items(located).unwrap();
    if !remain.fragment().is_empty() {
        let first_char_len = remain.chars().next().unwrap().len_utf8();
        return Err(Error::InvalidSyntax(
            &remain[0..first_char_len],
            line,
            remain.get_column(),
        ));
    }

    let mut result = vec![];
    for spans in items.into_iter().map(|item| item.into_spans(&mut extra, None)) {
        result.extend(spans?)
    }

    Ok(result)
}

#[cfg(test)]
mod parser_test {

    use crate::item::Item;

    macro_rules! test_ok {
        ($s:expr $(, $item:expr)*) => {
            let source = $s;
            let s = crate::parser::LSpan::new_extra(source, 1);
            let (remainder, _) = ::nom::InputTake::take_split(&s, source.len());

            assert_eq!(crate::parser::items(s), Ok((remainder, vec![$($item,)*])));
        };
    }

    macro_rules! test_fail {
        ($s:expr, $at:expr) => {
            assert_eq!(
                crate::parser::parse($s, 1, |_| None).unwrap_err().position().1,
                $at
            );
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
    fn test_escaped_string() {
        test_ok!("\\<", Item::PlainText("\\<"));
        test_ok!("\\>", Item::PlainText("\\>"));
        test_ok!("\\\\", Item::PlainText("\\\\"));
    }

    #[test]
    fn test_invalid_escaped_string() {
        test_fail!("\\x", 1);
    }

    #[test]
    fn test_empty_input() {
        test_ok!("");
    }

    #[test]
    fn test_no_space_element() {
        test_fail!("<green>", 1);
    }

    #[test]
    fn test_no_content_element() {
        test_ok!("<green >", Item::Element(vec![lspan!("<green >")], vec![]));
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
