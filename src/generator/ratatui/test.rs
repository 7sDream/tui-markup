use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

macro_rules! pt {
    ($text:literal) => {
        crate::parser::Item::PlainText($text.into())
    };
}

macro_rules! elem {
    (@tags, $($s:literal),+) => {{
        vec![$(crate::parser::LSpan::new_extra($s, 1)),+]
    }};
    ($($tags:tt),* ; $($items:expr),* $(,)?) => {
        crate::parser::Item::Element(elem!(@tags, $($tags),*), vec![$($items),*])
    };
}

macro_rules! test_ok {
    ($item:expr => $($result:expr),* $(,)?) => {
        let mut gen = crate::generator::RatatuiTextGenerator::default();
        let convertor =  <crate::generator::RatatuiTextGenerator as crate::generator::Generator>::convertor(&mut gen);
        let item = <<crate::generator::RatatuiTextGenerator as crate::generator::Generator>::Convertor as crate::generator::TagConvertor>::convert_item(convertor, $item);
        assert_eq!(
            crate::generator::helper::flatten::<_, ::ratatui::text::Span, _>(vec![item]),
            vec![$($result),*],
        )
    };
    ($custom:expr ; $item:expr => $($result:expr),* $(,)?) => {
        let mut gen = crate::generator::RatatuiTextGenerator::new($custom);
        let convertor =  <crate::generator::RatatuiTextGenerator<_> as crate::generator::Generator>::convertor(&mut gen);
        let item = <<crate::generator::RatatuiTextGenerator<_> as crate::generator::Generator>::Convertor as crate::generator::TagConvertor>::convert_item(convertor, $item);
        assert_eq!(
            crate::generator::helper::flatten::<_, ::ratatui::text::Span, _>(vec![item]),
            vec![$($result),*],
        )
    };
}

#[test]
fn test_normal_element() {
    test_ok!(elem!("green" ; pt!("xxx")) => Span::styled("xxx", Style::default().fg(Color::Green)));
    test_ok!(elem!("fg:red" ; pt!("xxx")) => Span::styled("xxx", Style::default().fg(Color::Red)));
    test_ok!(elem!("bg:yellow" ; pt!("xxx")) => Span::styled("xxx", Style::default().bg(Color::Yellow)));
    test_ok!(elem!("b" ; pt!("xxx")) => Span::styled("xxx", Style::default().add_modifier(Modifier::BOLD)));
    test_ok!(elem!("mod:i" ; pt!("xxx")) => Span::styled("xxx", Style::default().add_modifier(Modifier::ITALIC)));
}

#[test]
fn test_nested_element() {
    test_ok!(
        elem!("bg:blue" ; pt!("one "), elem!("green" ; pt!("two"))) =>
        Span::styled("one ", Style::default().bg(Color::Blue)),
        Span::styled("two", Style::default().bg(Color::Blue).fg(Color::Green)),
    );
}

#[test]
fn test_multi_tag_element() {
    test_ok!(
        elem!("bg:blue", "green", "b" ; pt!("one")) =>
        Span::styled("one", Style::default().bg(Color::Blue).fg(Color::Green).add_modifier(Modifier::BOLD)),
    );
}

#[test]
fn test_custom_tag_element() {
    let s = Style::default()
        .bg(Color::Blue)
        .fg(Color::Green)
        .add_modifier(Modifier::BOLD);
    test_ok!(
        |tag: &str| match tag {
            "keyboard" => Some(s),
            _ => None,
        } ; elem!("keyboard" ; pt!("W")) =>
        Span::styled("W", s),
    );
}

#[test]
fn test_invalid_element() {
    test_ok!(elem!("qwerty" ; pt!("one")) => Span::raw("one"));
}
