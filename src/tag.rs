use std::str::FromStr;

use nom::{bytes::complete::take_while_m_n, combinator::map_res, sequence::tuple, IResult};

use tui::style::{Color, Modifier, Style};

#[derive(Debug, Copy, Clone, PartialEq)]
enum OneStyle {
    Fg(Color),
    Bg(Color),
    Modifier(Modifier),
}

impl OneStyle {
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
}

impl FromStr for OneStyle {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ty_value = s.split(':');
        let mut ty = ty_value.next().ok_or(())?;
        let value = ty_value.next().unwrap_or_else(|| {
            let value = ty;
            ty = "";
            value
        });

        if ty_value.next().is_some() {
            return Err(());
        }

        Ok(match ty {
            "fg" => Self::Fg(Self::parse_color(value).ok_or(())?),
            "bg" => Self::Bg(Self::parse_color(value).ok_or(())?),
            "mod" => Self::Modifier(Self::parse_modifier(value).ok_or(())?),
            "" => {
                if let Some(color) = Self::parse_color(value) {
                    Self::Fg(color)
                } else if let Some(modifier) = Self::parse_modifier(value) {
                    Self::Modifier(modifier)
                } else {
                    return Err(());
                }
            }
            _ => return Err(()),
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) struct Tag(pub Style);

impl FromStr for Tag {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut style = Style::default();
        for patch in s.split(',').map(|s| s.parse::<OneStyle>()) {
            style = match patch? {
                OneStyle::Fg(color) => style.fg(color),
                OneStyle::Bg(color) => style.bg(color),
                OneStyle::Modifier(m) => style.add_modifier(m),
            }
        }

        Ok(Self(style))
    }
}
