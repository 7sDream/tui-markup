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
mod test {
    use nom::InputTake;
    use tui::style::{Color, Style};

    use crate::{item::Item, tag::Tag};

    use super::{items, parse, LSpan};

    #[test]
    fn test_escaped_string() {
        let s = LSpan::new("\\<");

        assert_eq!(items(s).unwrap().1, vec![Item::PlainText("\\<")]);

        let s = LSpan::new("\\>");

        assert_eq!(items(s).unwrap().1, vec![Item::PlainText("\\>")]);

        let s = LSpan::new("\\\\");

        assert_eq!(items(s).unwrap().1, vec![Item::PlainText("\\\\")]);
    }

    #[test]
    fn test_invalid_escaped_string() {
        assert!(parse("\\x").is_err());
    }

    #[test]
    fn test_ok_with_empty_input() {
        let s = LSpan::new("");

        assert_eq!(items(s), Ok((s, vec![])));
    }

    #[test]
    fn test_error_with_no_space_element() {
        assert!(parse("<green>").is_err());
    }

    #[test]
    fn test_ok_with_no_content_element() {
        let source = "<green >";
        let s = LSpan::new(source);
        let (remainder, _) = s.take_split(8);

        assert_eq!(
            items(s),
            Ok((
                remainder,
                vec![Item::Element(Tag(Style::default().fg(Color::Green)), vec![])]
            )),
        );
    }
}
