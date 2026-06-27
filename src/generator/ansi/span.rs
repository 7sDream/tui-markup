use std::fmt::Display;

use anstyle::{Color, Style};

use crate::generator::{
    Tag, TagConvertor,
    helper::{FlattenableSpan, FlattenableStyle},
};

/// A single styled text segment in the ANSI output.
///
/// Stores a style and the text it applies to. The text borrows from the original markup input
/// (zero-copy).
///
/// Display writes `{style}{text}{style:#}` — the ANSI escape sequence, then the text, then the
/// reset.
#[derive(Debug, Clone)]
pub struct StyledSpan<'a> {
    style: Style,
    text: &'a str,
}

impl<'a> StyledSpan<'a> {
    /// Create a new styled span.
    pub fn new(style: Style, text: &'a str) -> Self {
        Self { style, text }
    }

    /// Get the style applied to this span.
    pub fn style(&self) -> &Style {
        &self.style
    }

    /// Get the text content of this span.
    pub fn text(&self) -> &'a str {
        self.text
    }
}

impl Display for StyledSpan<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{:#}", self.style, self.text, self.style)
    }
}

/// A collection of [`StyledSpan`]s representing styled text ready for ANSI terminal output.
///
/// This is the output type of the
/// [`ANSIStringsGenerator`](crate::generator::ansi::ANSIStringsGenerator). It implements `Display`,
/// writing ANSI escape sequences for each styled span.
#[derive(Debug, Clone)]
pub struct StyledText<'a> {
    spans: Vec<StyledSpan<'a>>,
}

impl<'a> StyledText<'a> {
    /// Create a new styled text from spans.
    pub fn new(spans: Vec<StyledSpan<'a>>) -> Self {
        Self { spans }
    }

    /// Get a reference to the underlying spans.
    pub fn spans(&self) -> &[StyledSpan<'a>] {
        &self.spans
    }
}

impl Display for StyledText<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for span in &self.spans {
            Display::fmt(span, f)?;
        }
        Ok(())
    }
}

impl<'a> From<Vec<StyledSpan<'a>>> for StyledText<'a> {
    fn from(spans: Vec<StyledSpan<'a>>) -> Self {
        Self::new(spans)
    }
}

// --- From<Tag> impl for flatten ---

impl<'a, C> From<Tag<'a, C>> for Style
where
    C: TagConvertor<'a, Color = Color, Modifier = Style, Custom = Style>,
{
    fn from(t: Tag<'a, C>) -> Self {
        match t {
            Tag::Fg(c) => Self::new().fg_color(Some(c)),
            Tag::Bg(c) => Self::new().bg_color(Some(c)),
            Tag::Modifier(s) | Tag::Custom(s) => s,
        }
    }
}

// --- FlattenableStyle impl ---

impl FlattenableStyle for Style {
    /// `other` fg/bg override `self`; effects are additive (OR'd).
    fn patch(self, other: Self) -> Self {
        Self::new()
            .fg_color(other.get_fg_color().or(self.get_fg_color()))
            .bg_color(other.get_bg_color().or(self.get_bg_color()))
            .effects(self.get_effects() | other.get_effects())
    }
}

// --- FlattenableSpan impl ---

impl<'a> FlattenableSpan<'a, Style> for StyledSpan<'a> {
    fn with_style(s: &'a str, style: Option<Style>) -> Self {
        Self::new(style.unwrap_or_default(), s)
    }
}
