use thiserror::Error;

use crate::parser::{Error as ParseError, LSpan};

/// Error with a location info.
pub trait LocatedError {
    /// get error happened location in source input.
    fn location(&self) -> (usize, usize);
}

/// Error type for [compile][super::compile] function.
///
/// Display this error in `{}` formatter will show a error message with detailed reason.
/// So usually you don't need check variants.
#[derive(Debug, PartialEq, Eq, Error)]
pub enum Error<'a, GE> {
    /// Parsing step failed, usually means there is invalid syntax in source string
    #[error("parse failed: {0}")]
    Parse(ParseError<'a>),

    /// Invalid or unsupported tag.
    #[error("unsupported tag: {0}")]
    Tag(LSpan<'a>),

    /// Generating step failed, see document of generator type for detail.
    #[error("generator failed: {0}")]
    Gen(GE),
}

impl<'a, GE: LocatedError> LocatedError for Error<'a, GE> {
    fn location(&self) -> (usize, usize) {
        match self {
            Self::Parse(e) => e.location(),
            Self::Tag(span) => (span.extra, span.get_column()),
            Self::Gen(e) => e.location(),
        }
    }
}

impl<'a, GE> From<ParseError<'a>> for Error<'a, GE> {
    fn from(e: ParseError<'a>) -> Self {
        Self::Parse(e)
    }
}
