use std::fmt::{Display, Write};

use super::LSpan;
use crate::error::LocatedError;

/// Kind of parse error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    /// There is unescaped `<`, `>`, `\` character.
    UnescapedChar,
    /// There is a unescapable character after `\`.
    UnescapableChar,
    /// Element not closed but reaches line end.
    ElementNotClose,
}

/// Error type for [parse][super::parse].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error<'a> {
    nom_kind: nom::error::ErrorKind,
    pub(crate) span: LSpan<'a>,
    kind: Option<ErrorKind>,
}

impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self.kind {
            Some(kind) => match kind {
                ErrorKind::UnescapedChar => "unescaped character",
                ErrorKind::UnescapableChar => "unescapable character",
                ErrorKind::ElementNotClose => "expect '>' to close element for element starter",
            },
            None => "unknown error",
        })?;

        f.write_char(' ')?;
        if let Some(c) = self.span.chars().next() {
            f.write_char('\'')?;
            f.write_char(c)?;
            f.write_char('\'')?;
            f.write_char(' ')?;
        }

        f.write_fmt(format_args!("near {}:{}", self.span.extra, self.span.get_column()))
    }
}

impl<'a> std::error::Error for Error<'a> {}

impl<'a> Error<'a> {
    pub(crate) fn attach(mut self, kind: ErrorKind) -> Self {
        if self.kind.is_none() {
            self.kind = Some(kind);
        }
        self
    }

    /// Get error kind.
    #[must_use]
    pub fn kind(&self) -> Option<ErrorKind> {
        self.kind
    }
}

impl<'a> LocatedError for Error<'a> {
    fn location(&self) -> (usize, usize) {
        (self.span.extra, self.span.get_column())
    }
}

impl<'a> nom::error::ParseError<LSpan<'a>> for Error<'a> {
    fn from_error_kind(input: LSpan<'a>, kind: nom::error::ErrorKind) -> Self {
        Self {
            nom_kind: kind,
            span: input,
            kind: None,
        }
    }

    fn append(_input: LSpan<'a>, _kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}
