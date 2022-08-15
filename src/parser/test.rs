use super::{ErrorKind, Item};

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
