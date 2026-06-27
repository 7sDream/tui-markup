use super::{ErrorKind, Item};

macro_rules! test_ok {
    ($s:expr $(, $item:expr)*) => {
        assert_eq!(crate::parser::parse_line(0, $s), Ok(vec![$($item,)*]));
    };
}

macro_rules! test_fail {
    ($s:expr, $column:expr, $kind:expr) => {
        let e = crate::parser::parse_line(0, $s).unwrap_err();
        println!("parse failed error: {:?}", e);
        println!("parse failed message: {}", e);
        assert_eq!(crate::error::LocatedError::location(&e), (1, $column));
        assert_eq!(e.kind().unwrap(), $kind);
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
    test_ok!("<green >", Item::Element(vec!["green"], vec![]));
}

#[test]
fn test_foreground_element() {
    test_ok!(
        "<fg:green text>",
        Item::Element(vec!["fg:green"], vec![Item::PlainText("text")])
    );
}

#[test]
fn test_foreground_element_without_mode() {
    test_ok!(
        "<blue text>",
        Item::Element(vec!["blue"], vec![Item::PlainText("text")])
    );
}

#[test]
fn test_foreground_element_with_only_colon() {
    test_ok!(
        "<:white text>",
        Item::Element(vec![":white"], vec![Item::PlainText("text")])
    );
}

#[test]
fn test_background_element() {
    test_ok!(
        "<bg:red text>",
        Item::Element(vec!["bg:red"], vec![Item::PlainText("text")])
    );
}

#[test]
fn test_modifier_element() {
    test_ok!(
        "<mod:b text>",
        Item::Element(vec!["mod:b"], vec![Item::PlainText("text")])
    );
}

#[test]
fn test_modifier_element_without_mode() {
    test_ok!(
        "<i text>",
        Item::Element(vec!["i"], vec![Item::PlainText("text")])
    );
}

#[test]
fn test_modifier_element_with_only_colon() {
    test_ok!(
        "<:d text>",
        Item::Element(vec![":d"], vec![Item::PlainText("text")])
    );
}

#[test]
fn test_nested_element() {
    test_ok!(
        "<bg:cyan <yellow one> two>",
        Item::Element(
            vec!["bg:cyan"],
            vec![
                Item::Element(vec!["yellow"], vec![Item::PlainText("one")]),
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
            vec!["bg:magenta", "gray", "mod:u", "x",],
            vec![Item::PlainText("text"),]
        )
    );
}

#[test]
fn test_custom_color() {
    test_ok!(
        "<bg:ff8000,66ccff text>",
        Item::Element(vec!["bg:ff8000", "66ccff"], vec![Item::PlainText("text")])
    );
}
