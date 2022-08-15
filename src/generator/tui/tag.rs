use nom::{bytes::complete::take_while_m_n, combinator::map_res, sequence::tuple, IResult};
use tui::style::{Color, Modifier, Style};

use crate::parser::LSpan;

use super::{error::ErrorKind, Error};

pub trait CustomTagProvider {
    fn provide(&mut self, s: &str) -> Option<Style>;
}

impl<F> CustomTagProvider for F
where
    F: FnMut(&str) -> Option<Style>,
{
    fn provide(&mut self, s: &str) -> Option<Style> {
        self(s)
    }
}

impl CustomTagProvider for () {
    fn provide(&mut self, _s: &str) -> Option<Style> {
        None
    }
}

fn try_custom_tag<F>(s: &str, f: &mut Option<F>) -> Option<Style>
where
    F: CustomTagProvider,
{
    f.as_mut().and_then(|f| f.provide(s))
}

// TODO: Move CustomTagProvider to helper

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tag {
    Fg(Color),
    Bg(Color),
    Modifier(Modifier),
    Extra(Style),
}

impl Tag {
    fn parse_hex_color_part(s: &str) -> IResult<&str, u8> {
        map_res(take_while_m_n(2, 2, |c: char| c.is_ascii_hexdigit()), |x| {
            u8::from_str_radix(x, 16)
        })(s)
    }

    fn parse_hex_color(s: &str) -> Option<Color> {
        let (s, (r, g, b)) = tuple((
            Self::parse_hex_color_part,
            Self::parse_hex_color_part,
            Self::parse_hex_color_part,
        ))(s)
        .ok()?;

        if !s.is_empty() {
            return None;
        }

        Some(Color::Rgb(r, g, b))
    }

    fn parse_color(s: &str) -> Option<Color> {
        Some(match s {
            "black" => Color::Black,
            "red" => Color::Red,
            "green" => Color::Green,
            "yellow" => Color::Yellow,
            "blue" => Color::Blue,
            "magenta" => Color::Magenta,
            "cyan" => Color::Cyan,
            "gray" => Color::Gray,
            "gray+" => Color::DarkGray,
            "red-" => Color::LightRed,
            "green-" => Color::LightGreen,
            "yellow-" => Color::LightYellow,
            "blue-" => Color::LightBlue,
            "magenta-" => Color::LightMagenta,
            "cyan-" => Color::LightCyan,
            "white" => Color::White,
            s => return Self::parse_hex_color(s),
        })
    }

    fn parse_modifier(s: &str) -> Option<Modifier> {
        Some(match s {
            "b" => Modifier::BOLD,
            "d" => Modifier::DIM,
            "i" => Modifier::ITALIC,
            "u" => Modifier::UNDERLINED,
            "s" => Modifier::SLOW_BLINK,
            "r" => Modifier::RAPID_BLINK,
            "h" => Modifier::HIDDEN,
            "x" => Modifier::CROSSED_OUT,
            _ => return None,
        })
    }

    pub fn parse_built_in(ty: &str, value: &str) -> Option<Self> {
        Some(match ty {
            "fg" => Self::Fg(Self::parse_color(value)?),
            "bg" => Self::Bg(Self::parse_color(value)?),
            "mod" => Self::Modifier(Self::parse_modifier(value)?),
            "" => {
                if let Some(color) = Self::parse_color(value) {
                    Self::Fg(color)
                } else if let Some(modifier) = Self::parse_modifier(value) {
                    Self::Modifier(modifier)
                } else {
                    return None;
                }
            }
            _ => return None,
        })
    }

    pub fn parse<P: CustomTagProvider>(s: &str, custom: &mut Option<P>) -> Option<Self> {
        let mut ty_value = s.split(':');
        let mut ty = ty_value.next()?;
        let value = ty_value.next().unwrap_or_else(|| {
            let value = ty;
            ty = "";
            value
        });

        if ty_value.next().is_some() {
            return try_custom_tag(s, custom).map(Self::Extra);
        }

        try_custom_tag(s, custom)
            .map(Self::Extra)
            .or_else(|| Self::parse_built_in(ty, value))
    }

    pub fn into_style(self) -> Style {
        match self {
            Tag::Fg(color) => Style::default().fg(color),
            Tag::Bg(color) => Style::default().bg(color),
            Tag::Modifier(m) => Style::default().add_modifier(m),
            Tag::Extra(style) => style,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tags(Style);

impl Tags {
    fn parse_one<'a, 'b, P: CustomTagProvider>(s: LSpan<'b>, custom: &'a mut Option<P>) -> Result<Style, Error<'b>> {
        Tag::parse(s.fragment(), custom)
            .map(Tag::into_style)
            .ok_or_else(|| Error::new(ErrorKind::InvalidTag, s))
    }

    pub fn parse<'a, 'b, P: CustomTagProvider>(
        s: Vec<LSpan<'b>>, custom: &'a mut Option<P>,
    ) -> Result<Self, Error<'b>> {
        let mut style = Style::default();

        for patch in s.into_iter().map(|t| Self::parse_one(t, custom)) {
            style = style.patch(patch?);
        }

        Ok(Self(style))
    }

    pub fn style(&self) -> Style {
        self.0
    }
}
