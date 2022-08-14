use std::fmt::Display;

use crate::{error::LocatedError, parser::LSpan};

/// Kind of tui generator error
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    /// Contains invalid tag.
    InvalidTag,
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::InvalidTag => f.write_str("invalid tag "),
        }
    }
}

/// Error for [TuiTextGenerator][super::TuiTextGenerator].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error<'a> {
    kind: ErrorKind,
    pub(crate) span: LSpan<'a>,
}

impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <ErrorKind as Display>::fmt(&self.kind, f)?;

        f.write_fmt(format_args!(
            " \"{}\" near {}:{}",
            self.span,
            self.span.extra,
            self.span.get_column()
        ))
    }
}

impl<'a> From<Error<'a>> for crate::Error<'a, Error<'a>> {
    fn from(e: Error<'a>) -> Self {
        Self::Gen(e)
    }
}

impl<'a> Error<'a> {
    pub(crate) fn new(kind: ErrorKind, span: LSpan<'a>) -> Self {
        Self { kind, span }
    }

    /// Get error kind.
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl<'a> LocatedError for Error<'a> {
    fn location(&self) -> (usize, usize) {
        (self.span.extra, self.span.get_column())
    }
}
