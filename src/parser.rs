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
    let located = LSpan::new(s);

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

#[cfg(test)]
mod parser_test {
    use nom::InputTake;
    use tui::style::{Color, Modifier, Style};

    use crate::{item::Item, tag::Tag};

    macro_rules! test_ok {
        ($s:literal $(, $item:expr)*) => {
            let source = $s;
            let s = crate::parser::LSpan::new(source);
            let (remainder, _) = s.take_split(source.len());

            assert_eq!(crate::parser::items(s), Ok((remainder, vec![$($item,)*])));
        };
    }

    macro_rules! test_fail {
        ($s:literal, $at:literal) => {
            assert_eq!(crate::parser::parse($s), Err((&$s[$at - 1..], $at)));
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
        test_ok!(
            "<green >",
            Item::Element(Tag(Style::default().fg(Color::Green)), vec![])
        );
    }

    #[test]
    fn test_foreground_element() {
        test_ok!(
            "<fg:green text>",
            Item::Element(Tag(Style::default().fg(Color::Green)), vec![Item::PlainText("text")])
        );
    }

    #[test]
    fn test_foreground_element_without_mode() {
        test_ok!(
            "<blue text>",
            Item::Element(Tag(Style::default().fg(Color::Blue)), vec![Item::PlainText("text")])
        );
    }

    #[test]
    fn test_background_element() {
        test_ok!(
            "<bg:red text>",
            Item::Element(Tag(Style::default().bg(Color::Red)), vec![Item::PlainText("text")])
        );
    }

    #[test]
    fn test_modifier_element() {
        test_ok!(
            "<mod:b text>",
            Item::Element(
                Tag(Style::default().add_modifier(Modifier::BOLD)),
                vec![Item::PlainText("text")]
            )
        );
    }

    #[test]
    fn test_modifier_element_without_mode() {
        test_ok!(
            "<i text>",
            Item::Element(
                Tag(Style::default().add_modifier(Modifier::ITALIC)),
                vec![Item::PlainText("text")]
            )
        );
    }

    #[test]
    fn test_nested_element() {
        test_ok!(
            "<bg:cyan <yellow one> two>",
            Item::Element(
                Tag(Style::default().bg(Color::Cyan)),
                vec![
                    Item::Element(Tag(Style::default().fg(Color::Yellow)), vec![Item::PlainText("one")]),
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
                Tag(Style::default()
                    .bg(Color::Magenta)
                    .fg(Color::Gray)
                    .add_modifier(Modifier::UNDERLINED | Modifier::CROSSED_OUT)),
                vec![Item::PlainText("text"),]
            )
        );
    }

    #[test]
    fn test_custom_color() {
        test_ok!(
            "<bg:ff8000,66ccff text>",
            Item::Element(
                Tag(Style::default()
                    .bg(Color::Rgb(0xff, 0x80, 0x00))
                    .fg(Color::Rgb(0x66, 0xcc, 0xff))),
                vec![Item::PlainText("text")]
            )
        );
    }
}
